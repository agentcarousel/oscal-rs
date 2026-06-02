// @generated — do not edit by hand.
// Run `cargo run -p oscal-metaschema-gen` to regenerate.

//!Generated OSCAL types for the `assessment_results` model.
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
    objectives_and_methods: Vec<LocalObjective>,
    activities: Vec<Activity>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl LocalDefinitionsBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
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
///Security assessment results, such as those provided by a FedRAMP assessor in the FedRAMP Security Assessment Report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentResults {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this assessment results instance inthis or other OSCAL instances. The locally definedUUIDof theassessment resultcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///
    pub metadata: Metadata,
    ///
    pub import_ap: ImportAp,
    ///Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_definitions: Option<LocalDefinitions>,
    ///
    #[serde(default)]
    pub results: Vec<OscalResult>,
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
pub struct AssessmentResultsBuilder {
    uuid: Option<uuid::Uuid>,
    metadata: Option<Metadata>,
    import_ap: Option<ImportAp>,
    local_definitions: Option<LocalDefinitions>,
    results: Vec<OscalResult>,
    back_matter: Option<BackMatter>,
}
impl AssessmentResultsBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            metadata: None,
            import_ap: None,
            local_definitions: None,
            results: Vec::new(),
            back_matter: None,
        }
    }
}
impl Default for AssessmentResultsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssessmentResultsBuilder {
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
    ///Set the `import-ap` field.
    pub fn import_ap(mut self, v: impl Into<ImportAp>) -> Self {
        self.import_ap = Some(v.into());
        self
    }
    ///Set the `local-definitions` field.
    pub fn local_definitions(mut self, v: impl Into<LocalDefinitions>) -> Self {
        self.local_definitions = Some(v.into());
        self
    }
    ///Set the `results` field.
    pub fn results(mut self, v: impl Into<OscalResult>) -> Self {
        self.results.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<AssessmentResults, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let metadata = self
            .metadata
            .ok_or_else(|| BuildError::MissingField(
                "required field `metadata` not set",
            ))?;
        let import_ap = self
            .import_ap
            .ok_or_else(|| BuildError::MissingField(
                "required field `import-ap` not set",
            ))?;
        Ok(AssessmentResults {
            uuid,
            metadata,
            import_ap,
            local_definitions: self.local_definitions,
            results: self.results,
            back_matter: self.back_matter,
        })
    }
}
impl AssessmentResults {
    /// Return a new builder for this type.
    pub fn builder() -> AssessmentResultsBuilder {
        AssessmentResultsBuilder::new()
    }
}
///A set of textual statements, typically written by the assessor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Attestation {
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_parties: Vec<ResponsibleParty>,
    ///
    #[serde(default)]
    pub parts: Vec<AssessmentPart>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct AttestationBuilder {
    responsible_parties: Vec<ResponsibleParty>,
    parts: Vec<AssessmentPart>,
}
impl AttestationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            responsible_parties: Vec::new(),
            parts: Vec::new(),
        }
    }
}
impl Default for AttestationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AttestationBuilder {
    ///Set the `responsible-parties` field.
    pub fn responsible_parties(mut self, v: impl Into<ResponsibleParty>) -> Self {
        self.responsible_parties.push(v.into());
        self
    }
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
    pub fn build(self) -> ::core::result::Result<Attestation, BuildError> {
        Ok(Attestation {
            responsible_parties: self.responsible_parties,
            parts: self.parts,
        })
    }
}
impl Attestation {
    /// Return a new builder for this type.
    pub fn builder() -> AttestationBuilder {
        AttestationBuilder::new()
    }
}
///Identifies the result of an action and/or task that occurred as part of executing an assessment plan or an assessment event that occurred in producing the assessment results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Entry {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference an assessment event inthis or other OSCAL instances. The locally definedUUIDof theassessment log entrycan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The title for this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///A human-readable description of this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///Identifies the start date and time of an event.
    pub start: chrono::DateTime<chrono::Utc>,
    ///Identifies the end date and time of an event. If the event is a point in time, the start and end will be the same date and time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logged_by: Vec<LoggedBy>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_tasks: Vec<RelatedTask>,
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
pub struct EntryBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    start: Option<chrono::DateTime<chrono::Utc>>,
    end: Option<chrono::DateTime<chrono::Utc>>,
    props: Vec<Property>,
    links: Vec<Link>,
    logged_by: Vec<LoggedBy>,
    related_tasks: Vec<RelatedTask>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl EntryBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            start: None,
            end: None,
            props: Vec::new(),
            links: Vec::new(),
            logged_by: Vec::new(),
            related_tasks: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for EntryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl EntryBuilder {
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
    ///Set the `start` field.
    pub fn start(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.start = Some(v.into());
        self
    }
    ///Set the `end` field.
    pub fn end(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.end = Some(v.into());
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
    ///Set the `logged-by` field.
    pub fn logged_by(mut self, v: impl Into<LoggedBy>) -> Self {
        self.logged_by.push(v.into());
        self
    }
    ///Set the `related-tasks` field.
    pub fn related_tasks(mut self, v: impl Into<RelatedTask>) -> Self {
        self.related_tasks.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Entry, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let start = self
            .start
            .ok_or_else(|| BuildError::MissingField("required field `start` not set"))?;
        Ok(Entry {
            uuid,
            title: self.title,
            description: self.description,
            start,
            end: self.end,
            props: self.props,
            links: self.links,
            logged_by: self.logged_by,
            related_tasks: self.related_tasks,
            remarks: self.remarks,
        })
    }
}
impl Entry {
    /// Return a new builder for this type.
    pub fn builder() -> EntryBuilder {
        EntryBuilder::new()
    }
}
///A log of all assessment-related actions taken.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentLog {
    ///Identifies the result of an action and/or task that occurred as part of executing an assessment plan or an assessment event that occurred in producing the assessment results.
    #[serde(default)]
    pub entry: Vec<Entry>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct AssessmentLogBuilder {
    entry: Vec<Entry>,
}
impl AssessmentLogBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self { entry: Vec::new() }
    }
}
impl Default for AssessmentLogBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssessmentLogBuilder {
    ///Set the `entry` field.
    pub fn entry(mut self, v: impl Into<Entry>) -> Self {
        self.entry.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<AssessmentLog, BuildError> {
        Ok(AssessmentLog { entry: self.entry })
    }
}
impl AssessmentLog {
    /// Return a new builder for this type.
    pub fn builder() -> AssessmentLogBuilder {
        AssessmentLogBuilder::new()
    }
}
///Used by the assessment results and POA&M. In the assessment results, this identifies all of the assessment observations and findings, initial and residual risks, deviations, and disposition. In the POA&M, this identifies initial and residual risks, deviations, and disposition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OscalResult {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this set of results inthis or other OSCAL instances. The locally definedUUIDof theassessment resultcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The title for this set of results.
    pub title: crate::primitives::MarkupLine,
    ///A human-readable description of this set of test results.
    pub description: crate::primitives::MarkupMultiline,
    ///Date/time stamp identifying the start of the evidence collection reflected in these results.
    pub start: chrono::DateTime<chrono::Utc>,
    ///Date/time stamp identifying the end of the evidence collection reflected in these results. In a continuous motoring scenario, this may contain the same value as start if appropriate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_definitions: Option<LocalDefinitions>,
    ///
    pub reviewed_controls: ReviewedControls,
    ///A set of textual statements, typically written by the assessor.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attestation: Vec<Attestation>,
    ///A log of all assessment-related actions taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_log: Option<AssessmentLog>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<crate::primitives::MarkupMultiline>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct OscalResultBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    start: Option<chrono::DateTime<chrono::Utc>>,
    end: Option<chrono::DateTime<chrono::Utc>>,
    props: Vec<Property>,
    links: Vec<Link>,
    local_definitions: Option<LocalDefinitions>,
    reviewed_controls: Option<ReviewedControls>,
    attestation: Vec<Attestation>,
    assessment_log: Option<AssessmentLog>,
    observations: Vec<Observation>,
    risks: Vec<Risk>,
    findings: Vec<Finding>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl OscalResultBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            start: None,
            end: None,
            props: Vec::new(),
            links: Vec::new(),
            local_definitions: None,
            reviewed_controls: None,
            attestation: Vec::new(),
            assessment_log: None,
            observations: Vec::new(),
            risks: Vec::new(),
            findings: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for OscalResultBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl OscalResultBuilder {
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
    ///Set the `start` field.
    pub fn start(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.start = Some(v.into());
        self
    }
    ///Set the `end` field.
    pub fn end(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.end = Some(v.into());
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
    ///Set the `local-definitions` field.
    pub fn local_definitions(mut self, v: impl Into<LocalDefinitions>) -> Self {
        self.local_definitions = Some(v.into());
        self
    }
    ///Set the `reviewed-controls` field.
    pub fn reviewed_controls(mut self, v: impl Into<ReviewedControls>) -> Self {
        self.reviewed_controls = Some(v.into());
        self
    }
    ///Set the `attestation` field.
    pub fn attestation(mut self, v: impl Into<Attestation>) -> Self {
        self.attestation.push(v.into());
        self
    }
    ///Set the `assessment-log` field.
    pub fn assessment_log(mut self, v: impl Into<AssessmentLog>) -> Self {
        self.assessment_log = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<OscalResult, BuildError> {
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
        let start = self
            .start
            .ok_or_else(|| BuildError::MissingField("required field `start` not set"))?;
        let reviewed_controls = self
            .reviewed_controls
            .ok_or_else(|| BuildError::MissingField(
                "required field `reviewed-controls` not set",
            ))?;
        Ok(OscalResult {
            uuid,
            title,
            description,
            start,
            end: self.end,
            props: self.props,
            links: self.links,
            local_definitions: self.local_definitions,
            reviewed_controls,
            attestation: self.attestation,
            assessment_log: self.assessment_log,
            observations: self.observations,
            risks: self.risks,
            findings: self.findings,
            remarks: self.remarks,
        })
    }
}
impl OscalResult {
    /// Return a new builder for this type.
    pub fn builder() -> OscalResultBuilder {
        OscalResultBuilder::new()
    }
}
///Used by assessment-results to import information about the original plan for assessing the system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportAp {
    ///A resolvable URL reference to the assessment plan governing the assessment activities.
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
pub struct ImportApBuilder {
    href: Option<crate::primitives::UriReference>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ImportApBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self { href: None, remarks: None }
    }
}
impl Default for ImportApBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ImportApBuilder {
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
    pub fn build(self) -> ::core::result::Result<ImportAp, BuildError> {
        let href = self
            .href
            .ok_or_else(|| BuildError::MissingField("required field `href` not set"))?;
        Ok(ImportAp {
            href,
            remarks: self.remarks,
        })
    }
}
impl ImportAp {
    /// Return a new builder for this type.
    pub fn builder() -> ImportApBuilder {
        ImportApBuilder::new()
    }
}
