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

/// Short names of all OSCAL catalogs embedded in this build of the crate.
///
/// Each catalog is gated behind a same-named Cargo feature (all enabled by
/// default); building with `--no-default-features --features <name>` embeds
/// only that catalog's JSON, shrinking the binary.
///
/// Pass any of these to [`embedded_catalog_json`] or
/// [`crate::generated::catalog::load_catalog`] via `CatalogSource::Embedded`.
pub const EMBEDDED_CATALOG_NAMES: &[&str] = &[
    #[cfg(feature = "eu-ai-act")]
    "eu-ai-act",
    #[cfg(feature = "nist-ai-rmf")]
    "nist-ai-rmf",
    #[cfg(feature = "iso-42001")]
    "iso-42001",
    #[cfg(feature = "hipaa")]
    "hipaa",
    #[cfg(feature = "fda-samd")]
    "fda-samd",
    #[cfg(feature = "nist-800-207")]
    "nist-800-207",
    #[cfg(feature = "nist-800-171")]
    "nist-800-171",
    #[cfg(feature = "nist-800-172")]
    "nist-800-172",
];

/// Short name and human-readable description of every catalog embedded in
/// this build of the crate, in the same order as [`EMBEDDED_CATALOG_NAMES`].
pub const EMBEDDED_CATALOGS: &[(&str, &str)] = &[
    #[cfg(feature = "eu-ai-act")]
    ("eu-ai-act", "EU AI Act compliance controls"),
    #[cfg(feature = "nist-ai-rmf")]
    ("nist-ai-rmf", "NIST AI Risk Management Framework"),
    #[cfg(feature = "iso-42001")]
    ("iso-42001", "ISO/IEC 42001 AI management system"),
    #[cfg(feature = "hipaa")]
    ("hipaa", "HIPAA Security Rule"),
    #[cfg(feature = "fda-samd")]
    ("fda-samd", "FDA SaMD guidance controls"),
    #[cfg(feature = "nist-800-207")]
    ("nist-800-207", "NIST SP 800-207 Zero Trust Architecture"),
    #[cfg(feature = "nist-800-171")]
    (
        "nist-800-171",
        "NIST SP 800-171 Rev 3 — CUI protection (CMMC Level 2)",
    ),
    #[cfg(feature = "nist-800-172")]
    (
        "nist-800-172",
        "NIST SP 800-172 Rev 3 — enhanced CUI (CMMC Level 3)",
    ),
];

/// Returns the short name and description of every catalog embedded in this
/// crate, for use in catalog pickers or discovery UIs.
#[must_use]
pub fn available() -> &'static [(&'static str, &'static str)] {
    EMBEDDED_CATALOGS
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogs_available_lists_every_embedded_name() {
        let listed: Vec<&str> = available().iter().map(|(name, _)| *name).collect();
        for name in EMBEDDED_CATALOG_NAMES {
            assert!(
                listed.contains(name),
                "{name} missing from catalogs::available()"
            );
            assert!(embedded_catalog_json(name).is_some());
        }
        assert_eq!(listed.len(), EMBEDDED_CATALOG_NAMES.len());
    }
}
