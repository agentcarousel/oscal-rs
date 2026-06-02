// @generated — do not edit by hand.
// Run `cargo run -p oscal-metaschema-gen` to regenerate.

//!Generated OSCAL types for the `catalog` model.
#![allow(clippy::doc_markdown)]
use serde::{Deserialize, Serialize};
use super::common::*;
/// Error returned when a required field was not set before calling `build()`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BuildError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),
}
///A structured,organized collectionof control information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Catalog {
    ///Provides a globally unique means to identify a given catalog instance.
    pub uuid: uuid::Uuid,
    ///
    pub metadata: Metadata,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<Parameter>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub controls: Vec<Control>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Group>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_matter: Option<BackMatter>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct CatalogBuilder {
    uuid: Option<uuid::Uuid>,
    metadata: Option<Metadata>,
    params: Vec<Parameter>,
    controls: Vec<Control>,
    groups: Vec<Group>,
    back_matter: Option<BackMatter>,
}
impl CatalogBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            metadata: None,
            params: Vec::new(),
            controls: Vec::new(),
            groups: Vec::new(),
            back_matter: None,
        }
    }
}
impl Default for CatalogBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl CatalogBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `metadata` field.
    pub fn metadata(mut self, v: impl Into<Metadata>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    ///Set the `params` field.
    pub fn params(mut self, v: impl Into<Parameter>) -> Self {
        self.params.push(v.into());
        self
    }
    ///Set the `controls` field.
    pub fn controls(mut self, v: impl Into<Control>) -> Self {
        self.controls.push(v.into());
        self
    }
    ///Set the `groups` field.
    pub fn groups(mut self, v: impl Into<Group>) -> Self {
        self.groups.push(v.into());
        self
    }
    ///Set the `back-matter` field.
    pub fn back_matter(mut self, v: impl Into<BackMatter>) -> Self {
        self.back_matter = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Catalog, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let metadata = self
            .metadata
            .ok_or_else(|| BuildError::MissingField(
                "required field `metadata` not set",
            ))?;
        Ok(Catalog {
            uuid,
            metadata,
            params: self.params,
            controls: self.controls,
            groups: self.groups,
            back_matter: self.back_matter,
        })
    }
}
impl Catalog {
    /// Return a new builder for this type.
    pub fn builder() -> CatalogBuilder {
        CatalogBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupChoice1 {
    Group(Vec<Group>),
    Control(Vec<Control>),
}
///A group of controls, or of groups of controls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Group {
    ///Identifies the group for the purpose of cross-linking within the defining instance or from other instances that reference the catalog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A textual label that provides a sub-type or characterization of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    ///A name given to the group, which may be used by a tool for display and navigation.
    pub title: crate::primitives::MarkupLine,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<Parameter>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_choice1: Option<GroupChoice1>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct GroupBuilder {
    id: Option<String>,
    class: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    params: Vec<Parameter>,
    props: Vec<Property>,
    links: Vec<Link>,
    parts: Vec<Part>,
    group_choice1: Option<GroupChoice1>,
}
impl GroupBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            id: None,
            class: None,
            title: None,
            params: Vec::new(),
            props: Vec::new(),
            links: Vec::new(),
            parts: Vec::new(),
            group_choice1: None,
        }
    }
}
impl Default for GroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl GroupBuilder {
    ///Set the `id` field.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    ///Set the `class` field.
    pub fn class(mut self, v: impl Into<String>) -> Self {
        self.class = Some(v.into());
        self
    }
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `params` field.
    pub fn params(mut self, v: impl Into<Parameter>) -> Self {
        self.params.push(v.into());
        self
    }
    ///Set the `props` field.
    pub fn props(mut self, v: impl Into<Property>) -> Self {
        self.props.push(v.into());
        self
    }
    ///Set the `links` field.
    pub fn links(mut self, v: impl Into<Link>) -> Self {
        self.links.push(v.into());
        self
    }
    ///Set the `parts` field.
    pub fn parts(mut self, v: impl Into<Part>) -> Self {
        self.parts.push(v.into());
        self
    }
    ///Set the `group_choice1` field.
    pub fn group_choice1(mut self, v: impl Into<GroupChoice1>) -> Self {
        self.group_choice1 = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Group, BuildError> {
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        Ok(Group {
            id: self.id,
            class: self.class,
            title,
            params: self.params,
            props: self.props,
            links: self.links,
            parts: self.parts,
            group_choice1: self.group_choice1,
        })
    }
}
impl Group {
    /// Return a new builder for this type.
    pub fn builder() -> GroupBuilder {
        GroupBuilder::new()
    }
}
///Astructured objectrepresenting a requirement or guideline, which when implemented will reduce an aspect of risk related to an information system and its information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Control {
    ///Identifies a control such that it can be referenced in the defining catalog and other OSCAL instances (e.g., profiles).
    pub id: String,
    ///A textual label that provides a sub-type or characterization of the control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    ///A name given to the control, which may be used by a tool for display and navigation.
    pub title: crate::primitives::MarkupLine,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<Parameter>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<Part>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub controls: Vec<Control>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct ControlBuilder {
    id: Option<String>,
    class: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    params: Vec<Parameter>,
    props: Vec<Property>,
    links: Vec<Link>,
    parts: Vec<Part>,
    controls: Vec<Control>,
}
impl ControlBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            id: None,
            class: None,
            title: None,
            params: Vec::new(),
            props: Vec::new(),
            links: Vec::new(),
            parts: Vec::new(),
            controls: Vec::new(),
        }
    }
}
impl Default for ControlBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ControlBuilder {
    ///Set the `id` field.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    ///Set the `class` field.
    pub fn class(mut self, v: impl Into<String>) -> Self {
        self.class = Some(v.into());
        self
    }
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `params` field.
    pub fn params(mut self, v: impl Into<Parameter>) -> Self {
        self.params.push(v.into());
        self
    }
    ///Set the `props` field.
    pub fn props(mut self, v: impl Into<Property>) -> Self {
        self.props.push(v.into());
        self
    }
    ///Set the `links` field.
    pub fn links(mut self, v: impl Into<Link>) -> Self {
        self.links.push(v.into());
        self
    }
    ///Set the `parts` field.
    pub fn parts(mut self, v: impl Into<Part>) -> Self {
        self.parts.push(v.into());
        self
    }
    ///Set the `controls` field.
    pub fn controls(mut self, v: impl Into<Control>) -> Self {
        self.controls.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Control, BuildError> {
        let id = self
            .id
            .ok_or_else(|| BuildError::MissingField("required field `id` not set"))?;
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        Ok(Control {
            id,
            class: self.class,
            title,
            params: self.params,
            props: self.props,
            links: self.links,
            parts: self.parts,
            controls: self.controls,
        })
    }
}
impl Control {
    /// Return a new builder for this type.
    pub fn builder() -> ControlBuilder {
        ControlBuilder::new()
    }
}
