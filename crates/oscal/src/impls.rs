//! Hand-written method impls for generated OSCAL types.
//!
//! The code generator emits data types and builders; domain-specific traversal
//! methods live here so they survive metaschema regeneration.

use crate::generated::types::{Catalog, Control, Group, GroupChoice1};

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
    if let Some(choice) = &group.group_choice1 {
        match choice {
            GroupChoice1::Group(groups) => {
                for g in groups {
                    collect_group_controls(g, out);
                }
            }
            GroupChoice1::Control(controls) => {
                for c in controls {
                    collect_controls(c, out);
                }
            }
        }
    }
}
