//! Helpers for generating serde attributes on struct fields.

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::Cardinality;

/// Rust keywords that cannot be used as field identifiers without renaming.
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof",
    "unsized", "virtual", "yield", "async", "await", "dyn", "try",
];

/// Convert an OSCAL kebab-case name to a Rust snake_case field name.
/// If the result is a Rust keyword, appends `_` as per Rust convention.
#[must_use]
pub fn to_field_ident(name: &str) -> String {
    let snake = name.replace('-', "_").to_snake_case();
    if RUST_KEYWORDS.contains(&snake.as_str()) {
        format!("{snake}_")
    } else {
        snake
    }
}

/// Generate the `#[serde(...)]` attribute token stream for a struct field.
///
/// Handles:
/// - `rename` when Rust field name doesn't round-trip through kebab-case
/// - `skip_serializing_if` for optional fields and empty vecs
/// - `default` for vec fields (to accept absent arrays)
#[must_use]
pub fn field_serde_attr(oscal_name: &str, cardinality: Cardinality) -> TokenStream {
    let field_ident = to_field_ident(oscal_name);

    // What serde rename_all="kebab-case" would produce from the Rust field name.
    // e.g. "control_id" → "control-id", "type_" → "type-" (wrong without rename).
    let serde_produced = field_ident.replace('_', "-");

    // We need a rename when serde's automatic key differs from the OSCAL JSON key.
    let rename_part: TokenStream = if serde_produced != oscal_name {
        let orig = oscal_name;
        quote! { rename = #orig, }
    } else {
        quote! {}
    };

    let skip_part: TokenStream = match cardinality {
        Cardinality::Optional => {
            quote! { skip_serializing_if = "Option::is_none", }
        }
        Cardinality::ZeroOrMore => {
            quote! { default, skip_serializing_if = "Vec::is_empty", }
        }
        Cardinality::OneOrMore => {
            quote! { default, }
        }
        Cardinality::Required => {
            quote! {}
        }
    };

    let combined = quote! { #rename_part #skip_part };
    let has_content = !combined.is_empty();

    if has_content {
        quote! { #[serde(#rename_part #skip_part)] }
    } else {
        quote! {}
    }
}
