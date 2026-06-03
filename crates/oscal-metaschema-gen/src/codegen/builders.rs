//! Generates `{Name}Builder` structs and `BuildError` for every assembly.
//!
//! ## Builder contract
//!
//! - Required fields (`Cardinality::Required`) → held as `Option<T>` in the
//!   builder; `build()` returns `Err(BuildError::MissingField)` if unset.
//! - Optional fields (`Cardinality::Optional`) → held as `Option<T>`; pass
//!   through to the struct field as-is.
//! - Collection fields (`ZeroOrMore` / `OneOrMore`) → held as `Vec<T>`; a
//!   singular setter pushes one element.
//!
//! Each setter takes `impl Into<T>` so callers can pass string literals etc.
//! The struct gains a `fn builder() -> {Name}Builder` convenience constructor.

use heck::{ToSnakeCase, ToUpperCamelCase as _};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::{
    codegen::{
        serde_attrs::to_field_ident,
        structs::{data_type_tokens, sanitize_type_name},
    },
    ir::types::{AssemblyDef, Cardinality, IrDb, ModelItem},
};

// ── Public entry point ────────────────────────────────────────────────────────

/// Generate a `{Name}Builder` and related `BuildError` for `def`.
///
/// # Errors
///
/// Currently infallible; returns `Ok` always.
pub fn gen_builder(def: &AssemblyDef, db: &IrDb) -> TokenStream {
    let struct_name = format_ident!("{}", sanitize_type_name(&def.name.to_upper_camel_case()));
    let builder_name = format_ident!(
        "{}Builder",
        sanitize_type_name(&def.name.to_upper_camel_case())
    );

    let fields = collect_builder_fields(def, db);

    let builder_field_decls: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.builder_ty;
            quote! { #ident: #ty, }
        })
        .collect();

    // Default values for the builder's new() — cannot use #[derive(Default)]
    // because some inner types (e.g. url::Url) don't implement Default.
    let new_inits: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let default_val = match f.cardinality {
                Cardinality::ZeroOrMore | Cardinality::OneOrMore => quote! { Vec::new() },
                _ => quote! { None },
            };
            quote! { #ident: #default_val, }
        })
        .collect();

    let setters: Vec<TokenStream> = fields.iter().map(gen_setter).collect();

    let required_checks: Vec<TokenStream> = fields
        .iter()
        .filter(|f| f.cardinality == Cardinality::Required)
        .map(|f| {
            let ident = &f.ident;
            let msg = format!("required field `{}` not set", f.oscal_name);
            quote! {
                let #ident = self.#ident.ok_or_else(|| BuildError::MissingField(#msg))?;
            }
        })
        .collect();

    // Required fields use local bindings; optional/vec fields use self.field.
    let field_inits: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            if f.cardinality == Cardinality::Required {
                quote! { #ident, }
            } else {
                quote! { #ident: self.#ident, }
            }
        })
        .collect();

    // Build doc strings with the actual type names. Doc comments inside quote! are
    // treated as string literals and do NOT interpolate proc-macro2 Ident variables,
    // so we construct the strings explicitly and emit them via #[doc = ...].
    let sn = struct_name.to_string();
    let bn = builder_name.to_string();
    let builder_doc = format!(
        "Builder for [`{sn}`].\n\nConstruct via [`{sn}::builder()`], chain setters, then call [`{bn}::build()`].",
    );
    let build_doc = format!("Consume the builder and return a fully constructed [`{sn}`].");

    quote! {
        #[doc = #builder_doc]
        #[must_use]
        #[derive(Debug)]
        pub struct #builder_name {
            #(#builder_field_decls)*
        }

        impl #builder_name {
            /// Create an empty builder with all fields unset.
            pub fn new() -> Self {
                Self { #(#new_inits)* }
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl #builder_name {

            #(#setters)*

            #[doc = #build_doc]
            ///
            /// # Errors
            ///
            /// Returns [`BuildError::MissingField`] if any required field was not set.
            pub fn build(self) -> ::core::result::Result<#struct_name, BuildError> {
                #(#required_checks)*
                Ok(#struct_name { #(#field_inits)* })
            }
        }

        impl #struct_name {
            /// Return a new builder for this type.
            pub fn builder() -> #builder_name {
                #builder_name::new()
            }
        }
    }
}

// ── Field descriptor ──────────────────────────────────────────────────────────

struct BuilderField {
    ident: Ident,
    oscal_name: String,
    cardinality: Cardinality,
    /// The type as stored in the builder (always `Option<T>` or `Vec<T>`).
    builder_ty: TokenStream,
    /// The inner type `T`.
    inner_ty: TokenStream,
}

