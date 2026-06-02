//! Intermediate representation types for the OSCAL Metaschema.
//!
//! The XML parser populates an [`IrDb`] which the code generator consumes.
//! All types are owned; the parser resolves cross-references before returning.

#![allow(dead_code, clippy::pedantic, clippy::nursery)]
// IR fields defined for completeness; not all are consumed by v0.1 codegen.

use indexmap::IndexMap;

// ── Top-level registry ────────────────────────────────────────────────────────

/// All OSCAL definitions parsed from one or more metaschema files.
///
/// Assemblies, fields, and flags are stored in insertion-order maps so that
/// generated output is deterministic across runs.
#[derive(Debug, Default)]
pub struct IrDb {
    /// All `define-assembly` definitions keyed by their `name` attribute.
    pub assemblies: IndexMap<String, AssemblyDef>,
    /// All `define-field` definitions keyed by their `name` attribute.
    pub fields: IndexMap<String, FieldDef>,
    /// All `define-flag` definitions keyed by their `name` attribute.
    pub flags: IndexMap<String, FlagDef>,
    /// Metaschema source file short-names in parse order, used to assign
    /// each definition to an output module (e.g. "oscal-catalog" → catalog.rs).
    pub sources: IndexMap<String, Vec<String>>,
}

impl IrDb {
    /// Returns `true` if the named assembly directly or transitively contains
    /// a reference back to itself (e.g. `control` → `controls: Vec<Control>`).
    ///
    /// Used by the code generator to decide whether recursive fields need
    /// indirection — in practice all OSCAL recursive refs are `Vec<T>` which
    /// is already heap-allocated, so the generator uses this for documentation
    /// only.
    #[must_use]
    pub fn is_cyclic(&self, name: &str) -> bool {
        self.cycle_check(name, name, 0)
    }

    fn cycle_check(&self, start: &str, current: &str, depth: usize) -> bool {
        if depth > 20 {
            return false;
        }
        let Some(def) = self.assemblies.get(current) else {
            return false;
        };
        for item in &def.model {
            let ref_name = match item {
                ModelItem::AssemblyRef(r) => r.name.as_str(),
                ModelItem::InlineAssembly(d, _) => d.name.as_str(),
                _ => continue,
            };
            if ref_name == start {
                return true;
            }
            if self.cycle_check(start, ref_name, depth + 1) {
                return true;
            }
        }
        false
    }
}

// ── Definition types ──────────────────────────────────────────────────────────

/// An OSCAL `define-assembly` definition.
#[derive(Debug, Clone)]
pub struct AssemblyDef {
    /// The `name` attribute value.
    pub name: String,
    /// Human-readable label from `<formal-name>`.
    pub formal_name: String,
    /// Prose description from `<description>`.
    pub description: String,
    /// The XML element / JSON key used as the document root, from `<root-name>`.
    /// Present only on the primary assembly of each OSCAL document type.
    pub root_name: Option<String>,
    /// Flags declared directly on this assembly (from `<define-flag>` children
    /// that appear before `<model>`).
    pub flags: Vec<FlagInstance>,
    /// Ordered list of model items from the `<model>` element.
    pub model: Vec<ModelItem>,
    /// Short-name of the metaschema file that defined this assembly.
    pub source: String,
}

/// An OSCAL `define-field` definition.
#[derive(Debug, Clone)]
pub struct FieldDef {
    /// The `name` attribute value.
    pub name: String,
    /// Human-readable label.
    pub formal_name: String,
    /// Prose description.
    pub description: String,
    /// The scalar type of this field's value.
    pub as_type: DataType,
    /// Flags that annotate this field.
    pub flags: Vec<FlagInstance>,
    /// Short-name of the metaschema file that defined this field.
    pub source: String,
}

/// An OSCAL `define-flag` definition.
#[derive(Debug, Clone)]
pub struct FlagDef {
    /// The `name` attribute value.
    pub name: String,
    /// Human-readable label.
    pub formal_name: String,
    /// Prose description.
    pub description: String,
    /// The scalar type of this flag's value.
    pub as_type: DataType,
    /// Whether `required="yes"` is present.
    pub required: bool,
    /// Short-name of the metaschema file that defined this flag.
    pub source: String,
}

/// A flag as it appears inline on an assembly or field definition.
#[derive(Debug, Clone)]
pub struct FlagInstance {
    /// The effective name (from `<define-flag name="...">` or `<flag ref="..."`).
    pub name: String,
    /// Scalar data type.
    pub as_type: DataType,
    /// Whether `required="yes"`.
    pub required: bool,
    /// Documentation string.
    pub description: String,
    /// Default value from `default` attribute, if any.
    pub default: Option<String>,
    /// Enumerated allowed values (from `<allowed-values>` constraints).
    pub allowed: Vec<String>,
}

// ── Model items ───────────────────────────────────────────────────────────────

/// One item in an assembly's `<model>` block.
#[derive(Debug, Clone)]
pub enum ModelItem {
    /// A `<assembly ref="...">` reference.
    AssemblyRef(ModelRef),
    /// A `<field ref="...">` reference.
    FieldRef(ModelRef),
    /// An inline `<define-assembly>` with its effective cardinality from `min-occurs`.
    InlineAssembly(AssemblyDef, Cardinality),
    /// An inline `<define-field>` with its effective cardinality from `min-occurs`.
    InlineField(FieldDef, Cardinality),
    /// A `<choice>` block — exactly one of the contained items applies.
    Choice(Vec<ModelItem>),
}

