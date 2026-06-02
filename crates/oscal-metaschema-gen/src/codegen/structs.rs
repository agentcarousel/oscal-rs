//! Generates Rust struct and enum token streams from [`AssemblyDef`] / [`FieldDef`].

use std::collections::HashSet;

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::{
    codegen::serde_attrs::{field_serde_attr, to_field_ident},
    ir::types::{
        AssemblyDef, Cardinality, DataType, FieldDef, FlagInstance, IrDb, ModelItem, ModelRef,
    },
};

// ── Public entry point ────────────────────────────────────────────────────────

/// Generate the full token stream for one output module.
///
/// `names` is the list of assembly/field names assigned to this module.
/// Cross-module types are referenced as `super::common::Foo` etc. (unused in
/// the flat generated layout where all non-common modules import from common).
pub fn gen_module(module_name: &str, names: &[String], db: &IrDb) -> anyhow::Result<TokenStream> {
    let mut tokens = TokenStream::new();

    let mod_doc =
        format!("Generated OSCAL types for the `{module_name}` model. Do not edit by hand.");
    tokens.extend(quote! {
        #![doc = #mod_doc]
        // Generated code — many clippy lints are intentionally suppressed.
        #![allow(
            clippy::all,
            clippy::pedantic,
            clippy::nursery,
            unused_imports,
            unused_mut,
            unreachable_code,
            dead_code
        )]
        use serde::{Deserialize, Serialize};
    });

    // The flat "types" module has everything in one place — no cross-module imports needed.
    // For legacy multi-module layout, non-common modules import from common.
    if module_name != "common" && module_name != "types" {
        tokens.extend(quote! {
            use super::common::*;
        });
    }

    // One shared BuildError per module — all builders in this module reference it.
    tokens.extend(quote! {
        /// Error returned when a required field was not set before calling `build()`.
        #[derive(Debug, thiserror::Error, PartialEq, Eq)]
        pub enum BuildError {
            #[error("missing required field: {0}")]
            MissingField(&'static str),
        }
    });

    // Track emitted struct names to prevent duplicates from inline definitions.
    let mut emitted: HashSet<String> = HashSet::new();

    for name in names {
        if let Some(def) = db.assemblies.get(name.as_str()) {
            let type_key = sanitize_type_name(&def.name.to_upper_camel_case());
            if !emitted.insert(type_key) {
                continue; // already generated (e.g. multiple sources define same assembly name)
            }
            let mut extra: Vec<TokenStream> = Vec::new();
            let struct_ts = gen_assembly(def, db, &mut extra, &mut emitted)?;
            tokens.extend(extra);
            tokens.extend(struct_ts);
            let builder_ts = crate::codegen::builders::gen_builder(def, db);
            tokens.extend(builder_ts);
        } else if let Some(def) = db.fields.get(name.as_str()) {
            let struct_name = sanitize_type_name(&def.name.to_upper_camel_case());
            if emitted.insert(struct_name) {
                tokens.extend(gen_field_struct(def));
            }
        }
    }

    Ok(tokens)
}

// ── Assembly → struct ─────────────────────────────────────────────────────────

