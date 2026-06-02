//! Embedded OSCAL community catalogs for AI governance and security frameworks.
//!
//! These catalog JSON files are compiled into the binary via [`include_str!`].
//! Load any catalog by passing its short name to [`embedded_catalog_json`] or
//! [`crate::generated::catalog::load_catalog`].
//!
//! # Embedded catalogs
//!
//! | Name | Framework |
//! |---|---|
//! | `nist-800-171` | NIST SP 800-171 Rev 3 — CUI protection (CMMC Level 2) |
//! | `nist-800-172` | NIST SP 800-172 Rev 3 — enhanced CUI (CMMC Level 3) |
//! | `nist-ai-rmf`  | NIST AI Risk Management Framework |
//! | `eu-ai-act`    | EU AI Act compliance controls |
//! | `iso-42001`    | ISO/IEC 42001 AI management system |
//! | `hipaa`        | HIPAA Security Rule |
//! | `fda-samd`     | FDA SaMD guidance controls |
//! | `nist-800-207` | NIST SP 800-207 Zero Trust Architecture |

/// Short names of all OSCAL catalogs embedded in this crate.
///
/// Pass any of these to [`embedded_catalog_json`] or
/// [`crate::generated::catalog::load_catalog`] via `CatalogSource::Embedded`.
pub const EMBEDDED_CATALOG_NAMES: &[&str] = &[
    "eu-ai-act",
    "nist-ai-rmf",
    "iso-42001",
    "hipaa",
    "fda-samd",
    "nist-800-207",
    "nist-800-171",
    "nist-800-172",
];

/// Returns the raw JSON string for an embedded catalog by short name.
///
/// # Returns
///
/// `Some(&str)` if `name` matches a known catalog, `None` otherwise.
#[must_use]
pub fn embedded_catalog_json(name: &str) -> Option<&'static str> {
    match name {
        "eu-ai-act" => Some(include_str!("../catalogs/eu-ai-act.json")),
        "nist-ai-rmf" => Some(include_str!("../catalogs/nist-ai-rmf.json")),
        "iso-42001" => Some(include_str!("../catalogs/iso-42001.json")),
        "hipaa" => Some(include_str!("../catalogs/hipaa.json")),
        "fda-samd" => Some(include_str!("../catalogs/fda-samd.json")),
        "nist-800-207" => Some(include_str!("../catalogs/nist-800-207.json")),
        "nist-800-171" => Some(include_str!("../catalogs/nist-800-171.json")),
        "nist-800-172" => Some(include_str!("../catalogs/nist-800-172.json")),
        _ => None,
    }
}