/// A reference to a named assembly or field within a model, with its cardinality
/// and optional JSON grouping configuration.
#[derive(Debug, Clone)]
pub struct ModelRef {
    /// The `ref` attribute — name of the target `define-assembly` or `define-field`.
    pub name: String,
    /// An override for the serialized key name (`<use-name>` element).
    pub use_name: Option<String>,
    /// Cardinality derived from `min-occurs` and `max-occurs` attributes.
    pub cardinality: Cardinality,
    /// JSON grouping config from a `<group-as>` child element.
    pub grouping: Option<GroupingConfig>,
    /// Additional documentation specific to this usage context.
    pub description: String,
}

impl ModelRef {
    /// Returns the effective JSON / Rust field name: `use_name` if present,
    /// otherwise `name`.
    #[must_use]
    pub fn effective_name(&self) -> &str {
        self.use_name.as_deref().unwrap_or(self.name.as_str())
    }
}

// ── Cardinality ───────────────────────────────────────────────────────────────

/// The cardinality of a model reference, derived from `min-occurs`/`max-occurs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality {
    /// `min=1, max=1` — required single value → `T`
    Required,
    /// `min=0, max=1` — optional single value → `Option<T>`
    Optional,
    /// `min=0, max=unbounded` — optional collection → `Vec<T>`
    ZeroOrMore,
    /// `min=1, max=unbounded` — non-empty collection → `Vec<T>`
    OneOrMore,
}

impl Cardinality {
    /// Construct from raw `min-occurs` / `max-occurs` attribute strings.
    #[must_use]
    pub fn from_min_max(min: Option<&str>, max: Option<&str>) -> Self {
        let min_n: u32 = min.and_then(|s| s.parse().ok()).unwrap_or(0);
        let unbounded = max.map_or(false, |s| s == "unbounded");
        let max_n: u32 = if unbounded {
            u32::MAX
        } else {
            max.and_then(|s| s.parse().ok()).unwrap_or(1)
        };

        match (min_n, max_n) {
            (1, 1) => Self::Required,
            (0, 1) => Self::Optional,
            (0, _) if max_n > 1 => Self::ZeroOrMore,
            (1, _) if max_n > 1 => Self::OneOrMore,
            _ => Self::Optional,
        }
    }
}

// ── Grouping ──────────────────────────────────────────────────────────────────

/// JSON serialization grouping from a `<group-as>` element.
#[derive(Debug, Clone)]
pub struct GroupingConfig {
    /// The `name` attribute — the JSON key wrapping this collection.
    pub name: String,
    /// The `in-json` attribute value.
    pub in_json: JsonGrouping,
}

/// How an array is represented in JSON output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonGrouping {
    /// Always an array: `"controls": [...]`
    Array,
    /// Array when multiple, object when single.
    SingletonOrArray,
    /// Map keyed by a flag value: `"params": { "id1": {...}, "id2": {...} }`
    ByKey,
}

impl JsonGrouping {
    /// Parse the `in-json` attribute value.
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "ARRAY" => Self::Array,
            "BY_KEY" => Self::ByKey,
            _ => Self::SingletonOrArray,
        }
    }
}

// ── Data types ────────────────────────────────────────────────────────────────

/// Every primitive data type defined in the OSCAL Metaschema specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    // Strings
    StringType,
    Token,
    Email,
    Hostname,
    Base64,
    IpV4,
    IpV6,
    // Identifiers
    /// RFC 4122 UUID → `uuid::Uuid`
    Uuid,
    // URIs
    /// Absolute URI → `url::Url`
    Uri,
    /// Absolute or relative URI → `crate::primitives::UriReference`
    UriReference,
    // Temporals — RFC 3339 / ISO 8601
    /// Date without required timezone → `chrono::NaiveDate`
    Date,
    /// Date with required timezone → `chrono::NaiveDate`
    DateWithTimezone,
    /// Date-time without required timezone → `String`
    DateTime,
    /// Date-time with required timezone → `chrono::DateTime<chrono::Utc>`
    DateTimeWithTimezone,
    DayTimeDuration,
    YearMonthDuration,
    // Numerics
    /// Decimal real number → `String` (to preserve precision)
    Decimal,
    /// Signed integer → `i64`
    Integer,
    /// Non-negative integer → `u64`
    NonNegativeInteger,
    /// Positive integer → `u64`
    PositiveInteger,
    // Boolean
    Boolean,
    // Rich text
    /// Inline markup → `crate::primitives::MarkupLine`
    MarkupLine,
    /// Block markup → `crate::primitives::MarkupMultiline`
    MarkupMultiline,
}

impl DataType {
    /// Parse an OSCAL `as-type` attribute string.
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "uuid" => Self::Uuid,
            "uri" => Self::Uri,
            "uri-reference" => Self::UriReference,
            "date" => Self::Date,
            "date-with-timezone" => Self::DateWithTimezone,
            "date-time" => Self::DateTime,
            "date-time-with-timezone" => Self::DateTimeWithTimezone,
            "day-time-duration" => Self::DayTimeDuration,
            "year-month-duration" => Self::YearMonthDuration,
            "decimal" => Self::Decimal,
            "integer" => Self::Integer,
            "non-negative-integer" => Self::NonNegativeInteger,
            "positive-integer" => Self::PositiveInteger,
            "boolean" => Self::Boolean,
            "markup-line" => Self::MarkupLine,
            "markup-multiline" => Self::MarkupMultiline,
            "email-address" => Self::Email,
            "hostname" => Self::Hostname,
            "base64" => Self::Base64,
            "ip-v4-address" => Self::IpV4,
            "ip-v6-address" => Self::IpV6,
            "token" => Self::Token,
            // "string" and everything else falls through to StringType
            _ => Self::StringType,
        }
    }
}