/// Generate a struct definition for one assembly, plus any sibling enums for
/// `<choice>` blocks. Extra enums are pushed into `extra_before`.
pub fn gen_assembly(
    def: &AssemblyDef,
    db: &IrDb,
    extra_before: &mut Vec<TokenStream>,
    emitted: &mut HashSet<String>,
) -> anyhow::Result<TokenStream> {
    let struct_name = format_ident!("{}", sanitize_type_name(&def.name.to_upper_camel_case()));
    let doc = &def.description;

    let mut field_tokens: Vec<TokenStream> = Vec::new();

    // Flags become struct fields first (they appear first in JSON)
    for flag in &def.flags {
        field_tokens.push(gen_flag_field(flag));
    }

    // Model items
    let mut choice_idx: usize = 0;
    for item in &def.model {
        match item {
            ModelItem::AssemblyRef(r) => {
                let ft = gen_assembly_ref_field(r, db);
                field_tokens.push(ft);
            }
            ModelItem::FieldRef(r) => {
                let ft = gen_field_ref_field(r, db);
                field_tokens.push(ft);
            }
            ModelItem::InlineAssembly(inner, cardinality) => {
                let inner_type_name = inner.name.to_upper_camel_case();
                if emitted.insert(inner_type_name) {
                    let mut inner_extra: Vec<TokenStream> = Vec::new();
                    let inner_struct = gen_assembly(inner, db, &mut inner_extra, emitted)?;
                    extra_before.extend(inner_extra);
                    extra_before.push(inner_struct);
                    let builder = crate::codegen::builders::gen_builder(inner, db);
                    extra_before.push(builder);
                }
                let mr = ModelRef {
                    name: inner.name.to_owned(),
                    use_name: None,
                    cardinality: *cardinality,
                    grouping: None,
                    description: inner.description.clone(),
                };
                {
                    let ft = gen_assembly_ref_field(&mr, db);
                    field_tokens.push(ft);
                }
            }
            ModelItem::InlineField(inner, cardinality) => {
                if !inner.flags.is_empty() {
                    let struct_name = inner.name.to_upper_camel_case();
                    if emitted.insert(struct_name) {
                        extra_before.push(gen_field_struct(inner));
                    }
                }
                let field_ident_tok = field_ident(&inner.name);
                let inner_ty = data_type_tokens(&inner.as_type);
                let wrapped = wrap_type(inner_ty, *cardinality);
                let serde_attr = field_serde_attr(&inner.name, *cardinality);
                let doc = &inner.description;
                field_tokens.push(quote! {
                    #[doc = #doc]
                    #serde_attr
                    pub #field_ident_tok: #wrapped,
                });
            }
            ModelItem::Choice(choices) => {
                choice_idx += 1;
                let enum_name =
                    format_ident!("{}Choice{}", def.name.to_upper_camel_case(), choice_idx);
                let (choice_enum, choice_field) = gen_choice(&enum_name, choices, db);
                extra_before.push(choice_enum);
                field_tokens.push(choice_field);
            }
        }
    }

    Ok(quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct #struct_name {
            #(#field_tokens)*
        }
    })
}

// ── Field generation helpers ──────────────────────────────────────────────────

fn gen_flag_field(flag: &FlagInstance) -> TokenStream {
    let ident = field_ident(&flag.name);
    let ty = data_type_tokens(&flag.as_type);
    let cardinality = if flag.required {
        Cardinality::Required
    } else {
        Cardinality::Optional
    };
    let wrapped = wrap_type(ty, cardinality);
    let serde_attr = field_serde_attr(&flag.name, cardinality);
    let doc = &flag.description;

    quote! {
        #[doc = #doc]
        #serde_attr
        pub #ident: #wrapped,
    }
}

