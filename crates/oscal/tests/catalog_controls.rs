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
