//! Regression test guarding against silent control-traversal data loss.
//!
//! `Catalog::all_controls()` must see every control nested under top-level
//! `controls` and (recursively) under `groups`. A prior bug left `Group`
//! controls/groups unreachable, so embedded catalogs that nest controls
//! inside groups silently returned zero controls.

#[test]
fn all_controls_nonempty_for_grouped_catalogs() {
    for name in ["nist-800-171", "nist-800-172", "hipaa"] {
        let catalog = oscal::load_catalog(name).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert!(
            !catalog.all_controls().is_empty(),
            "{name} returned 0 controls from all_controls()"
        );
    }
}

#[test]
#[cfg(feature = "iso-42001")]
fn iso_annex_a_uses_the_2023_control_identifiers() {
    let catalog = oscal::load_catalog("iso-42001").unwrap();
    let controls = catalog.all_controls();
    assert_eq!(controls.len(), 38);
    let find = |suffix: &str| {
        controls
            .iter()
            .find(|control| control.id == format!("iso-42001-a.{suffix}"))
            .map(|control| control.title.as_ref())
    };
    assert_eq!(find("6.1.5"), None);
    assert_eq!(find("4.1"), None);
    assert_eq!(
        find("6.2.2"),
        Some("A.6.2.2 — Requirements and specifications")
    );
    assert_eq!(find("6.2.4"), Some("A.6.2.4 — Verification and validation"));
    assert_eq!(find("6.2.8"), Some("A.6.2.8 — Event log recording"));
    assert_eq!(find("7.5"), Some("A.7.5 — Data provenance tracking"));
}

#[test]
#[cfg(feature = "eu-ai-act")]
fn eu_threshold_and_technical_documentation_citations_resolve() {
    let catalog = oscal::load_catalog("eu-ai-act").unwrap();
    let controls = catalog.all_controls();
    let prose = |id: &str| {
        controls
            .iter()
            .find(|control| control.id == format!("eu-ai-act-{id}"))
            .and_then(|control| control.statement())
            .unwrap()
    };
    assert!(prose("art-9-5").contains("residual risk"));
    assert!(prose("art-9-8").contains("probabilistic thresholds"));
    assert!(prose("annex-iv-2d").contains("training"));
    assert!(prose("annex-iv-2e").contains("human oversight"));
    assert!(prose("annex-iv-2g").contains("validation procedures"));
    assert!(prose("annex-iv-2h").contains("cybersecurity"));
    assert!(prose("annex-iv-9").contains("post-market monitoring"));
}
