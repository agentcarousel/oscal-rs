//! Strong primitive types for OSCAL's rich-text and URI data types.
//!
//! These wrap `String` but carry semantic meaning in the type system and ensure
//! correct serde round-trips. Downstream code should prefer these over bare
//! `String` for any OSCAL field typed `markup-line`, `markup-multiline`, or
//! `uri-reference`.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Inline rich text with emphasis, links, code spans, and similar inline formatting.
///
/// Maps to the OSCAL Metaschema `markup-line` data type. The inner string may
/// contain Markdown or a limited HTML subset depending on the serialization
/// format; this crate stores it verbatim without parsing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct MarkupLine(pub String);

/// Block-level rich text: paragraphs, headings, lists, tables, and inline markup.
///
/// Maps to the OSCAL Metaschema `markup-multiline` data type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct MarkupMultiline(pub String);

/// A URI reference that may be relative, absolute, or a bare fragment (`#uuid`).
///
/// Maps to the OSCAL Metaschema `uri-reference` data type. Unlike `url::Url`,
/// this type accepts relative URIs such as `#uuid-back-matter-ref` which appear
/// extensively in OSCAL back-matter cross-references.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct UriReference(pub String);

// ── MarkupLine conversions ────────────────────────────────────────────────────

impl From<&str> for MarkupLine {
    #[inline]
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for MarkupLine {
    #[inline]
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for MarkupLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for MarkupLine {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// ── MarkupMultiline conversions ───────────────────────────────────────────────

impl From<&str> for MarkupMultiline {
    #[inline]
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for MarkupMultiline {
    #[inline]
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for MarkupMultiline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for MarkupMultiline {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// ── UriReference conversions ──────────────────────────────────────────────────

impl From<&str> for UriReference {
    #[inline]
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for UriReference {
    #[inline]
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for UriReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for UriReference {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}