fn gen_assembly_ref_field(r: &ModelRef, db: &IrDb) -> TokenStream {
    let eff_name = r.effective_name();

    // The JSON key is determined by group-as name when present, else the ref name.
    // Use the same name for both the Rust ident and the serde attribute so that
    // builders and structs stay in sync.
    let json_name = r
        .grouping
        .as_ref()
        .map(|g| g.name.as_str())
        .unwrap_or(eff_name);

    let ident = field_ident(json_name);

    let type_name = db.assemblies.get(r.name.as_str()).map_or_else(
        || r.name.to_upper_camel_case(),
        |d| d.name.to_upper_camel_case(),
    );
    let ty_ident = format_ident!("{}", sanitize_type_name(&type_name));

    let cardinality = r.cardinality;
    let wrapped = wrap_type(quote! { #ty_ident }, cardinality);
    let serde_attr = field_serde_attr(json_name, cardinality);
    let doc = &r.description;

    quote! {
        #[doc = #doc]
        #serde_attr
        pub #ident: #wrapped,
    }
}

fn gen_field_ref_field(r: &ModelRef, db: &IrDb) -> TokenStream {
    let eff_name = r.effective_name();

    let json_name = r
        .grouping
        .as_ref()
        .map(|g| g.name.as_str())
        .unwrap_or(eff_name);

    // Same name for ident and serde attr so struct ↔ builder stay in sync.
    let ident = field_ident(json_name);

    let cardinality = r.cardinality;

    // Look up the field definition for its data type; fall back to String.
    let inner_ty = if let Some(fd) = db.fields.get(r.name.as_str()) {
        if fd.flags.is_empty() {
            data_type_tokens(&fd.as_type)
        } else {
            let ty = format_ident!("{}", sanitize_type_name(&fd.name.to_upper_camel_case()));
            quote! { #ty }
        }
    } else {
        // Unknown field → treat as markup-multiline (common for remarks, prose)
        let n = r.name.as_str();
        if n == "remarks" || n.contains("prose") {
            quote! { crate::primitives::MarkupMultiline }
        } else {
            quote! { String }
        }
    };

    let wrapped = wrap_type(inner_ty, cardinality);
    let serde_attr = field_serde_attr(json_name, cardinality);
    let doc = &r.description;

    quote! {
        #[doc = #doc]
        #serde_attr
        pub #ident: #wrapped,
    }
}

// ── Field struct (for define-field with flags) ────────────────────────────────

fn gen_field_struct(def: &FieldDef) -> TokenStream {
    let struct_name = format_ident!("{}", sanitize_type_name(&def.name.to_upper_camel_case()));
    let doc = &def.description;
    let value_ty = data_type_tokens(&def.as_type);
    let mut fields: Vec<TokenStream> = Vec::new();

    for flag in &def.flags {
        fields.push(gen_flag_field(flag));
    }
    fields.push(quote! {
        #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
        pub value: Option<#value_ty>,
    });

    // No #[derive(Default)] — some field types (e.g. url::Url) don't implement Default.
    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct #struct_name {
            #(#fields)*
        }
    }
}

// ── Choice → enum ─────────────────────────────────────────────────────────────

fn gen_choice(enum_name: &Ident, choices: &[ModelItem], db: &IrDb) -> (TokenStream, TokenStream) {
    let mut variants: Vec<TokenStream> = Vec::new();

    for choice in choices {
        match choice {
            ModelItem::AssemblyRef(r) => {
                let variant_name =
                    format_ident!("{}", sanitize_type_name(&r.name.to_upper_camel_case()));
                let type_name =
                    format_ident!("{}", sanitize_type_name(&r.name.to_upper_camel_case()));
                let cardinality = r.cardinality;
                let ty = wrap_type(quote! { #type_name }, cardinality);
                variants.push(quote! { #variant_name(#ty), });
            }
            ModelItem::FieldRef(r) => {
                let variant_name = format_ident!("{}", r.name.to_upper_camel_case());
                let inner = if let Some(fd) = db.fields.get(r.name.as_str()) {
                    data_type_tokens(&fd.as_type)
                } else {
                    quote! { String }
                };
                let ty = wrap_type(inner, r.cardinality);
                variants.push(quote! { #variant_name(#ty), });
            }
            _ => {}
        }
    }

    let enum_def = quote! {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum #enum_name {
            #(#variants)*
        }
    };

    let field_ident_name = {
        let raw = enum_name.to_string().to_snake_case();
        format_ident!("{}", raw)
    };
    let choice_field = quote! {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub #field_ident_name: Option<#enum_name>,
    };

    (enum_def, choice_field)
}

// ── Type mapping ──────────────────────────────────────────────────────────────

/// Map an OSCAL [`DataType`] to its Rust token representation.
#[must_use]
pub fn data_type_tokens(dt: &DataType) -> TokenStream {
    match dt {
        DataType::Uuid => quote! { uuid::Uuid },
        DataType::DateTimeWithTimezone => quote! { chrono::DateTime<chrono::Utc> },
        DataType::Date | DataType::DateWithTimezone => quote! { chrono::NaiveDate },
        DataType::DateTime => quote! { String },
        DataType::Uri => quote! { url::Url },
        DataType::UriReference => quote! { crate::primitives::UriReference },
        DataType::Boolean => quote! { bool },
        DataType::Integer => quote! { i64 },
        DataType::NonNegativeInteger | DataType::PositiveInteger => quote! { u64 },
        DataType::Decimal => quote! { String },
        DataType::MarkupLine => quote! { crate::primitives::MarkupLine },
        DataType::MarkupMultiline => quote! { crate::primitives::MarkupMultiline },
        _ => quote! { String },
    }
}

/// Wrap a type token in `Option<T>` or `Vec<T>` based on cardinality.
#[must_use]
pub fn wrap_type(inner: TokenStream, card: Cardinality) -> TokenStream {
    match card {
        Cardinality::Required => inner,
        Cardinality::Optional => quote! { Option<#inner> },
        Cardinality::ZeroOrMore | Cardinality::OneOrMore => quote! { Vec<#inner> },
    }
}

// ── Identifier helpers ────────────────────────────────────────────────────────

fn field_ident(name: &str) -> Ident {
    let s = to_field_ident(name);
    Ident::new(&s, Span::call_site())
}

/// Rust standard library type names that must not be used as generated struct names.
const RESERVED_TYPE_NAMES: &[&str] = &[
    "Result", "Option", "Box", "Vec", "String", "Error", "Iterator", "Future", "Read", "Write",
    "Drop", "Clone", "Copy", "Default", "Debug", "Display", "Send", "Sync", "Sized", "From",
    "Into", "Hash", "Eq", "Ord",
];

/// Rename a generated struct/enum name if it conflicts with a Rust built-in.
/// Uses an `Oscal` prefix to disambiguate (e.g. `Result` → `OscalResult`).
#[must_use]
pub fn sanitize_type_name(camel: &str) -> String {
    if RESERVED_TYPE_NAMES.contains(&camel) {
        format!("Oscal{camel}")
    } else {
        camel.to_owned()
    }
}
