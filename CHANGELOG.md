# Changelog

## Unreleased

## 0.1.2 — 2026-11-28

### Added

- A Cargo feature per embedded catalog, named after it (`eu-ai-act`, `nist-ai-rmf`, `iso-42001`,
  `hipaa`, `fda-samd`, `nist-800-207`, `nist-800-171`, `nist-800-172`), all on by default. The
  features choose which catalogs `available()` lists. Every catalog's JSON is still compiled
  into the binary, whatever features are enabled.
- `EMBEDDED_CATALOGS` and `available()`: the embedded catalogs, for catalog pickers.
- `Catalog::find_control`: look up one control by id.
- `load_catalog_from_str`: parse catalog JSON from a string.

### Changed

- Builder documentation names the struct each builder produces.

### Deprecated

- `generated::types::GroupChoice1`. Nothing has used it since 0.1.1 gave `Group` flat `controls`
  and `groups` fields. It will be removed in 0.2.

### Fixed

- **ISO/IEC 42001 catalog:** control numbering and titles now match the official Annex A,
  including A.4.1, A.6.1.5 and A.6.2.4 (event logs).
- **EU AI Act catalog:** the Article 9 testing citations and the Annex IV references are corrected.
- Broken intra-doc links in `catalogs.rs`.

## 0.1.1 — 2026-06-03

### Fixed

- `Group` has flat `controls` and `groups` fields, matching OSCAL's JSON. 0.1.0 modelled them as
  a `group_choice1` enum that never deserialized, so `Catalog::all_controls()` missed every
  control nested in a group.

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
