// @generated — do not edit by hand.
// Run `cargo run -p oscal-metaschema-gen` to regenerate.

//!Generated OSCAL types for the `component` model.
#![allow(clippy::doc_markdown)]
use serde::{Deserialize, Serialize};
use super::common::*;
/// Error returned when a required field was not set before calling `build()`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BuildError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),
}
///A collection of component descriptions, which may optionally be grouped by capability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ComponentDefinition {
    ///Provides a globally unique means to identify a given component definition instance.
    pub uuid: uuid::Uuid,
    ///
    pub metadata: Metadata,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import_component_definitions: Vec<ImportComponentDefinition>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<DefinedComponent>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
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
pub struct ComponentDefinitionBuilder {
    uuid: Option<uuid::Uuid>,
    metadata: Option<Metadata>,
    import_component_definitions: Vec<ImportComponentDefinition>,
    components: Vec<DefinedComponent>,
    capabilities: Vec<Capability>,
    back_matter: Option<BackMatter>,
}
impl ComponentDefinitionBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            metadata: None,
            import_component_definitions: Vec::new(),
            components: Vec::new(),
            capabilities: Vec::new(),
            back_matter: None,
        }
    }
}
impl Default for ComponentDefinitionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ComponentDefinitionBuilder {
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
    ///Set the `import-component-definitions` field.
    pub fn import_component_definitions(
        mut self,
        v: impl Into<ImportComponentDefinition>,
    ) -> Self {
        self.import_component_definitions.push(v.into());
        self
    }
    ///Set the `components` field.
    pub fn components(mut self, v: impl Into<DefinedComponent>) -> Self {
        self.components.push(v.into());
        self
    }
    ///Set the `capabilities` field.
    pub fn capabilities(mut self, v: impl Into<Capability>) -> Self {
        self.capabilities.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<ComponentDefinition, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let metadata = self
            .metadata
            .ok_or_else(|| BuildError::MissingField(
                "required field `metadata` not set",
            ))?;
        Ok(ComponentDefinition {
            uuid,
            metadata,
            import_component_definitions: self.import_component_definitions,
            components: self.components,
            capabilities: self.capabilities,
            back_matter: self.back_matter,
        })
    }
}
impl ComponentDefinition {
    /// Return a new builder for this type.
    pub fn builder() -> ComponentDefinitionBuilder {
        ComponentDefinitionBuilder::new()
    }
}
///Loads a component definition from another resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportComponentDefinition {
    ///A link to a resource that defines a set of components and/or capabilities to import into this collection.
    pub href: crate::primitives::UriReference,
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
pub struct ImportComponentDefinitionBuilder {
    href: Option<crate::primitives::UriReference>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ImportComponentDefinitionBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self { href: None, remarks: None }
    }
}
impl Default for ImportComponentDefinitionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ImportComponentDefinitionBuilder {
    ///Set the `href` field.
    pub fn href(mut self, v: impl Into<crate::primitives::UriReference>) -> Self {
        self.href = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<ImportComponentDefinition, BuildError> {
        let href = self
            .href
            .ok_or_else(|| BuildError::MissingField("required field `href` not set"))?;
        Ok(ImportComponentDefinition {
            href,
            remarks: self.remarks,
        })
    }
}
impl ImportComponentDefinition {
    /// Return a new builder for this type.
    pub fn builder() -> ImportComponentDefinitionBuilder {
        ImportComponentDefinitionBuilder::new()
    }
}
///A defined component that can be part of an implemented system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DefinedComponent {
    ///Provides a globally unique means to identify a given component.
    pub uuid: uuid::Uuid,
    ///A human readable name for the component.
    pub title: crate::primitives::MarkupLine,
    ///A description of the component, including information about its function.
    pub description: crate::primitives::MarkupMultiline,
    ///A summary of the technological or business purpose of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<crate::primitives::MarkupLine>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_roles: Vec<ResponsibleRole>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocols: Vec<Protocol>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub control_implementations: Vec<ControlImplementation>,
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
pub struct DefinedComponentBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    purpose: Option<crate::primitives::MarkupLine>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_roles: Vec<ResponsibleRole>,
    protocols: Vec<Protocol>,
    control_implementations: Vec<ControlImplementation>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl DefinedComponentBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            purpose: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_roles: Vec::new(),
            protocols: Vec::new(),
            control_implementations: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for DefinedComponentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl DefinedComponentBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(
        mut self,
        v: impl Into<crate::primitives::MarkupMultiline>,
    ) -> Self {
        self.description = Some(v.into());
        self
    }
    ///Set the `purpose` field.
    pub fn purpose(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.purpose = Some(v.into());
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
    ///Set the `responsible-roles` field.
    pub fn responsible_roles(mut self, v: impl Into<ResponsibleRole>) -> Self {
        self.responsible_roles.push(v.into());
        self
    }
    ///Set the `protocols` field.
    pub fn protocols(mut self, v: impl Into<Protocol>) -> Self {
        self.protocols.push(v.into());
        self
    }
    ///Set the `control-implementations` field.
    pub fn control_implementations(
        mut self,
        v: impl Into<ControlImplementation>,
    ) -> Self {
        self.control_implementations.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<DefinedComponent, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
        Ok(DefinedComponent {
            uuid,
            title,
            description,
            purpose: self.purpose,
            props: self.props,
            links: self.links,
            responsible_roles: self.responsible_roles,
            protocols: self.protocols,
            control_implementations: self.control_implementations,
            remarks: self.remarks,
        })
    }
}
impl DefinedComponent {
    /// Return a new builder for this type.
    pub fn builder() -> DefinedComponentBuilder {
        DefinedComponentBuilder::new()
    }
}
///A grouping of other components and/or capabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Capability {
    ///Provides a globally unique means to identify a given capability.
    pub uuid: uuid::Uuid,
    ///The capability's human-readable name.
    pub name: String,
    ///A summary of the capability.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub incorporates_components: Vec<IncorporatesComponent>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub control_implementations: Vec<ControlImplementation>,
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
pub struct CapabilityBuilder {
    uuid: Option<uuid::Uuid>,
    name: Option<String>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    incorporates_components: Vec<IncorporatesComponent>,
    control_implementations: Vec<ControlImplementation>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl CapabilityBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            name: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            incorporates_components: Vec::new(),
            control_implementations: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for CapabilityBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl CapabilityBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `name` field.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(
        mut self,
        v: impl Into<crate::primitives::MarkupMultiline>,
    ) -> Self {
        self.description = Some(v.into());
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
    ///Set the `incorporates-components` field.
    pub fn incorporates_components(
        mut self,
        v: impl Into<IncorporatesComponent>,
    ) -> Self {
        self.incorporates_components.push(v.into());
        self
    }
    ///Set the `control-implementations` field.
    pub fn control_implementations(
        mut self,
        v: impl Into<ControlImplementation>,
    ) -> Self {
        self.control_implementations.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Capability, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let name = self
            .name
            .ok_or_else(|| BuildError::MissingField("required field `name` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
        Ok(Capability {
            uuid,
            name,
            description,
            props: self.props,
            links: self.links,
            incorporates_components: self.incorporates_components,
            control_implementations: self.control_implementations,
            remarks: self.remarks,
        })
    }
}
impl Capability {
    /// Return a new builder for this type.
    pub fn builder() -> CapabilityBuilder {
        CapabilityBuilder::new()
    }
}
///The collection of components comprising this capability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IncorporatesComponent {
    ///Amachine-orientedidentifier reference to acomponent.
    pub component_uuid: uuid::Uuid,
    ///A description of the component, including information about its function.
    pub description: crate::primitives::MarkupMultiline,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct IncorporatesComponentBuilder {
    component_uuid: Option<uuid::Uuid>,
    description: Option<crate::primitives::MarkupMultiline>,
}
impl IncorporatesComponentBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            component_uuid: None,
            description: None,
        }
    }
}
impl Default for IncorporatesComponentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl IncorporatesComponentBuilder {
    ///Set the `component-uuid` field.
    pub fn component_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.component_uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(
        mut self,
        v: impl Into<crate::primitives::MarkupMultiline>,
    ) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<IncorporatesComponent, BuildError> {
        let component_uuid = self
            .component_uuid
            .ok_or_else(|| BuildError::MissingField(
                "required field `component-uuid` not set",
            ))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
        Ok(IncorporatesComponent {
            component_uuid,
            description,
        })
    }
}
impl IncorporatesComponent {
    /// Return a new builder for this type.
    pub fn builder() -> IncorporatesComponentBuilder {
        IncorporatesComponentBuilder::new()
    }
}
///Defines how the component or capability supports a set of controls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlImplementation {
    ///Provides a means to identify a set of control implementations that are supported by a given component or capability.
    pub uuid: uuid::Uuid,
    ///A reference to an OSCAL catalog or profile providing the referenced control or subcontrol definition.
    pub source: crate::primitives::UriReference,
    ///A description of how the specified set of controls are implemented for the containing component or capability.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub set_parameters: Vec<SetParameter>,
    ///
    #[serde(default)]
    pub implemented_requirements: Vec<ImplementedRequirement>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct ControlImplementationBuilder {
    uuid: Option<uuid::Uuid>,
    source: Option<crate::primitives::UriReference>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    set_parameters: Vec<SetParameter>,
    implemented_requirements: Vec<ImplementedRequirement>,
}
impl ControlImplementationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            source: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            set_parameters: Vec::new(),
            implemented_requirements: Vec::new(),
        }
    }
}
impl Default for ControlImplementationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ControlImplementationBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `source` field.
    pub fn source(mut self, v: impl Into<crate::primitives::UriReference>) -> Self {
        self.source = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(
        mut self,
        v: impl Into<crate::primitives::MarkupMultiline>,
    ) -> Self {
        self.description = Some(v.into());
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
    ///Set the `set-parameters` field.
    pub fn set_parameters(mut self, v: impl Into<SetParameter>) -> Self {
        self.set_parameters.push(v.into());
        self
    }
    ///Set the `implemented-requirements` field.
    pub fn implemented_requirements(
        mut self,
        v: impl Into<ImplementedRequirement>,
    ) -> Self {
        self.implemented_requirements.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<ControlImplementation, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let source = self
            .source
            .ok_or_else(|| BuildError::MissingField("required field `source` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
        Ok(ControlImplementation {
            uuid,
            source,
            description,
            props: self.props,
            links: self.links,
            set_parameters: self.set_parameters,
            implemented_requirements: self.implemented_requirements,
        })
    }
}
impl ControlImplementation {
    /// Return a new builder for this type.
    pub fn builder() -> ControlImplementationBuilder {
        ControlImplementationBuilder::new()
    }
}
///Describes how the containing component or capability implements an individual control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementedRequirement {
    ///Provides a globally unique means to identify a given control implementation by a component.
    pub uuid: uuid::Uuid,
    ///
    pub control_id: String,
    ///A suggestion from the supplier (e.g., component vendor or author) for how the specified control may be implemented if the containing component or capability is instantiated in a system security plan.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub set_parameters: Vec<SetParameter>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_roles: Vec<ResponsibleRole>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statements: Vec<Statement>,
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
pub struct ImplementedRequirementBuilder {
    uuid: Option<uuid::Uuid>,
    control_id: Option<String>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    set_parameters: Vec<SetParameter>,
    responsible_roles: Vec<ResponsibleRole>,
    statements: Vec<Statement>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ImplementedRequirementBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            control_id: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            set_parameters: Vec::new(),
            responsible_roles: Vec::new(),
            statements: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ImplementedRequirementBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ImplementedRequirementBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `control-id` field.
    pub fn control_id(mut self, v: impl Into<String>) -> Self {
        self.control_id = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(
        mut self,
        v: impl Into<crate::primitives::MarkupMultiline>,
    ) -> Self {
        self.description = Some(v.into());
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
    ///Set the `set-parameters` field.
    pub fn set_parameters(mut self, v: impl Into<SetParameter>) -> Self {
        self.set_parameters.push(v.into());
        self
    }
    ///Set the `responsible-roles` field.
    pub fn responsible_roles(mut self, v: impl Into<ResponsibleRole>) -> Self {
        self.responsible_roles.push(v.into());
        self
    }
    ///Set the `statements` field.
    pub fn statements(mut self, v: impl Into<Statement>) -> Self {
        self.statements.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<ImplementedRequirement, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let control_id = self
            .control_id
            .ok_or_else(|| BuildError::MissingField(
                "required field `control-id` not set",
            ))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
        Ok(ImplementedRequirement {
            uuid,
            control_id,
            description,
            props: self.props,
            links: self.links,
            set_parameters: self.set_parameters,
            responsible_roles: self.responsible_roles,
            statements: self.statements,
            remarks: self.remarks,
        })
    }
}
impl ImplementedRequirement {
    /// Return a new builder for this type.
    pub fn builder() -> ImplementedRequirementBuilder {
        ImplementedRequirementBuilder::new()
    }
}
///Identifies which statements within a control are addressed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Statement {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this control statement elsewhere inthis or other OSCAL instances. TheUUIDof thecontrol statementin the source OSCAL instance is sufficient to reference the data item locally or globally (e.g., in an imported OSCAL instance).
    pub uuid: uuid::Uuid,
    ///A summary of how the containing control statement is implemented by the component or capability.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_roles: Vec<ResponsibleRole>,
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
pub struct StatementBuilder {
    uuid: Option<uuid::Uuid>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_roles: Vec<ResponsibleRole>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl StatementBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_roles: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for StatementBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl StatementBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(
        mut self,
        v: impl Into<crate::primitives::MarkupMultiline>,
    ) -> Self {
        self.description = Some(v.into());
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
    ///Set the `responsible-roles` field.
    pub fn responsible_roles(mut self, v: impl Into<ResponsibleRole>) -> Self {
        self.responsible_roles.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Statement, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
        Ok(Statement {
            uuid,
            description,
            props: self.props,
            links: self.links,
            responsible_roles: self.responsible_roles,
            remarks: self.remarks,
        })
    }
}
impl Statement {
    /// Return a new builder for this type.
    pub fn builder() -> StatementBuilder {
        StatementBuilder::new()
    }
}
