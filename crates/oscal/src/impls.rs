//! Hand-written method impls for generated OSCAL types.
//!
//! The code generator emits data types and builders; domain-specific traversal
//! methods live here so they survive metaschema regeneration.

use crate::generated::types::{Catalog, Control, Group};

impl Catalog {
    /// Returns every control in the catalog, flattening groups and nested
    /// control enhancements into a single flat list.
    #[must_use]
    pub fn all_controls(&self) -> Vec<&Control> {
        let mut out = Vec::new();
        for c in &self.controls {
            collect_controls(c, &mut out);
        }
        for g in &self.groups {
            collect_group_controls(g, &mut out);
        }
        out
    }

    /// Returns the control with the given `id`, searching the whole catalog
    /// (including nested groups and control enhancements).
    #[must_use]
    pub fn find_control(&self, id: &str) -> Option<&Control> {
        self.all_controls().into_iter().find(|c| c.id == id)
    }
}

impl Control {
    /// Returns the statement prose text from the control's `statement` part,
    /// if one exists.
    #[must_use]
    pub fn statement(&self) -> Option<&str> {
        self.parts
            .iter()
            .find(|p| p.name == "statement")
            .and_then(|p| p.prose.as_ref())
            .map(|m| m.0.as_str())
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn collect_controls<'a>(control: &'a Control, out: &mut Vec<&'a Control>) {
    out.push(control);
    for nested in &control.controls {
        collect_controls(nested, out);
    }
}

fn collect_group_controls<'a>(group: &'a Group, out: &mut Vec<&'a Control>) {
    for c in &group.controls {
        collect_controls(c, out);
    }
    for g in &group.groups {
        collect_group_controls(g, out);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_control_locates_a_nested_control_by_id() {
        let catalog = crate::load_catalog("nist-800-171").unwrap();
        let control = catalog.find_control("SP_800_171_03.01.01").unwrap();
        assert_eq!(control.id, "SP_800_171_03.01.01");
        assert!(catalog.find_control("does-not-exist").is_none());
    }
}
