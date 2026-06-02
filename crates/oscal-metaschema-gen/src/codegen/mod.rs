//! Code generation: converts the [`IrDb`] into Rust source files.
pub mod builders;
pub mod emit;
pub mod serde_attrs;
pub mod structs;

use anyhow::Context;
use proc_macro2::TokenStream;

use crate::ir::types::IrDb;

/// Generate all modules from the IR database.
///
/// All types are emitted into a single flat `types` module to avoid
/// cross-module reference issues. Returns a list of `(module_name,
/// token_stream)` pairs (currently always length 1).
///
/// # Errors
///
/// Returns an error if token-stream generation fails.
pub fn generate_all(db: &IrDb) -> anyhow::Result<Vec<(String, TokenStream)>> {
    // Collect all assembly/field names in stable order across all sources.
    let mut all_names: Vec<String> = Vec::new();
    // Maintain insertion order deterministically.
    for names in db.sources.values() {
        all_names.extend(names.iter().cloned());
    }
    // Dedup while preserving first-occurrence order.
    let mut seen = std::collections::HashSet::new();
    all_names.retain(|n| seen.insert(n.clone()));

    let tokens =
        structs::gen_module("types", &all_names, db).context("codegen failed for types module")?;

    Ok(vec![("types".to_owned(), tokens)])
}
