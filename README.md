# oscal-rs

[![crates.io](https://img.shields.io/crates/v/oscal.svg)](https://crates.io/crates/oscal)
[![docs.rs](https://docs.rs/oscal/badge.svg)](https://docs.rs/oscal)
[![CI](https://github.com/agentcarousel/oscal-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/agentcarousel/oscal-rs/actions/workflows/ci.yml)

Rust types for the [OSCAL](https://pages.nist.gov/OSCAL/) (Open Security Controls Assessment
Language) data models, generated directly from the official [NIST OSCAL Metaschema][ms].

[ms]: https://pages.nist.gov/OSCAL/

## What is OSCAL?

OSCAL is an XML/JSON schema standard published by NIST for expressing security control catalogs,
component descriptions, system security plans, and assessment results in machine-readable form.
It is the required format for FedRAMP authorizations and is widely used in CMMC, HIPAA, and
enterprise GRC tooling.

## How types are generated

```
[ NIST Metaschema XML ]
         │
         ▼
  oscal-metaschema-gen  ← parses XML into an IR
         │
         ▼
   quote + syn + rustfmt  ← emits syntactically valid Rust
         │
         ▼
  crates/oscal/src/generated/types.rs  ← checked in
```

When NIST updates the schema, run `cargo run -p oscal-metaschema-gen` to regenerate. A weekly
CI workflow opens a pull request automatically when the upstream schema changes.

## Quick Start

```toml
[dependencies]
oscal = "0.1"
```

```rust
use oscal::load_catalog;

fn main() -> Result<(), String> {
    let catalog = load_catalog("nist-800-171")?;

    for control in catalog.all_controls() {
        println!("{}: {}", control.id, control.title);
        if let Some(stmt) = control.statement() {
            println!("  {stmt}");
        }
    }
    Ok(())
}
```

## Embedded Catalogs

Eight community-authored OSCAL catalog JSON files are compiled into the binary and available
via [`load_catalog`]:

| Name | Framework |
|---|---|
| `nist-800-171` | NIST SP 800-171 Rev 3 — CUI protection (CMMC Level 2) |
| `nist-800-172` | NIST SP 800-172 Rev 3 — enhanced CUI (CMMC Level 3) |
| `nist-ai-rmf`  | NIST AI Risk Management Framework |
| `eu-ai-act`    | EU AI Act compliance controls |
| `iso-42001`    | ISO/IEC 42001 AI management system |
| `hipaa`        | HIPAA Security Rule |
| `fda-samd`     | FDA SaMD guidance controls |
| `nist-800-207` | NIST SP 800-207 Zero Trust Architecture |

## Generated Models

All seven OSCAL document types are covered:

| OSCAL model | Top-level type |
|---|---|
| Catalog | `Catalog` |
| Component Definition | `ComponentDefinition` |
| Profile | `Profile` |
| System Security Plan | `SystemSecurityPlan` |
| Assessment Plan | `AssessmentPlan` |
| Assessment Results | `AssessmentResults` |
| Plan of Action and Milestones | `PlanOfActionAndMilestones` |

All types derive `Debug`, `Clone`, `PartialEq`, `Serialize`, and `Deserialize`. Every assembly
also ships a builder: `Catalog::builder()` returns a `CatalogBuilder`.

## Strong Primitive Types

OSCAL's rich data types map to Rust types that carry semantic meaning:

| OSCAL type | Rust type |
|---|---|
| `uuid` | `uuid::Uuid` |
| `date-time-with-timezone` | `chrono::DateTime<Utc>` |
| `uri` | `url::Url` |
| `uri-reference` | `oscal::UriReference` (relative-safe wrapper) |
| `markup-line` | `oscal::MarkupLine` |
| `markup-multiline` | `oscal::MarkupMultiline` |

## Regenerating Types

```bash
# Fetch latest NIST metaschemas (first time or to update)
BASE=https://raw.githubusercontent.com/usnistgov/OSCAL/main/src/metaschema
for f in oscal_catalog_metaschema.xml oscal_assessment-results_metaschema.xml \
         oscal_component_metaschema.xml oscal_profile_metaschema.xml \
         oscal_ssp_metaschema.xml oscal_poam_metaschema.xml \
         oscal_assessment-plan_metaschema.xml oscal_complete_metaschema.xml \
         oscal_metadata_metaschema.xml oscal_control-common_metaschema.xml \
         oscal_assessment-common_metaschema.xml oscal_implementation-common_metaschema.xml; do
  curl -fsSL "${BASE}/${f}" -o "metaschemas/${f}"
done

# Run the generator
cargo run -p oscal-metaschema-gen
```

## Contributing

Contributions are welcome. The most useful additions are:

- Corrections to generated type mappings or serde attributes
- Additional embedded catalog JSON files for frameworks not yet covered
- Improvements to the `oscal-metaschema-gen` parser (better handling of edge cases)
- Hand-written impl blocks in `crates/oscal/src/impls.rs` for traversal utilities

Please open an issue before large structural changes.

## Origin

Extracted from [agentcarousel](https://github.com/agentcarousel/agentcarousel), a behavioral
testing and compliance attestation tool for AI agents.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
