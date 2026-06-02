//! OSCAL (Open Security Controls Assessment Language) Rust types.
//!
//! Types are generated from the official [NIST OSCAL Metaschema][ms] and cover
//! all seven OSCAL document models. Embedded community catalogs for AI
//! governance and security frameworks are available via [`catalogs`].
//!
//! [ms]: https://pages.nist.gov/OSCAL/
//!
//! # Quick start
//!
//! ```rust
//! use oscal::load_catalog;
//!
//! # fn main() -> Result<(), String> {
//! let catalog = load_catalog("nist-800-171")?;
//! println!("Controls: {}", catalog.all_controls().len());
//! # Ok(())
//! # }
//! ```

#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::multiple_crate_versions,
    clippy::module_name_repetitions,
    clippy::wildcard_imports
)]
#![forbid(unsafe_code)]

pub mod catalogs;
pub mod generated;
mod impls;
pub mod primitives;

pub use generated::types::Catalog;
pub use primitives::{MarkupLine, MarkupMultiline, UriReference};

/// Load an embedded OSCAL catalog by short name.
///
/// Short names: `nist-800-171`, `nist-800-172`, `nist-ai-rmf`, `eu-ai-act`,
/// `iso-42001`, `hipaa`, `fda-samd`, `nist-800-207`.
///
/// # Errors
///
/// Returns an error string if `name` is unknown or if the embedded JSON fails
/// to parse.
pub fn load_catalog(name: &str) -> Result<Catalog, String> {
    let json = catalogs::embedded_catalog_json(name)
        .ok_or_else(|| format!("unknown embedded catalog '{name}'"))?;
    let doc: CatalogDoc =
        serde_json::from_str(json).map_err(|e| format!("catalog parse error: {e}"))?;
    Ok(doc.catalog)
}

#[derive(serde::Deserialize)]
struct CatalogDoc {
    catalog: Catalog,
}
