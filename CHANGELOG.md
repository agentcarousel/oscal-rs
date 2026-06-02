# Changelog

## 0.1.0 — 2026-06-02

Initial release. Extracted from [agentcarousel](https://github.com/agentcarousel/agentcarousel).

### Types

All seven OSCAL document models are covered, generated from NIST OSCAL Metaschema 1.2.2:
`Catalog`, `ComponentDefinition`, `Profile`, `SystemSecurityPlan`, `AssessmentPlan`,
`AssessmentResults`, `PlanOfActionAndMilestones`, plus all shared common types.

### Primitives

Strong types for OSCAL's rich data types: `MarkupLine`, `MarkupMultiline`, `UriReference`.
Fields typed `uuid`, `date-time-with-timezone`, and `uri` map to `uuid::Uuid`,
`chrono::DateTime<Utc>`, and `url::Url` respectively.

### Builders

Every assembly ships a `{Name}Builder` with setter methods and a `build()` method that
validates required fields.

### Embedded Catalogs

Eight community-authored OSCAL catalog JSON files compiled into the binary:
`nist-800-171`, `nist-800-172`, `nist-ai-rmf`, `eu-ai-act`, `iso-42001`, `hipaa`,
`fda-samd`, `nist-800-207`.

### Generator

`oscal-metaschema-gen`: a CLI that parses NIST OSCAL Metaschema XML files and emits
`crates/oscal/src/generated/types.rs` via `quote` + `syn` + `rustfmt`.
