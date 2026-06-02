// @generated — do not edit by hand.
// Run `cargo run -p oscal-metaschema-gen` to regenerate.

//!Generated OSCAL types for the `assessment_plan` model.
#![allow(clippy::doc_markdown)]
use serde::{Deserialize, Serialize};
use super::common::*;
/// Error returned when a required field was not set before calling `build()`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BuildError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),
}
///Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP.
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<SystemUser>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objectives_and_methods: Vec<LocalObjective>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activities: Vec<Activity>,
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
    users: Vec<SystemUser>,
    objectives_and_methods: Vec<LocalObjective>,
    activities: Vec<Activity>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl LocalDefinitionsBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            inventory_items: Vec::new(),
            users: Vec::new(),
            objectives_and_methods: Vec::new(),
            activities: Vec::new(),
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
    ///Set the `users` field.
    pub fn users(mut self, v: impl Into<SystemUser>) -> Self {
        self.users.push(v.into());
        self
    }
    ///Set the `objectives-and-methods` field.
    pub fn objectives_and_methods(mut self, v: impl Into<LocalObjective>) -> Self {
        self.objectives_and_methods.push(v.into());
        self
    }
    ///Set the `activities` field.
    pub fn activities(mut self, v: impl Into<Activity>) -> Self {
        self.activities.push(v.into());
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
            users: self.users,
            objectives_and_methods: self.objectives_and_methods,
            activities: self.activities,
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
///Used to define various terms and conditions under which an assessment, described by the plan, can be performed. Each child part defines a different type of term or condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TermsAndConditions {
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<AssessmentPart>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct TermsAndConditionsBuilder {
    parts: Vec<AssessmentPart>,
}
impl TermsAndConditionsBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }
}
impl Default for TermsAndConditionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl TermsAndConditionsBuilder {
    ///Set the `parts` field.
    pub fn parts(mut self, v: impl Into<AssessmentPart>) -> Self {
        self.parts.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<TermsAndConditions, BuildError> {
        Ok(TermsAndConditions {
            parts: self.parts,
        })
    }
}
impl TermsAndConditions {
    /// Return a new builder for this type.
    pub fn builder() -> TermsAndConditionsBuilder {
        TermsAndConditionsBuilder::new()
    }
}
///An assessment plan, such as those provided by a FedRAMP assessor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentPlan {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this assessment plan inthis or other OSCAL instances. The locally definedUUIDof theassessment plancan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///
    pub metadata: Metadata,
    ///
    pub import_ssp: ImportSsp,
    ///Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_definitions: Option<LocalDefinitions>,
    ///Used to define various terms and conditions under which an assessment, described by the plan, can be performed. Each child part defines a different type of term or condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_and_conditions: Option<TermsAndConditions>,
    ///
    pub reviewed_controls: ReviewedControls,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assessment_subjects: Vec<AssessmentSubject>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_assets: Option<AssessmentAssets>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<Task>,
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
pub struct AssessmentPlanBuilder {
    uuid: Option<uuid::Uuid>,
    metadata: Option<Metadata>,
    import_ssp: Option<ImportSsp>,
    local_definitions: Option<LocalDefinitions>,
    terms_and_conditions: Option<TermsAndConditions>,
    reviewed_controls: Option<ReviewedControls>,
    assessment_subjects: Vec<AssessmentSubject>,
    assessment_assets: Option<AssessmentAssets>,
    tasks: Vec<Task>,
    back_matter: Option<BackMatter>,
}
impl AssessmentPlanBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            metadata: None,
            import_ssp: None,
            local_definitions: None,
            terms_and_conditions: None,
            reviewed_controls: None,
            assessment_subjects: Vec::new(),
            assessment_assets: None,
            tasks: Vec::new(),
            back_matter: None,
        }
    }
}
impl Default for AssessmentPlanBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssessmentPlanBuilder {
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
    ///Set the `local-definitions` field.
    pub fn local_definitions(mut self, v: impl Into<LocalDefinitions>) -> Self {
        self.local_definitions = Some(v.into());
        self
    }
    ///Set the `terms-and-conditions` field.
    pub fn terms_and_conditions(mut self, v: impl Into<TermsAndConditions>) -> Self {
        self.terms_and_conditions = Some(v.into());
        self
    }
    ///Set the `reviewed-controls` field.
    pub fn reviewed_controls(mut self, v: impl Into<ReviewedControls>) -> Self {
        self.reviewed_controls = Some(v.into());
        self
    }
    ///Set the `assessment-subjects` field.
    pub fn assessment_subjects(mut self, v: impl Into<AssessmentSubject>) -> Self {
        self.assessment_subjects.push(v.into());
        self
    }
    ///Set the `assessment-assets` field.
    pub fn assessment_assets(mut self, v: impl Into<AssessmentAssets>) -> Self {
        self.assessment_assets = Some(v.into());
        self
    }
    ///Set the `tasks` field.
    pub fn tasks(mut self, v: impl Into<Task>) -> Self {
        self.tasks.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<AssessmentPlan, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let metadata = self
            .metadata
            .ok_or_else(|| BuildError::MissingField(
                "required field `metadata` not set",
            ))?;
        let import_ssp = self
            .import_ssp
            .ok_or_else(|| BuildError::MissingField(
                "required field `import-ssp` not set",
            ))?;
        let reviewed_controls = self
            .reviewed_controls
            .ok_or_else(|| BuildError::MissingField(
                "required field `reviewed-controls` not set",
            ))?;
        Ok(AssessmentPlan {
            uuid,
            metadata,
            import_ssp,
            local_definitions: self.local_definitions,
            terms_and_conditions: self.terms_and_conditions,
            reviewed_controls,
            assessment_subjects: self.assessment_subjects,
            assessment_assets: self.assessment_assets,
            tasks: self.tasks,
            back_matter: self.back_matter,
        })
    }
}
impl AssessmentPlan {
    /// Return a new builder for this type.
    pub fn builder() -> AssessmentPlanBuilder {
        AssessmentPlanBuilder::new()
    }
}