fn collect_builder_fields(def: &AssemblyDef, db: &IrDb) -> Vec<BuilderField> {
    let mut out: Vec<BuilderField> = Vec::new();
    let parent_camel = def.name.to_upper_camel_case();

    for flag in &def.flags {
        let cardinality = if flag.required {
            Cardinality::Required
        } else {
            Cardinality::Optional
        };
        let inner = data_type_tokens(&flag.as_type);
        let builder_ty = quote! { Option<#inner> };
        out.push(BuilderField {
            ident: Ident::new(&to_field_ident(&flag.name), Span::call_site()),
            oscal_name: flag.name.clone(),
            cardinality,
            builder_ty,
            inner_ty: inner,
        });
    }

    let mut choice_idx: usize = 0;

    for item in &def.model {
        match item {
            ModelItem::AssemblyRef(r) => {
                let eff = r.effective_name();
                let json_name = r.grouping.as_ref().map_or(eff, |g| g.name.as_str());
                let type_name = db.assemblies.get(r.name.as_str()).map_or_else(
                    || r.name.to_upper_camel_case(),
                    |d| d.name.to_upper_camel_case(),
                );
                let inner = {
                    let t = format_ident!("{}", sanitize_type_name(&type_name));
                    quote! { #t }
                };
                push_ref_field(&mut out, json_name, r.cardinality, inner);
            }
            ModelItem::FieldRef(r) => {
                let eff = r.effective_name();
                let json_name = r.grouping.as_ref().map_or(eff, |g| g.name.as_str());
                let inner = if let Some(fd) = db.fields.get(r.name.as_str()) {
                    if fd.flags.is_empty() {
                        data_type_tokens(&fd.as_type)
                    } else {
                        let t =
                            format_ident!("{}", sanitize_type_name(&fd.name.to_upper_camel_case()));
                        quote! { #t }
                    }
                } else if r.name == "remarks" || r.name.contains("prose") {
                    quote! { crate::primitives::MarkupMultiline }
                } else {
                    quote! { String }
                };
                push_ref_field(&mut out, json_name, r.cardinality, inner);
            }
            ModelItem::InlineAssembly(inner, cardinality) => {
                let type_name =
                    format_ident!("{}", sanitize_type_name(&inner.name.to_upper_camel_case()));
                let inner_ty = quote! { #type_name };
                let field_name = inner.name.as_str();
                push_ref_field(&mut out, field_name, *cardinality, inner_ty);
            }
            ModelItem::InlineField(inner, cardinality) => {
                let inner_ty = data_type_tokens(&inner.as_type);
                push_ref_field(&mut out, &inner.name, *cardinality, inner_ty);
            }
            ModelItem::Choice(_) => {
                // Mirror the name gen_choice uses: `{ParentCamel}Choice{N}` → snake_case.
                choice_idx += 1;
                let enum_name = format!("{parent_camel}Choice{choice_idx}");
                let field_name = enum_name.to_snake_case();
                let enum_ident = format_ident!("{}", enum_name);
                let inner = quote! { #enum_ident };
                push_ref_field(&mut out, &field_name, Cardinality::Optional, inner);
            }
        }
    }

    out
}

fn push_ref_field(
    out: &mut Vec<BuilderField>,
    name: &str,
    cardinality: Cardinality,
    inner: TokenStream,
) {
    let builder_ty = match cardinality {
        Cardinality::Required | Cardinality::Optional => quote! { Option<#inner> },
        Cardinality::ZeroOrMore | Cardinality::OneOrMore => quote! { Vec<#inner> },
    };
    out.push(BuilderField {
        ident: Ident::new(&to_field_ident(name), Span::call_site()),
        oscal_name: name.to_owned(),
        cardinality,
        builder_ty,
        inner_ty: inner,
    });
}

// ── Setter generation ─────────────────────────────────────────────────────────

fn gen_setter(f: &BuilderField) -> TokenStream {
    // Keep keyword-escape trailing underscore (e.g. type_ → type_).
    let fn_name = format_ident!("{}", to_field_ident(&f.oscal_name));
    let ident = &f.ident;
    let inner = &f.inner_ty;
    let doc = format!("Set the `{}` field.", f.oscal_name);

    match f.cardinality {
        Cardinality::Required | Cardinality::Optional => quote! {
            #[doc = #doc]
            pub fn #fn_name(mut self, v: impl Into<#inner>) -> Self {
                self.#ident = Some(v.into());
                self
            }
        },
        Cardinality::ZeroOrMore | Cardinality::OneOrMore => quote! {
            #[doc = #doc]
            pub fn #fn_name(mut self, v: impl Into<#inner>) -> Self {
                self.#ident.push(v.into());
                self
            }
        },
    }
}
