// @generated — do not edit by hand.
// Run `cargo run -p oscal-metaschema-gen` to regenerate.

//!Generated OSCAL types for the `profile` model.
#![allow(clippy::doc_markdown)]
use serde::{Deserialize, Serialize};
use super::common::*;
/// Error returned when a required field was not set before calling `build()`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BuildError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),
}
///Each OSCAL profile is defined by aprofileelement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    ///Provides a globally unique means to identify a given profile instance.
    pub uuid: uuid::Uuid,
    ///
    pub metadata: Metadata,
    ///
    #[serde(default)]
    pub imports: Vec<Import>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<Merge>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify: Option<Modify>,
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
pub struct ProfileBuilder {
    uuid: Option<uuid::Uuid>,
    metadata: Option<Metadata>,
    imports: Vec<Import>,
    merge: Option<Merge>,
    modify: Option<Modify>,
    back_matter: Option<BackMatter>,
}
impl ProfileBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            metadata: None,
            imports: Vec::new(),
            merge: None,
            modify: None,
            back_matter: None,
        }
    }
}
impl Default for ProfileBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ProfileBuilder {
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
    ///Set the `imports` field.
    pub fn imports(mut self, v: impl Into<Import>) -> Self {
        self.imports.push(v.into());
        self
    }
    ///Set the `merge` field.
    pub fn merge(mut self, v: impl Into<Merge>) -> Self {
        self.merge = Some(v.into());
        self
    }
    ///Set the `modify` field.
    pub fn modify(mut self, v: impl Into<Modify>) -> Self {
        self.modify = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Profile, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let metadata = self
            .metadata
            .ok_or_else(|| BuildError::MissingField(
                "required field `metadata` not set",
            ))?;
        Ok(Profile {
            uuid,
            metadata,
            imports: self.imports,
            merge: self.merge,
            modify: self.modify,
            back_matter: self.back_matter,
        })
    }
}
impl Profile {
    /// Return a new builder for this type.
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportChoice1 {
    IncludeAll(IncludeAll),
    SelectControlById(Vec<SelectControlById>),
}
///Designates a referenced source catalog or profile that provides a source of control information for use in creating a new overlay or baseline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Import {
    ///A resolvable URL reference to the base catalog or profile that this profile is tailoring.
    pub href: crate::primitives::UriReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_choice1: Option<ImportChoice1>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_controls: Vec<SelectControlById>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct ImportBuilder {
    href: Option<crate::primitives::UriReference>,
    import_choice1: Option<ImportChoice1>,
    exclude_controls: Vec<SelectControlById>,
}
impl ImportBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            href: None,
            import_choice1: None,
            exclude_controls: Vec::new(),
        }
    }
}
impl Default for ImportBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ImportBuilder {
    ///Set the `href` field.
    pub fn href(mut self, v: impl Into<crate::primitives::UriReference>) -> Self {
        self.href = Some(v.into());
        self
    }
    ///Set the `import_choice1` field.
    pub fn import_choice1(mut self, v: impl Into<ImportChoice1>) -> Self {
        self.import_choice1 = Some(v.into());
        self
    }
    ///Set the `exclude-controls` field.
    pub fn exclude_controls(mut self, v: impl Into<SelectControlById>) -> Self {
        self.exclude_controls.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Import, BuildError> {
        let href = self
            .href
            .ok_or_else(|| BuildError::MissingField("required field `href` not set"))?;
        Ok(Import {
            href,
            import_choice1: self.import_choice1,
            exclude_controls: self.exclude_controls,
        })
    }
}
impl Import {
    /// Return a new builder for this type.
    pub fn builder() -> ImportBuilder {
        ImportBuilder::new()
    }
}
///A Combine element defines how to resolve duplicate instances of the same control (e.g., controls with the same ID).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Combine {
    ///Declare how clashing controls should be handled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct CombineBuilder {
    method: Option<String>,
}
impl CombineBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self { method: None }
    }
}
impl Default for CombineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl CombineBuilder {
    ///Set the `method` field.
    pub fn method(mut self, v: impl Into<String>) -> Self {
        self.method = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Combine, BuildError> {
        Ok(Combine { method: self.method })
    }
}
impl Combine {
    /// Return a new builder for this type.
    pub fn builder() -> CombineBuilder {
        CombineBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MergeChoice1 {}
///Provides structuring directives that instruct how controls are organized after profile resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Merge {
    ///A Combine element defines how to resolve duplicate instances of the same control (e.g., controls with the same ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combine: Option<Combine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_choice1: Option<MergeChoice1>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct MergeBuilder {
    combine: Option<Combine>,
    merge_choice1: Option<MergeChoice1>,
}
impl MergeBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            combine: None,
            merge_choice1: None,
        }
    }
}
impl Default for MergeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl MergeBuilder {
    ///Set the `combine` field.
    pub fn combine(mut self, v: impl Into<Combine>) -> Self {
        self.combine = Some(v.into());
        self
    }
    ///Set the `merge_choice1` field.
    pub fn merge_choice1(mut self, v: impl Into<MergeChoice1>) -> Self {
        self.merge_choice1 = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Merge, BuildError> {
        Ok(Merge {
            combine: self.combine,
            merge_choice1: self.merge_choice1,
        })
    }
}
impl Merge {
    /// Return a new builder for this type.
    pub fn builder() -> MergeBuilder {
        MergeBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetParameterChoice1 {
    ParameterValue(Vec<String>),
    ParameterSelection(Option<ParameterSelection>),
}
///A parameter setting, to be propagated to points of insertion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SetParameter {
    ///An identifier for the parameter.
    pub param_id: String,
    ///A textual label that provides a characterization of the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    ///**(deprecated)** Another parameter invoking this one. This construct has been deprecated and should not be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<String>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///A short, placeholder name for the parameter, which can be used as a substitute for avalueif no value is assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<crate::primitives::MarkupLine>,
    ///Describes the purpose and use of a parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<ParameterConstraint>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guidelines: Vec<ParameterGuideline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_parameter_choice1: Option<SetParameterChoice1>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct SetParameterBuilder {
    param_id: Option<String>,
    class: Option<String>,
    depends_on: Option<String>,
    props: Vec<Property>,
    links: Vec<Link>,
    label: Option<crate::primitives::MarkupLine>,
    usage: Option<crate::primitives::MarkupMultiline>,
    constraints: Vec<ParameterConstraint>,
    guidelines: Vec<ParameterGuideline>,
    set_parameter_choice1: Option<SetParameterChoice1>,
}
impl SetParameterBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            param_id: None,
            class: None,
            depends_on: None,
            props: Vec::new(),
            links: Vec::new(),
            label: None,
            usage: None,
            constraints: Vec::new(),
            guidelines: Vec::new(),
            set_parameter_choice1: None,
        }
    }
}
impl Default for SetParameterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SetParameterBuilder {
    ///Set the `param-id` field.
    pub fn param_id(mut self, v: impl Into<String>) -> Self {
        self.param_id = Some(v.into());
        self
    }
    ///Set the `class` field.
    pub fn class(mut self, v: impl Into<String>) -> Self {
        self.class = Some(v.into());
        self
    }
    ///Set the `depends-on` field.
    pub fn depends_on(mut self, v: impl Into<String>) -> Self {
        self.depends_on = Some(v.into());
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
    ///Set the `label` field.
    pub fn label(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.label = Some(v.into());
        self
    }
    ///Set the `usage` field.
    pub fn usage(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.usage = Some(v.into());
        self
    }
    ///Set the `constraints` field.
    pub fn constraints(mut self, v: impl Into<ParameterConstraint>) -> Self {
        self.constraints.push(v.into());
        self
    }
    ///Set the `guidelines` field.
    pub fn guidelines(mut self, v: impl Into<ParameterGuideline>) -> Self {
        self.guidelines.push(v.into());
        self
    }
    ///Set the `set_parameter_choice1` field.
    pub fn set_parameter_choice1(mut self, v: impl Into<SetParameterChoice1>) -> Self {
        self.set_parameter_choice1 = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<SetParameter, BuildError> {
        let param_id = self
            .param_id
            .ok_or_else(|| BuildError::MissingField(
                "required field `param-id` not set",
            ))?;
        Ok(SetParameter {
            param_id,
            class: self.class,
            depends_on: self.depends_on,
            props: self.props,
            links: self.links,
            label: self.label,
            usage: self.usage,
            constraints: self.constraints,
            guidelines: self.guidelines,
            set_parameter_choice1: self.set_parameter_choice1,
        })
    }
}
impl SetParameter {
    /// Return a new builder for this type.
    pub fn builder() -> SetParameterBuilder {
        SetParameterBuilder::new()
    }
}
///Specifies objects to be removed from a control based on specific aspects of the object that must all match.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Remove {
    ///Identify items remove by matching their assigned name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_name: Option<String>,
    ///Identify items to remove by matching theirclass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_class: Option<String>,
    ///Identify items to remove indicated by theirid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_id: Option<String>,
    ///Identify items to remove by the name of the item's information object name, e.g.titleorprop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_item_name: Option<String>,
    ///Identify items to remove by the item'sns, which is the namespace associated with apart, orprop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_ns: Option<url::Url>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<crate::primitives::MarkupMultiline>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct RemoveBuilder {
    by_name: Option<String>,
    by_class: Option<String>,
    by_id: Option<String>,
    by_item_name: Option<String>,
    by_ns: Option<url::Url>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl RemoveBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            by_name: None,
            by_class: None,
            by_id: None,
            by_item_name: None,
            by_ns: None,
            remarks: None,
        }
    }
}
impl Default for RemoveBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RemoveBuilder {
    ///Set the `by-name` field.
    pub fn by_name(mut self, v: impl Into<String>) -> Self {
        self.by_name = Some(v.into());
        self
    }
    ///Set the `by-class` field.
    pub fn by_class(mut self, v: impl Into<String>) -> Self {
        self.by_class = Some(v.into());
        self
    }
    ///Set the `by-id` field.
    pub fn by_id(mut self, v: impl Into<String>) -> Self {
        self.by_id = Some(v.into());
        self
    }
    ///Set the `by-item-name` field.
    pub fn by_item_name(mut self, v: impl Into<String>) -> Self {
        self.by_item_name = Some(v.into());
        self
    }
    ///Set the `by-ns` field.
    pub fn by_ns(mut self, v: impl Into<url::Url>) -> Self {
        self.by_ns = Some(v.into());
        self
    }
    ///Set the `remarks` field.
    pub fn remarks(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.remarks = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Remove, BuildError> {
        Ok(Remove {
            by_name: self.by_name,
            by_class: self.by_class,
            by_id: self.by_id,
            by_item_name: self.by_item_name,
            by_ns: self.by_ns,
            remarks: self.remarks,
        })
    }
}
impl Remove {
    /// Return a new builder for this type.
    pub fn builder() -> RemoveBuilder {
        RemoveBuilder::new()
    }
}
///Specifies contents to be added into controls, in resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Add {
    ///Where to add the new content with respect to the targeted element (beside it or inside it).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    ///Target location of the addition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_id: Option<String>,
    ///A name given to the control, which may be used by a tool for display and navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
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
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct AddBuilder {
    position: Option<String>,
    by_id: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    params: Vec<Parameter>,
    props: Vec<Property>,
    links: Vec<Link>,
    parts: Vec<Part>,
}
impl AddBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            position: None,
            by_id: None,
            title: None,
            params: Vec::new(),
            props: Vec::new(),
            links: Vec::new(),
            parts: Vec::new(),
        }
    }
}
impl Default for AddBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AddBuilder {
    ///Set the `position` field.
    pub fn position(mut self, v: impl Into<String>) -> Self {
        self.position = Some(v.into());
        self
    }
    ///Set the `by-id` field.
    pub fn by_id(mut self, v: impl Into<String>) -> Self {
        self.by_id = Some(v.into());
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
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Add, BuildError> {
        Ok(Add {
            position: self.position,
            by_id: self.by_id,
            title: self.title,
            params: self.params,
            props: self.props,
            links: self.links,
            parts: self.parts,
        })
    }
}
impl Add {
    /// Return a new builder for this type.
    pub fn builder() -> AddBuilder {
        AddBuilder::new()
    }
}
///Specifies changes to be made to an included control when a profile is resolved.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Alter {
    ///
    pub control_id: String,
    ///Specifies objects to be removed from a control based on specific aspects of the object that must all match.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove: Vec<Remove>,
    ///Specifies contents to be added into controls, in resolution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<Add>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct AlterBuilder {
    control_id: Option<String>,
    remove: Vec<Remove>,
    add: Vec<Add>,
}
impl AlterBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            control_id: None,
            remove: Vec::new(),
            add: Vec::new(),
        }
    }
}
impl Default for AlterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AlterBuilder {
    ///Set the `control-id` field.
    pub fn control_id(mut self, v: impl Into<String>) -> Self {
        self.control_id = Some(v.into());
        self
    }
    ///Set the `remove` field.
    pub fn remove(mut self, v: impl Into<Remove>) -> Self {
        self.remove.push(v.into());
        self
    }
    ///Set the `add` field.
    pub fn add(mut self, v: impl Into<Add>) -> Self {
        self.add.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Alter, BuildError> {
        let control_id = self
            .control_id
            .ok_or_else(|| BuildError::MissingField(
                "required field `control-id` not set",
            ))?;
        Ok(Alter {
            control_id,
            remove: self.remove,
            add: self.add,
        })
    }
}
impl Alter {
    /// Return a new builder for this type.
    pub fn builder() -> AlterBuilder {
        AlterBuilder::new()
    }
}
///Set parameters or amend controls in resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Modify {
    ///A parameter setting, to be propagated to points of insertion.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub set_parameter: Vec<SetParameter>,
    ///Specifies changes to be made to an included control when a profile is resolved.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alter: Vec<Alter>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct ModifyBuilder {
    set_parameter: Vec<SetParameter>,
    alter: Vec<Alter>,
}
impl ModifyBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            set_parameter: Vec::new(),
            alter: Vec::new(),
        }
    }
}
impl Default for ModifyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ModifyBuilder {
    ///Set the `set-parameter` field.
    pub fn set_parameter(mut self, v: impl Into<SetParameter>) -> Self {
        self.set_parameter.push(v.into());
        self
    }
    ///Set the `alter` field.
    pub fn alter(mut self, v: impl Into<Alter>) -> Self {
        self.alter.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Modify, BuildError> {
        Ok(Modify {
            set_parameter: self.set_parameter,
            alter: self.alter,
        })
    }
}
impl Modify {
    /// Return a new builder for this type.
    pub fn builder() -> ModifyBuilder {
        ModifyBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InsertControlsChoice1 {
    IncludeAll(IncludeAll),
    SelectControlById(Vec<SelectControlById>),
}
///Specifies which controls to use in the containing context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InsertControls {
    ///A designation of how a selection of controls in a profile is to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_controls_choice1: Option<InsertControlsChoice1>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_controls: Vec<SelectControlById>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct InsertControlsBuilder {
    order: Option<String>,
    insert_controls_choice1: Option<InsertControlsChoice1>,
    exclude_controls: Vec<SelectControlById>,
}
impl InsertControlsBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            order: None,
            insert_controls_choice1: None,
            exclude_controls: Vec::new(),
        }
    }
}
impl Default for InsertControlsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl InsertControlsBuilder {
    ///Set the `order` field.
    pub fn order(mut self, v: impl Into<String>) -> Self {
        self.order = Some(v.into());
        self
    }
    ///Set the `insert_controls_choice1` field.
    pub fn insert_controls_choice1(
        mut self,
        v: impl Into<InsertControlsChoice1>,
    ) -> Self {
        self.insert_controls_choice1 = Some(v.into());
        self
    }
    ///Set the `exclude-controls` field.
    pub fn exclude_controls(mut self, v: impl Into<SelectControlById>) -> Self {
        self.exclude_controls.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<InsertControls, BuildError> {
        Ok(InsertControls {
            order: self.order,
            insert_controls_choice1: self.insert_controls_choice1,
            exclude_controls: self.exclude_controls,
        })
    }
}
impl InsertControls {
    /// Return a new builder for this type.
    pub fn builder() -> InsertControlsBuilder {
        InsertControlsBuilder::new()
    }
}
