// @generated — do not edit by hand.
// Run `cargo run -p oscal-metaschema-gen` to regenerate.

//!Generated OSCAL types for the `poam` model.
#![allow(clippy::doc_markdown)]
use serde::{Deserialize, Serialize};
use super::common::*;
/// Error returned when a required field was not set before calling `build()`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BuildError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),
}
///A plan of action and milestones which identifies initial and residual risks, deviations, and disposition, such as those required by FedRAMP.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlanOfActionAndMilestones {
    ///Amachine-oriented,globally uniqueidentifier withinstancescope that can be used to reference this POA&M instance inthis OSCAL instance. This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///
    pub metadata: Metadata,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_ssp: Option<ImportSsp>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_id: Option<SystemId>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_definitions: Option<LocalDefinitions>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observations: Vec<Observation>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub risks: Vec<Risk>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<Finding>,
    ///
    #[serde(default)]
    pub poam_items: Vec<PoamItem>,
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
pub struct PlanOfActionAndMilestonesBuilder {
    uuid: Option<uuid::Uuid>,
    metadata: Option<Metadata>,
    import_ssp: Option<ImportSsp>,
    system_id: Option<SystemId>,
    local_definitions: Option<LocalDefinitions>,
    observations: Vec<Observation>,
    risks: Vec<Risk>,
    findings: Vec<Finding>,
    poam_items: Vec<PoamItem>,
    back_matter: Option<BackMatter>,
}
impl PlanOfActionAndMilestonesBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            metadata: None,
            import_ssp: None,
            system_id: None,
            local_definitions: None,
            observations: Vec::new(),
            risks: Vec::new(),
            findings: Vec::new(),
            poam_items: Vec::new(),
            back_matter: None,
        }
    }
}
impl Default for PlanOfActionAndMilestonesBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl PlanOfActionAndMilestonesBuilder {
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
    ///Set the `import-ssp` field.
    pub fn import_ssp(mut self, v: impl Into<ImportSsp>) -> Self {
        self.import_ssp = Some(v.into());
        self
    }
    ///Set the `system-id` field.
    pub fn system_id(mut self, v: impl Into<SystemId>) -> Self {
        self.system_id = Some(v.into());
        self
    }
    ///Set the `local-definitions` field.
    pub fn local_definitions(mut self, v: impl Into<LocalDefinitions>) -> Self {
        self.local_definitions = Some(v.into());
        self
    }
    ///Set the `observations` field.
    pub fn observations(mut self, v: impl Into<Observation>) -> Self {
        self.observations.push(v.into());
        self
    }
    ///Set the `risks` field.
    pub fn risks(mut self, v: impl Into<Risk>) -> Self {
        self.risks.push(v.into());
        self
    }
    ///Set the `findings` field.
    pub fn findings(mut self, v: impl Into<Finding>) -> Self {
        self.findings.push(v.into());
        self
    }
    ///Set the `poam-items` field.
    pub fn poam_items(mut self, v: impl Into<PoamItem>) -> Self {
        self.poam_items.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<PlanOfActionAndMilestones, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let metadata = self
            .metadata
            .ok_or_else(|| BuildError::MissingField(
                "required field `metadata` not set",
            ))?;
        Ok(PlanOfActionAndMilestones {
            uuid,
            metadata,
            import_ssp: self.import_ssp,
            system_id: self.system_id,
            local_definitions: self.local_definitions,
            observations: self.observations,
            risks: self.risks,
            findings: self.findings,
            poam_items: self.poam_items,
            back_matter: self.back_matter,
        })
    }
}
impl PlanOfActionAndMilestones {
    /// Return a new builder for this type.
    pub fn builder() -> PlanOfActionAndMilestonesBuilder {
        PlanOfActionAndMilestonesBuilder::new()
    }
}
///Allows components, and inventory-items to be defined within the POA&M for circumstances where no OSCAL-based SSP exists, or is not delivered with the POA&M.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalDefinitions {
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<SystemComponent>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inventory_items: Vec<InventoryItem>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_assets: Option<AssessmentAssets>,
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
pub struct LocalDefinitionsBuilder {
    components: Vec<SystemComponent>,
    inventory_items: Vec<InventoryItem>,
    assessment_assets: Option<AssessmentAssets>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl LocalDefinitionsBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            inventory_items: Vec::new(),
            assessment_assets: None,
            remarks: None,
        }
    }
}
impl Default for LocalDefinitionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl LocalDefinitionsBuilder {
    ///Set the `components` field.
    pub fn components(mut self, v: impl Into<SystemComponent>) -> Self {
        self.components.push(v.into());
        self
    }
    ///Set the `inventory-items` field.
    pub fn inventory_items(mut self, v: impl Into<InventoryItem>) -> Self {
        self.inventory_items.push(v.into());
        self
    }
    ///Set the `assessment-assets` field.
    pub fn assessment_assets(mut self, v: impl Into<AssessmentAssets>) -> Self {
        self.assessment_assets = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<LocalDefinitions, BuildError> {
        Ok(LocalDefinitions {
            components: self.components,
            inventory_items: self.inventory_items,
            assessment_assets: self.assessment_assets,
            remarks: self.remarks,
        })
    }
}
impl LocalDefinitions {
    /// Return a new builder for this type.
    pub fn builder() -> LocalDefinitionsBuilder {
        LocalDefinitionsBuilder::new()
    }
}
///Identifies the source of the finding, such as a tool or person.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Origin {
    ///
    #[serde(default)]
    pub actors: Vec<OriginActor>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct OriginBuilder {
    actors: Vec<OriginActor>,
}
impl OriginBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self { actors: Vec::new() }
    }
}
impl Default for OriginBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl OriginBuilder {
    ///Set the `actors` field.
    pub fn actors(mut self, v: impl Into<OriginActor>) -> Self {
        self.actors.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Origin, BuildError> {
        Ok(Origin { actors: self.actors })
    }
}
impl Origin {
    /// Return a new builder for this type.
    pub fn builder() -> OriginBuilder {
        OriginBuilder::new()
    }
}
///Relates the poam-item to referenced finding(s).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedFinding {
    ///Amachine-orientedidentifier reference to a finding defined in the list of findings.
    pub finding_uuid: uuid::Uuid,
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
pub struct RelatedFindingBuilder {
    finding_uuid: Option<uuid::Uuid>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl RelatedFindingBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            finding_uuid: None,
            remarks: None,
        }
    }
}
impl Default for RelatedFindingBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RelatedFindingBuilder {
    ///Set the `finding-uuid` field.
    pub fn finding_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.finding_uuid = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<RelatedFinding, BuildError> {
        let finding_uuid = self
            .finding_uuid
            .ok_or_else(|| BuildError::MissingField(
                "required field `finding-uuid` not set",
            ))?;
        Ok(RelatedFinding {
            finding_uuid,
            remarks: self.remarks,
        })
    }
}
impl RelatedFinding {
    /// Return a new builder for this type.
    pub fn builder() -> RelatedFindingBuilder {
        RelatedFindingBuilder::new()
    }
}
///Describes an individual POA&M item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PoamItem {
    ///Amachine-oriented,globally uniqueidentifier withinstancescope that can be used to reference this POA&M item entry inthis OSCAL instance. This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    ///The title or name for this POA&M item .
    pub title: crate::primitives::MarkupLine,
    ///A human-readable description of POA&M item.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///Identifies the source of the finding, such as a tool or person.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<Origin>,
    ///Relates the poam-item to referenced finding(s).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_finding: Vec<RelatedFinding>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_observations: Vec<RelatedObservation>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_risks: Vec<AssociatedRisk>,
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
pub struct PoamItemBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    origin: Vec<Origin>,
    related_finding: Vec<RelatedFinding>,
    related_observations: Vec<RelatedObservation>,
    related_risks: Vec<AssociatedRisk>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl PoamItemBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            origin: Vec::new(),
            related_finding: Vec::new(),
            related_observations: Vec::new(),
            related_risks: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for PoamItemBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl PoamItemBuilder {
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
    ///Set the `origin` field.
    pub fn origin(mut self, v: impl Into<Origin>) -> Self {
        self.origin.push(v.into());
        self
    }
    ///Set the `related-finding` field.
    pub fn related_finding(mut self, v: impl Into<RelatedFinding>) -> Self {
        self.related_finding.push(v.into());
        self
    }
    ///Set the `related-observations` field.
    pub fn related_observations(mut self, v: impl Into<RelatedObservation>) -> Self {
        self.related_observations.push(v.into());
        self
    }
    ///Set the `related-risks` field.
    pub fn related_risks(mut self, v: impl Into<AssociatedRisk>) -> Self {
        self.related_risks.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<PoamItem, BuildError> {
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
        Ok(PoamItem {
            uuid: self.uuid,
            title,
            description,
            props: self.props,
            links: self.links,
            origin: self.origin,
            related_finding: self.related_finding,
            related_observations: self.related_observations,
            related_risks: self.related_risks,
            remarks: self.remarks,
        })
    }
}
impl PoamItem {
    /// Return a new builder for this type.
    pub fn builder() -> PoamItemBuilder {
        PoamItemBuilder::new()
    }
}
