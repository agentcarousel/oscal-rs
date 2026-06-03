// @generated — do not edit by hand.
// Run `cargo run -p oscal-metaschema-gen` to regenerate.

//!Generated OSCAL types for the `types` model. Do not edit by hand.
#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    unused_imports,
    unused_mut,
    unreachable_code,
    dead_code
)]
use serde::{Deserialize, Serialize};
/// Error returned when a required field was not set before calling `build()`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BuildError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),
}
///Used by the assessment plan and POA&M to import information about the system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportSsp {
    ///A resolvable URL reference to the system security plan for the system being assessed.
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
pub struct ImportSspBuilder {
    href: Option<crate::primitives::UriReference>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ImportSspBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            href: None,
            remarks: None,
        }
    }
}
impl Default for ImportSspBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ImportSspBuilder {
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
    pub fn build(self) -> ::core::result::Result<ImportSsp, BuildError> {
        let href = self
            .href
            .ok_or_else(|| BuildError::MissingField("required field `href` not set"))?;
        Ok(ImportSsp {
            href,
            remarks: self.remarks,
        })
    }
}
impl ImportSsp {
    /// Return a new builder for this type.
    pub fn builder() -> ImportSspBuilder {
        ImportSspBuilder::new()
    }
}
///A local definition of a control objective for this assessment. Uses catalog syntax for control objective and assessment actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalObjective {
    ///A human-readable description of this control objective.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default)]
    pub parts: Vec<Part>,
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
pub struct LocalObjectiveBuilder {
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    parts: Vec<Part>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl LocalObjectiveBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            parts: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for LocalObjectiveBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl LocalObjectiveBuilder {
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `parts` field.
    pub fn parts(mut self, v: impl Into<Part>) -> Self {
        self.parts.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<LocalObjective, BuildError> {
        Ok(LocalObjective {
            description: self.description,
            props: self.props,
            links: self.links,
            parts: self.parts,
            remarks: self.remarks,
        })
    }
}
impl LocalObjective {
    /// Return a new builder for this type.
    pub fn builder() -> LocalObjectiveBuilder {
        LocalObjectiveBuilder::new()
    }
}
///A local definition of a control objective. Uses catalog syntax for control objective and assessment activities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentMethod {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this assessment method elsewhere inthis or other OSCAL instances. The locally definedUUIDof theassessment methodcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///A human-readable description of this assessment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    pub assessment_part: AssessmentPart,
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
pub struct AssessmentMethodBuilder {
    uuid: Option<uuid::Uuid>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    assessment_part: Option<AssessmentPart>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl AssessmentMethodBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            assessment_part: None,
            remarks: None,
        }
    }
}
impl Default for AssessmentMethodBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssessmentMethodBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `assessment-part` field.
    pub fn assessment_part(mut self, v: impl Into<AssessmentPart>) -> Self {
        self.assessment_part = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<AssessmentMethod, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let assessment_part = self
            .assessment_part
            .ok_or_else(|| BuildError::MissingField("required field `assessment-part` not set"))?;
        Ok(AssessmentMethod {
            uuid,
            description: self.description,
            props: self.props,
            links: self.links,
            assessment_part,
            remarks: self.remarks,
        })
    }
}
impl AssessmentMethod {
    /// Return a new builder for this type.
    pub fn builder() -> AssessmentMethodBuilder {
        AssessmentMethodBuilder::new()
    }
}
///Identifies an individual step in a series of steps related to an activity, such as an assessment test or examination procedure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Step {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this step elsewhere inthis or other OSCAL instances. The locally definedUUIDof thestep(in a series of steps) can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The title for this step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///A human-readable description of this step.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_controls: Option<ReviewedControls>,
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
pub struct StepBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    reviewed_controls: Option<ReviewedControls>,
    responsible_roles: Vec<ResponsibleRole>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl StepBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            reviewed_controls: None,
            responsible_roles: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for StepBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl StepBuilder {
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `reviewed-controls` field.
    pub fn reviewed_controls(mut self, v: impl Into<ReviewedControls>) -> Self {
        self.reviewed_controls = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Step, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(Step {
            uuid,
            title: self.title,
            description,
            props: self.props,
            links: self.links,
            reviewed_controls: self.reviewed_controls,
            responsible_roles: self.responsible_roles,
            remarks: self.remarks,
        })
    }
}
impl Step {
    /// Return a new builder for this type.
    pub fn builder() -> StepBuilder {
        StepBuilder::new()
    }
}
///Identifies an assessment or related process that can be performed. In the assessment plan, this is an intended activity which may be associated with an assessment task. In the assessment results, this an activity that was actually performed as part of an assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Activity {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this assessment activity elsewhere inthis or other OSCAL instances. The locally definedUUIDof theactivitycan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The title for this included activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///A human-readable description of this included activity.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///Identifies an individual step in a series of steps related to an activity, such as an assessment test or examination procedure.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<Step>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_controls: Option<ReviewedControls>,
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
pub struct ActivityBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    step: Vec<Step>,
    related_controls: Option<ReviewedControls>,
    responsible_roles: Vec<ResponsibleRole>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ActivityBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            step: Vec::new(),
            related_controls: None,
            responsible_roles: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ActivityBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ActivityBuilder {
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `step` field.
    pub fn step(mut self, v: impl Into<Step>) -> Self {
        self.step.push(v.into());
        self
    }
    ///Set the `related-controls` field.
    pub fn related_controls(mut self, v: impl Into<ReviewedControls>) -> Self {
        self.related_controls = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Activity, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(Activity {
            uuid,
            title: self.title,
            description,
            props: self.props,
            links: self.links,
            step: self.step,
            related_controls: self.related_controls,
            responsible_roles: self.responsible_roles,
            remarks: self.remarks,
        })
    }
}
impl Activity {
    /// Return a new builder for this type.
    pub fn builder() -> ActivityBuilder {
        ActivityBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimingChoice1 {}
///The timing under which the task is intended to occur.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Timing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_choice1: Option<TimingChoice1>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct TimingBuilder {
    timing_choice1: Option<TimingChoice1>,
}
impl TimingBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            timing_choice1: None,
        }
    }
}
impl Default for TimingBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl TimingBuilder {
    ///Set the `timing_choice1` field.
    pub fn timing_choice1(mut self, v: impl Into<TimingChoice1>) -> Self {
        self.timing_choice1 = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Timing, BuildError> {
        Ok(Timing {
            timing_choice1: self.timing_choice1,
        })
    }
}
impl Timing {
    /// Return a new builder for this type.
    pub fn builder() -> TimingBuilder {
        TimingBuilder::new()
    }
}
///Used to indicate that a task is dependent on another task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Dependency {
    ///Amachine-orientedidentifier reference to a unique task.
    pub task_uuid: uuid::Uuid,
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
pub struct DependencyBuilder {
    task_uuid: Option<uuid::Uuid>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl DependencyBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            task_uuid: None,
            remarks: None,
        }
    }
}
impl Default for DependencyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl DependencyBuilder {
    ///Set the `task-uuid` field.
    pub fn task_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.task_uuid = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Dependency, BuildError> {
        let task_uuid = self
            .task_uuid
            .ok_or_else(|| BuildError::MissingField("required field `task-uuid` not set"))?;
        Ok(Dependency {
            task_uuid,
            remarks: self.remarks,
        })
    }
}
impl Dependency {
    /// Return a new builder for this type.
    pub fn builder() -> DependencyBuilder {
        DependencyBuilder::new()
    }
}
///Identifies an individual activity to be performed as part of a task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssociatedActivity {
    ///Amachine-orientedidentifier reference to an activity defined in the list of activities.
    pub activity_uuid: uuid::Uuid,
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
    #[serde(default)]
    pub subjects: Vec<AssessmentSubject>,
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
pub struct AssociatedActivityBuilder {
    activity_uuid: Option<uuid::Uuid>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_roles: Vec<ResponsibleRole>,
    subjects: Vec<AssessmentSubject>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl AssociatedActivityBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            activity_uuid: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_roles: Vec::new(),
            subjects: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for AssociatedActivityBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssociatedActivityBuilder {
    ///Set the `activity-uuid` field.
    pub fn activity_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.activity_uuid = Some(v.into());
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
    ///Set the `subjects` field.
    pub fn subjects(mut self, v: impl Into<AssessmentSubject>) -> Self {
        self.subjects.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<AssociatedActivity, BuildError> {
        let activity_uuid = self
            .activity_uuid
            .ok_or_else(|| BuildError::MissingField("required field `activity-uuid` not set"))?;
        Ok(AssociatedActivity {
            activity_uuid,
            props: self.props,
            links: self.links,
            responsible_roles: self.responsible_roles,
            subjects: self.subjects,
            remarks: self.remarks,
        })
    }
}
impl AssociatedActivity {
    /// Return a new builder for this type.
    pub fn builder() -> AssociatedActivityBuilder {
        AssociatedActivityBuilder::new()
    }
}
///Represents a scheduled event or milestone, which may be associated with a series of assessment actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Task {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this task elsewhere inthis or other OSCAL instances. The locally definedUUIDof thetaskcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The type of task.
    #[serde(rename = "type")]
    pub type_: String,
    ///The title for this task.
    pub title: crate::primitives::MarkupLine,
    ///A human-readable description of this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///The timing under which the task is intended to occur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
    ///Used to indicate that a task is dependent on another task.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency: Vec<Dependency>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<Task>,
    ///Identifies an individual activity to be performed as part of a task.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub associated_activity: Vec<AssociatedActivity>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<AssessmentSubject>,
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
pub struct TaskBuilder {
    uuid: Option<uuid::Uuid>,
    type_: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    timing: Option<Timing>,
    dependency: Vec<Dependency>,
    tasks: Vec<Task>,
    associated_activity: Vec<AssociatedActivity>,
    subjects: Vec<AssessmentSubject>,
    responsible_roles: Vec<ResponsibleRole>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl TaskBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            type_: None,
            title: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            timing: None,
            dependency: Vec::new(),
            tasks: Vec::new(),
            associated_activity: Vec::new(),
            subjects: Vec::new(),
            responsible_roles: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for TaskBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl TaskBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `type` field.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `timing` field.
    pub fn timing(mut self, v: impl Into<Timing>) -> Self {
        self.timing = Some(v.into());
        self
    }
    ///Set the `dependency` field.
    pub fn dependency(mut self, v: impl Into<Dependency>) -> Self {
        self.dependency.push(v.into());
        self
    }
    ///Set the `tasks` field.
    pub fn tasks(mut self, v: impl Into<Task>) -> Self {
        self.tasks.push(v.into());
        self
    }
    ///Set the `associated-activity` field.
    pub fn associated_activity(mut self, v: impl Into<AssociatedActivity>) -> Self {
        self.associated_activity.push(v.into());
        self
    }
    ///Set the `subjects` field.
    pub fn subjects(mut self, v: impl Into<AssessmentSubject>) -> Self {
        self.subjects.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Task, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let type_ = self
            .type_
            .ok_or_else(|| BuildError::MissingField("required field `type` not set"))?;
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        Ok(Task {
            uuid,
            type_,
            title,
            description: self.description,
            props: self.props,
            links: self.links,
            timing: self.timing,
            dependency: self.dependency,
            tasks: self.tasks,
            associated_activity: self.associated_activity,
            subjects: self.subjects,
            responsible_roles: self.responsible_roles,
            remarks: self.remarks,
        })
    }
}
impl Task {
    /// Return a new builder for this type.
    pub fn builder() -> TaskBuilder {
        TaskBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ControlSelectionChoice1 {
    IncludeAll(IncludeAll),
    SelectControlById(Vec<SelectControlById>),
}
///Identifies the controls being assessed. In the assessment plan, these are the planned controls. In the assessment results, these are the actual controls, and reflects any changes from the plan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlSelection {
    ///A human-readable description of in-scope controls specified for assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_selection_choice1: Option<ControlSelectionChoice1>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_controls: Vec<SelectControlById>,
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
pub struct ControlSelectionBuilder {
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    control_selection_choice1: Option<ControlSelectionChoice1>,
    exclude_controls: Vec<SelectControlById>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ControlSelectionBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            control_selection_choice1: None,
            exclude_controls: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ControlSelectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ControlSelectionBuilder {
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `control_selection_choice1` field.
    pub fn control_selection_choice1(mut self, v: impl Into<ControlSelectionChoice1>) -> Self {
        self.control_selection_choice1 = Some(v.into());
        self
    }
    ///Set the `exclude-controls` field.
    pub fn exclude_controls(mut self, v: impl Into<SelectControlById>) -> Self {
        self.exclude_controls.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<ControlSelection, BuildError> {
        Ok(ControlSelection {
            description: self.description,
            props: self.props,
            links: self.links,
            control_selection_choice1: self.control_selection_choice1,
            exclude_controls: self.exclude_controls,
            remarks: self.remarks,
        })
    }
}
impl ControlSelection {
    /// Return a new builder for this type.
    pub fn builder() -> ControlSelectionBuilder {
        ControlSelectionBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ControlObjectiveSelectionChoice1 {
    IncludeAll(IncludeAll),
    SelectObjectiveById(Vec<SelectObjectiveById>),
}
///Identifies the control objectives of the assessment. In the assessment plan, these are the planned objectives. In the assessment results, these are the assessed objectives, and reflects any changes from the plan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlObjectiveSelection {
    ///A human-readable description of this collection of control objectives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_objective_selection_choice1: Option<ControlObjectiveSelectionChoice1>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_objectives: Vec<SelectObjectiveById>,
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
pub struct ControlObjectiveSelectionBuilder {
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    control_objective_selection_choice1: Option<ControlObjectiveSelectionChoice1>,
    exclude_objectives: Vec<SelectObjectiveById>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ControlObjectiveSelectionBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            control_objective_selection_choice1: None,
            exclude_objectives: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ControlObjectiveSelectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ControlObjectiveSelectionBuilder {
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `control_objective_selection_choice1` field.
    pub fn control_objective_selection_choice1(
        mut self,
        v: impl Into<ControlObjectiveSelectionChoice1>,
    ) -> Self {
        self.control_objective_selection_choice1 = Some(v.into());
        self
    }
    ///Set the `exclude-objectives` field.
    pub fn exclude_objectives(mut self, v: impl Into<SelectObjectiveById>) -> Self {
        self.exclude_objectives.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<ControlObjectiveSelection, BuildError> {
        Ok(ControlObjectiveSelection {
            description: self.description,
            props: self.props,
            links: self.links,
            control_objective_selection_choice1: self.control_objective_selection_choice1,
            exclude_objectives: self.exclude_objectives,
            remarks: self.remarks,
        })
    }
}
impl ControlObjectiveSelection {
    /// Return a new builder for this type.
    pub fn builder() -> ControlObjectiveSelectionBuilder {
        ControlObjectiveSelectionBuilder::new()
    }
}
///Identifies the controls being assessed and their control objectives.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReviewedControls {
    ///A human-readable description of control objectives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///Identifies the controls being assessed. In the assessment plan, these are the planned controls. In the assessment results, these are the actual controls, and reflects any changes from the plan.
    #[serde(default)]
    pub control_selection: Vec<ControlSelection>,
    ///Identifies the control objectives of the assessment. In the assessment plan, these are the planned objectives. In the assessment results, these are the assessed objectives, and reflects any changes from the plan.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub control_objective_selection: Vec<ControlObjectiveSelection>,
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
pub struct ReviewedControlsBuilder {
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    control_selection: Vec<ControlSelection>,
    control_objective_selection: Vec<ControlObjectiveSelection>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ReviewedControlsBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            control_selection: Vec::new(),
            control_objective_selection: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ReviewedControlsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ReviewedControlsBuilder {
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `control-selection` field.
    pub fn control_selection(mut self, v: impl Into<ControlSelection>) -> Self {
        self.control_selection.push(v.into());
        self
    }
    ///Set the `control-objective-selection` field.
    pub fn control_objective_selection(mut self, v: impl Into<ControlObjectiveSelection>) -> Self {
        self.control_objective_selection.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<ReviewedControls, BuildError> {
        Ok(ReviewedControls {
            description: self.description,
            props: self.props,
            links: self.links,
            control_selection: self.control_selection,
            control_objective_selection: self.control_objective_selection,
            remarks: self.remarks,
        })
    }
}
impl ReviewedControls {
    /// Return a new builder for this type.
    pub fn builder() -> ReviewedControlsBuilder {
        ReviewedControlsBuilder::new()
    }
}
///Used to select a control for inclusion/exclusion based on one or more control identifiers. A set of statement identifiers can be used to target the inclusion/exclusion to only specific control statements providing more granularity over the specific statements that are within the assessment scope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectControlById {
    ///
    pub control_id: String,
    ///Used to constrain the selection to only specificity identified statements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statement_id: Vec<String>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct SelectControlByIdBuilder {
    control_id: Option<String>,
    statement_id: Vec<String>,
}
impl SelectControlByIdBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            control_id: None,
            statement_id: Vec::new(),
        }
    }
}
impl Default for SelectControlByIdBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SelectControlByIdBuilder {
    ///Set the `control-id` field.
    pub fn control_id(mut self, v: impl Into<String>) -> Self {
        self.control_id = Some(v.into());
        self
    }
    ///Set the `statement-id` field.
    pub fn statement_id(mut self, v: impl Into<String>) -> Self {
        self.statement_id.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<SelectControlById, BuildError> {
        let control_id = self
            .control_id
            .ok_or_else(|| BuildError::MissingField("required field `control-id` not set"))?;
        Ok(SelectControlById {
            control_id,
            statement_id: self.statement_id,
        })
    }
}
impl SelectControlById {
    /// Return a new builder for this type.
    pub fn builder() -> SelectControlByIdBuilder {
        SelectControlByIdBuilder::new()
    }
}
///Used to select a control objective for inclusion/exclusion based on the control objective's identifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectObjectiveById {
    ///
    pub objective_id: String,
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
pub struct SelectObjectiveByIdBuilder {
    objective_id: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SelectObjectiveByIdBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            objective_id: None,
            remarks: None,
        }
    }
}
impl Default for SelectObjectiveByIdBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SelectObjectiveByIdBuilder {
    ///Set the `objective-id` field.
    pub fn objective_id(mut self, v: impl Into<String>) -> Self {
        self.objective_id = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<SelectObjectiveById, BuildError> {
        let objective_id = self
            .objective_id
            .ok_or_else(|| BuildError::MissingField("required field `objective-id` not set"))?;
        Ok(SelectObjectiveById {
            objective_id,
            remarks: self.remarks,
        })
    }
}
impl SelectObjectiveById {
    /// Return a new builder for this type.
    pub fn builder() -> SelectObjectiveByIdBuilder {
        SelectObjectiveByIdBuilder::new()
    }
}
///Assessment subjects will be identified while conducting the referenced activity-instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Source {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference (in this or other OSCAL instances) an assessment activity to be performed as part of the event. The locally definedUUIDof thetaskcan be used to reference the data item locally or globally (e.g., in animported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub task_uuid: uuid::Uuid,
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
pub struct SourceBuilder {
    task_uuid: Option<uuid::Uuid>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SourceBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            task_uuid: None,
            remarks: None,
        }
    }
}
impl Default for SourceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SourceBuilder {
    ///Set the `task-uuid` field.
    pub fn task_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.task_uuid = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Source, BuildError> {
        let task_uuid = self
            .task_uuid
            .ok_or_else(|| BuildError::MissingField("required field `task-uuid` not set"))?;
        Ok(Source {
            task_uuid,
            remarks: self.remarks,
        })
    }
}
impl Source {
    /// Return a new builder for this type.
    pub fn builder() -> SourceBuilder {
        SourceBuilder::new()
    }
}
///Used when the assessment subjects will be determined as part of one or more other assessment activities. These assessment subjects will be recorded in the assessment results in the assessment log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentSubjectPlaceholder {
    ///Amachine-oriented,globally uniqueidentifier for a set of assessment subjects that will be identified by a task or an activity that is part of a task. The locally definedUUIDof theassessment subject placeholdercan be used to reference the data item locally or globally (e.g., in animported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///A human-readable description of intent of this assessment subject placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///Assessment subjects will be identified while conducting the referenced activity-instance.
    #[serde(default)]
    pub source: Vec<Source>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct AssessmentSubjectPlaceholderBuilder {
    uuid: Option<uuid::Uuid>,
    description: Option<crate::primitives::MarkupMultiline>,
    source: Vec<Source>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl AssessmentSubjectPlaceholderBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            description: None,
            source: Vec::new(),
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for AssessmentSubjectPlaceholderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssessmentSubjectPlaceholderBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.description = Some(v.into());
        self
    }
    ///Set the `source` field.
    pub fn source(mut self, v: impl Into<Source>) -> Self {
        self.source.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<AssessmentSubjectPlaceholder, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        Ok(AssessmentSubjectPlaceholder {
            uuid,
            description: self.description,
            source: self.source,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl AssessmentSubjectPlaceholder {
    /// Return a new builder for this type.
    pub fn builder() -> AssessmentSubjectPlaceholderBuilder {
        AssessmentSubjectPlaceholderBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssessmentSubjectChoice1 {
    IncludeAll(IncludeAll),
    SelectSubjectById(Vec<SelectSubjectById>),
}
///Identifies system elements being assessed, such as components, inventory items, and locations. In the assessment plan, this identifies a planned assessment subject. In the assessment results this is an actual assessment subject, and reflects any changes from the plan. exactly what will be the focus of this assessment. Any subjects not identified in this way are out-of-scope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentSubject {
    ///Indicates the type of assessment subject, such as a component, inventory, item, location, or party represented by this selection statement.
    #[serde(rename = "type")]
    pub type_: String,
    ///A human-readable description of the collection of subjects being included in this assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_subject_choice1: Option<AssessmentSubjectChoice1>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_subjects: Vec<SelectSubjectById>,
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
pub struct AssessmentSubjectBuilder {
    type_: Option<String>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    assessment_subject_choice1: Option<AssessmentSubjectChoice1>,
    exclude_subjects: Vec<SelectSubjectById>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl AssessmentSubjectBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            type_: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            assessment_subject_choice1: None,
            exclude_subjects: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for AssessmentSubjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssessmentSubjectBuilder {
    ///Set the `type` field.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `assessment_subject_choice1` field.
    pub fn assessment_subject_choice1(mut self, v: impl Into<AssessmentSubjectChoice1>) -> Self {
        self.assessment_subject_choice1 = Some(v.into());
        self
    }
    ///Set the `exclude-subjects` field.
    pub fn exclude_subjects(mut self, v: impl Into<SelectSubjectById>) -> Self {
        self.exclude_subjects.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<AssessmentSubject, BuildError> {
        let type_ = self
            .type_
            .ok_or_else(|| BuildError::MissingField("required field `type` not set"))?;
        Ok(AssessmentSubject {
            type_,
            description: self.description,
            props: self.props,
            links: self.links,
            assessment_subject_choice1: self.assessment_subject_choice1,
            exclude_subjects: self.exclude_subjects,
            remarks: self.remarks,
        })
    }
}
impl AssessmentSubject {
    /// Return a new builder for this type.
    pub fn builder() -> AssessmentSubjectBuilder {
        AssessmentSubjectBuilder::new()
    }
}
///Identifies a set of assessment subjects to include/exclude by UUID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectSubjectById {
    ///
    pub subject_uuid: String,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct SelectSubjectByIdBuilder {
    subject_uuid: Option<String>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SelectSubjectByIdBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            subject_uuid: None,
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for SelectSubjectByIdBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SelectSubjectByIdBuilder {
    ///Set the `subject-uuid` field.
    pub fn subject_uuid(mut self, v: impl Into<String>) -> Self {
        self.subject_uuid = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<SelectSubjectById, BuildError> {
        let subject_uuid = self
            .subject_uuid
            .ok_or_else(|| BuildError::MissingField("required field `subject-uuid` not set"))?;
        Ok(SelectSubjectById {
            subject_uuid,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl SelectSubjectById {
    /// Return a new builder for this type.
    pub fn builder() -> SelectSubjectByIdBuilder {
        SelectSubjectByIdBuilder::new()
    }
}
///Ahuman-orientedidentifier reference to a resource. Use type to indicate whether the identified resource is a component, inventory item, location, user, or something else.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SubjectReference {
    ///
    pub subject_uuid: String,
    ///The title or name for the referenced subject.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct SubjectReferenceBuilder {
    subject_uuid: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SubjectReferenceBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            subject_uuid: None,
            title: None,
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for SubjectReferenceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SubjectReferenceBuilder {
    ///Set the `subject-uuid` field.
    pub fn subject_uuid(mut self, v: impl Into<String>) -> Self {
        self.subject_uuid = Some(v.into());
        self
    }
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<SubjectReference, BuildError> {
        let subject_uuid = self
            .subject_uuid
            .ok_or_else(|| BuildError::MissingField("required field `subject-uuid` not set"))?;
        Ok(SubjectReference {
            subject_uuid,
            title: self.title,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl SubjectReference {
    /// Return a new builder for this type.
    pub fn builder() -> SubjectReferenceBuilder {
        SubjectReferenceBuilder::new()
    }
}
///The set of components that are used by the assessment platform.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UsesComponent {
    ///Amachine-orientedidentifier reference to a component that is implemented as part of an inventory item.
    pub component_uuid: uuid::Uuid,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_parties: Vec<ResponsibleParty>,
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
pub struct UsesComponentBuilder {
    component_uuid: Option<uuid::Uuid>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_parties: Vec<ResponsibleParty>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl UsesComponentBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            component_uuid: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_parties: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for UsesComponentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl UsesComponentBuilder {
    ///Set the `component-uuid` field.
    pub fn component_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.component_uuid = Some(v.into());
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
    ///Set the `responsible-parties` field.
    pub fn responsible_parties(mut self, v: impl Into<ResponsibleParty>) -> Self {
        self.responsible_parties.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<UsesComponent, BuildError> {
        let component_uuid = self
            .component_uuid
            .ok_or_else(|| BuildError::MissingField("required field `component-uuid` not set"))?;
        Ok(UsesComponent {
            component_uuid,
            props: self.props,
            links: self.links,
            responsible_parties: self.responsible_parties,
            remarks: self.remarks,
        })
    }
}
impl UsesComponent {
    /// Return a new builder for this type.
    pub fn builder() -> UsesComponentBuilder {
        UsesComponentBuilder::new()
    }
}
///Used to represent the toolset used to perform aspects of the assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentPlatform {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this assessment platform elsewhere in this or other OSCAL instances. The locally definedUUIDof theassessment platformcan be used to reference the data item locally or globally (e.g., in animported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The title or name for the assessment platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///The set of components that are used by the assessment platform.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uses_component: Vec<UsesComponent>,
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
pub struct AssessmentPlatformBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    props: Vec<Property>,
    links: Vec<Link>,
    uses_component: Vec<UsesComponent>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl AssessmentPlatformBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            props: Vec::new(),
            links: Vec::new(),
            uses_component: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for AssessmentPlatformBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssessmentPlatformBuilder {
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
    ///Set the `uses-component` field.
    pub fn uses_component(mut self, v: impl Into<UsesComponent>) -> Self {
        self.uses_component.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<AssessmentPlatform, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        Ok(AssessmentPlatform {
            uuid,
            title: self.title,
            props: self.props,
            links: self.links,
            uses_component: self.uses_component,
            remarks: self.remarks,
        })
    }
}
impl AssessmentPlatform {
    /// Return a new builder for this type.
    pub fn builder() -> AssessmentPlatformBuilder {
        AssessmentPlatformBuilder::new()
    }
}
///Identifies the assets used to perform this assessment, such as the assessment team, scanning tools, and assumptions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentAssets {
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<SystemComponent>,
    ///Used to represent the toolset used to perform aspects of the assessment.
    #[serde(default)]
    pub assessment_platform: Vec<AssessmentPlatform>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct AssessmentAssetsBuilder {
    components: Vec<SystemComponent>,
    assessment_platform: Vec<AssessmentPlatform>,
}
impl AssessmentAssetsBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            assessment_platform: Vec::new(),
        }
    }
}
impl Default for AssessmentAssetsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssessmentAssetsBuilder {
    ///Set the `components` field.
    pub fn components(mut self, v: impl Into<SystemComponent>) -> Self {
        self.components.push(v.into());
        self
    }
    ///Set the `assessment-platform` field.
    pub fn assessment_platform(mut self, v: impl Into<AssessmentPlatform>) -> Self {
        self.assessment_platform.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<AssessmentAssets, BuildError> {
        Ok(AssessmentAssets {
            components: self.components,
            assessment_platform: self.assessment_platform,
        })
    }
}
impl AssessmentAssets {
    /// Return a new builder for this type.
    pub fn builder() -> AssessmentAssetsBuilder {
        AssessmentAssetsBuilder::new()
    }
}
///A determination of if the objective is satisfied or not within a given system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Status {
    ///An indication as to whether the objective is satisfied or not.
    pub state: String,
    ///The reason the objective was given it's status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
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
pub struct StatusBuilder {
    state: Option<String>,
    reason: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl StatusBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            state: None,
            reason: None,
            remarks: None,
        }
    }
}
impl Default for StatusBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl StatusBuilder {
    ///Set the `state` field.
    pub fn state(mut self, v: impl Into<String>) -> Self {
        self.state = Some(v.into());
        self
    }
    ///Set the `reason` field.
    pub fn reason(mut self, v: impl Into<String>) -> Self {
        self.reason = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Status, BuildError> {
        let state = self
            .state
            .ok_or_else(|| BuildError::MissingField("required field `state` not set"))?;
        Ok(Status {
            state,
            reason: self.reason,
            remarks: self.remarks,
        })
    }
}
impl Status {
    /// Return a new builder for this type.
    pub fn builder() -> StatusBuilder {
        StatusBuilder::new()
    }
}
///Captures an assessor's conclusions regarding the degree to which an objective is satisfied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FindingTarget {
    ///Identifies the type of the target.
    #[serde(rename = "type")]
    pub type_: String,
    ///Amachine-orientedidentifier reference for a specific target qualified by thetype.
    pub target_id: String,
    ///The title for this objective status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///A human-readable description of the assessor's conclusions regarding the degree to which an objective is satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///A determination of if the objective is satisfied or not within a given system.
    pub status: Status,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_status: Option<ImplementationStatus>,
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
pub struct FindingTargetBuilder {
    type_: Option<String>,
    target_id: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    status: Option<Status>,
    implementation_status: Option<ImplementationStatus>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl FindingTargetBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            type_: None,
            target_id: None,
            title: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            status: None,
            implementation_status: None,
            remarks: None,
        }
    }
}
impl Default for FindingTargetBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl FindingTargetBuilder {
    ///Set the `type` field.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    ///Set the `target-id` field.
    pub fn target_id(mut self, v: impl Into<String>) -> Self {
        self.target_id = Some(v.into());
        self
    }
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `status` field.
    pub fn status(mut self, v: impl Into<Status>) -> Self {
        self.status = Some(v.into());
        self
    }
    ///Set the `implementation-status` field.
    pub fn implementation_status(mut self, v: impl Into<ImplementationStatus>) -> Self {
        self.implementation_status = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<FindingTarget, BuildError> {
        let type_ = self
            .type_
            .ok_or_else(|| BuildError::MissingField("required field `type` not set"))?;
        let target_id = self
            .target_id
            .ok_or_else(|| BuildError::MissingField("required field `target-id` not set"))?;
        let status = self
            .status
            .ok_or_else(|| BuildError::MissingField("required field `status` not set"))?;
        Ok(FindingTarget {
            type_,
            target_id,
            title: self.title,
            description: self.description,
            props: self.props,
            links: self.links,
            status,
            implementation_status: self.implementation_status,
            remarks: self.remarks,
        })
    }
}
impl FindingTarget {
    /// Return a new builder for this type.
    pub fn builder() -> FindingTargetBuilder {
        FindingTargetBuilder::new()
    }
}
///Describes an individual finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Finding {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this finding inthis or other OSCAL instances. The locally definedUUIDof thefindingcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The title for this finding.
    pub title: crate::primitives::MarkupLine,
    ///A human-readable description of this finding.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<Origin>,
    ///
    pub target: FindingTarget,
    ///Amachine-orientedidentifier reference to the implementation statement in the SSP to which this finding is related.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_statement_uuid: Option<uuid::Uuid>,
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
pub struct FindingBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    origins: Vec<Origin>,
    target: Option<FindingTarget>,
    implementation_statement_uuid: Option<uuid::Uuid>,
    related_observations: Vec<RelatedObservation>,
    related_risks: Vec<AssociatedRisk>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl FindingBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            origins: Vec::new(),
            target: None,
            implementation_statement_uuid: None,
            related_observations: Vec::new(),
            related_risks: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for FindingBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl FindingBuilder {
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `origins` field.
    pub fn origins(mut self, v: impl Into<Origin>) -> Self {
        self.origins.push(v.into());
        self
    }
    ///Set the `target` field.
    pub fn target(mut self, v: impl Into<FindingTarget>) -> Self {
        self.target = Some(v.into());
        self
    }
    ///Set the `implementation-statement-uuid` field.
    pub fn implementation_statement_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.implementation_statement_uuid = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Finding, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        let target = self
            .target
            .ok_or_else(|| BuildError::MissingField("required field `target` not set"))?;
        Ok(Finding {
            uuid,
            title,
            description,
            props: self.props,
            links: self.links,
            origins: self.origins,
            target,
            implementation_statement_uuid: self.implementation_statement_uuid,
            related_observations: self.related_observations,
            related_risks: self.related_risks,
            remarks: self.remarks,
        })
    }
}
impl Finding {
    /// Return a new builder for this type.
    pub fn builder() -> FindingBuilder {
        FindingBuilder::new()
    }
}
///Relates the identified element to a set of referenced observations that were used to support its determination.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedObservation {
    ///Amachine-orientedidentifier reference to an observation defined in the list of observations.
    pub observation_uuid: uuid::Uuid,
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
pub struct RelatedObservationBuilder {
    observation_uuid: Option<uuid::Uuid>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl RelatedObservationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            observation_uuid: None,
            remarks: None,
        }
    }
}
impl Default for RelatedObservationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RelatedObservationBuilder {
    ///Set the `observation-uuid` field.
    pub fn observation_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.observation_uuid = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<RelatedObservation, BuildError> {
        let observation_uuid = self
            .observation_uuid
            .ok_or_else(|| BuildError::MissingField("required field `observation-uuid` not set"))?;
        Ok(RelatedObservation {
            observation_uuid,
            remarks: self.remarks,
        })
    }
}
impl RelatedObservation {
    /// Return a new builder for this type.
    pub fn builder() -> RelatedObservationBuilder {
        RelatedObservationBuilder::new()
    }
}
///Relates the finding to a set of referenced risks that were used to determine the finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssociatedRisk {
    ///Amachine-orientedidentifier reference to a risk defined in the list of risks.
    pub risk_uuid: uuid::Uuid,
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
pub struct AssociatedRiskBuilder {
    risk_uuid: Option<uuid::Uuid>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl AssociatedRiskBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            risk_uuid: None,
            remarks: None,
        }
    }
}
impl Default for AssociatedRiskBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssociatedRiskBuilder {
    ///Set the `risk-uuid` field.
    pub fn risk_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.risk_uuid = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<AssociatedRisk, BuildError> {
        let risk_uuid = self
            .risk_uuid
            .ok_or_else(|| BuildError::MissingField("required field `risk-uuid` not set"))?;
        Ok(AssociatedRisk {
            risk_uuid,
            remarks: self.remarks,
        })
    }
}
impl AssociatedRisk {
    /// Return a new builder for this type.
    pub fn builder() -> AssociatedRiskBuilder {
        AssociatedRiskBuilder::new()
    }
}
///Links this observation to relevant evidence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelevantEvidence {
    ///A resolvable URL reference to relevant evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<crate::primitives::UriReference>,
    ///A human-readable description of this evidence.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct RelevantEvidenceBuilder {
    href: Option<crate::primitives::UriReference>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl RelevantEvidenceBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            href: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for RelevantEvidenceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RelevantEvidenceBuilder {
    ///Set the `href` field.
    pub fn href(mut self, v: impl Into<crate::primitives::UriReference>) -> Self {
        self.href = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    pub fn build(self) -> ::core::result::Result<RelevantEvidence, BuildError> {
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(RelevantEvidence {
            href: self.href,
            description,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl RelevantEvidence {
    /// Return a new builder for this type.
    pub fn builder() -> RelevantEvidenceBuilder {
        RelevantEvidenceBuilder::new()
    }
}
///Describes an individual observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Observation {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this observation elsewhere inthis or other OSCAL instances. The locally definedUUIDof theobservationcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The title for this observation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///A human-readable description of this assessment observation.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///Identifies how the observation was made.
    #[serde(default)]
    pub method: Vec<String>,
    ///Identifies the nature of the observation. More than one may be used to further qualify and enable filtering.
    #[serde(rename = "type", default, skip_serializing_if = "Vec::is_empty")]
    pub type_: Vec<String>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<Origin>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<SubjectReference>,
    ///Links this observation to relevant evidence.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_evidence: Vec<RelevantEvidence>,
    ///Date/time stamp identifying when the finding information was collected.
    pub collected: chrono::DateTime<chrono::Utc>,
    ///Date/time identifying when the finding information is out-of-date and no longer valid. Typically used with continuous assessment scenarios.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<chrono::DateTime<chrono::Utc>>,
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
pub struct ObservationBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    method: Vec<String>,
    type_: Vec<String>,
    origins: Vec<Origin>,
    subjects: Vec<SubjectReference>,
    relevant_evidence: Vec<RelevantEvidence>,
    collected: Option<chrono::DateTime<chrono::Utc>>,
    expires: Option<chrono::DateTime<chrono::Utc>>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ObservationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            method: Vec::new(),
            type_: Vec::new(),
            origins: Vec::new(),
            subjects: Vec::new(),
            relevant_evidence: Vec::new(),
            collected: None,
            expires: None,
            remarks: None,
        }
    }
}
impl Default for ObservationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ObservationBuilder {
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `method` field.
    pub fn method(mut self, v: impl Into<String>) -> Self {
        self.method.push(v.into());
        self
    }
    ///Set the `type` field.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_.push(v.into());
        self
    }
    ///Set the `origins` field.
    pub fn origins(mut self, v: impl Into<Origin>) -> Self {
        self.origins.push(v.into());
        self
    }
    ///Set the `subjects` field.
    pub fn subjects(mut self, v: impl Into<SubjectReference>) -> Self {
        self.subjects.push(v.into());
        self
    }
    ///Set the `relevant-evidence` field.
    pub fn relevant_evidence(mut self, v: impl Into<RelevantEvidence>) -> Self {
        self.relevant_evidence.push(v.into());
        self
    }
    ///Set the `collected` field.
    pub fn collected(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.collected = Some(v.into());
        self
    }
    ///Set the `expires` field.
    pub fn expires(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.expires = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Observation, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        let collected = self
            .collected
            .ok_or_else(|| BuildError::MissingField("required field `collected` not set"))?;
        Ok(Observation {
            uuid,
            title: self.title,
            description,
            props: self.props,
            links: self.links,
            method: self.method,
            type_: self.type_,
            origins: self.origins,
            subjects: self.subjects,
            relevant_evidence: self.relevant_evidence,
            collected,
            expires: self.expires,
            remarks: self.remarks,
        })
    }
}
impl Observation {
    /// Return a new builder for this type.
    pub fn builder() -> ObservationBuilder {
        ObservationBuilder::new()
    }
}
///Identifies the source of the finding, such as a tool, interviewed person, or activity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Origin {
    ///
    #[serde(default)]
    pub actors: Vec<OriginActor>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_tasks: Vec<RelatedTask>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct OriginBuilder {
    actors: Vec<OriginActor>,
    related_tasks: Vec<RelatedTask>,
}
impl OriginBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            actors: Vec::new(),
            related_tasks: Vec::new(),
        }
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
    ///Set the `related-tasks` field.
    pub fn related_tasks(mut self, v: impl Into<RelatedTask>) -> Self {
        self.related_tasks.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Origin, BuildError> {
        Ok(Origin {
            actors: self.actors,
            related_tasks: self.related_tasks,
        })
    }
}
impl Origin {
    /// Return a new builder for this type.
    pub fn builder() -> OriginBuilder {
        OriginBuilder::new()
    }
}
///The actor that produces an observation, a finding, or a risk. One or more actor type can be used to specify a person that is using a tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OriginActor {
    ///The kind of actor.
    #[serde(rename = "type")]
    pub type_: String,
    ///Amachine-orientedidentifier reference to the tool or person based on the associated type.
    pub actor_uuid: uuid::Uuid,
    ///For a party, this can optionally be used to specify the role the actor was performing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct OriginActorBuilder {
    type_: Option<String>,
    actor_uuid: Option<uuid::Uuid>,
    role_id: Option<String>,
    props: Vec<Property>,
    links: Vec<Link>,
}
impl OriginActorBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            type_: None,
            actor_uuid: None,
            role_id: None,
            props: Vec::new(),
            links: Vec::new(),
        }
    }
}
impl Default for OriginActorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl OriginActorBuilder {
    ///Set the `type` field.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    ///Set the `actor-uuid` field.
    pub fn actor_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.actor_uuid = Some(v.into());
        self
    }
    ///Set the `role-id` field.
    pub fn role_id(mut self, v: impl Into<String>) -> Self {
        self.role_id = Some(v.into());
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
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<OriginActor, BuildError> {
        let type_ = self
            .type_
            .ok_or_else(|| BuildError::MissingField("required field `type` not set"))?;
        let actor_uuid = self
            .actor_uuid
            .ok_or_else(|| BuildError::MissingField("required field `actor-uuid` not set"))?;
        Ok(OriginActor {
            type_,
            actor_uuid,
            role_id: self.role_id,
            props: self.props,
            links: self.links,
        })
    }
}
impl OriginActor {
    /// Return a new builder for this type.
    pub fn builder() -> OriginActorBuilder {
        OriginActorBuilder::new()
    }
}
///Used to detail assessment subjects that were identified by this task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IdentifiedSubject {
    ///Amachine-orientedidentifier reference to a unique assessment subject placeholder defined by this task.
    pub subject_placeholder_uuid: uuid::Uuid,
    ///
    #[serde(default)]
    pub subjects: Vec<AssessmentSubject>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct IdentifiedSubjectBuilder {
    subject_placeholder_uuid: Option<uuid::Uuid>,
    subjects: Vec<AssessmentSubject>,
}
impl IdentifiedSubjectBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            subject_placeholder_uuid: None,
            subjects: Vec::new(),
        }
    }
}
impl Default for IdentifiedSubjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl IdentifiedSubjectBuilder {
    ///Set the `subject-placeholder-uuid` field.
    pub fn subject_placeholder_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.subject_placeholder_uuid = Some(v.into());
        self
    }
    ///Set the `subjects` field.
    pub fn subjects(mut self, v: impl Into<AssessmentSubject>) -> Self {
        self.subjects.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<IdentifiedSubject, BuildError> {
        let subject_placeholder_uuid = self.subject_placeholder_uuid.ok_or_else(|| {
            BuildError::MissingField("required field `subject-placeholder-uuid` not set")
        })?;
        Ok(IdentifiedSubject {
            subject_placeholder_uuid,
            subjects: self.subjects,
        })
    }
}
impl IdentifiedSubject {
    /// Return a new builder for this type.
    pub fn builder() -> IdentifiedSubjectBuilder {
        IdentifiedSubjectBuilder::new()
    }
}
///Identifies an individual task for which the containing object is a consequence of.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedTask {
    ///Amachine-orientedidentifier reference to a unique task.
    pub task_uuid: uuid::Uuid,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_parties: Vec<ResponsibleParty>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<AssessmentSubject>,
    ///Used to detail assessment subjects that were identified by this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified_subject: Option<IdentifiedSubject>,
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
pub struct RelatedTaskBuilder {
    task_uuid: Option<uuid::Uuid>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_parties: Vec<ResponsibleParty>,
    subjects: Vec<AssessmentSubject>,
    identified_subject: Option<IdentifiedSubject>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl RelatedTaskBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            task_uuid: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_parties: Vec::new(),
            subjects: Vec::new(),
            identified_subject: None,
            remarks: None,
        }
    }
}
impl Default for RelatedTaskBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RelatedTaskBuilder {
    ///Set the `task-uuid` field.
    pub fn task_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.task_uuid = Some(v.into());
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
    ///Set the `responsible-parties` field.
    pub fn responsible_parties(mut self, v: impl Into<ResponsibleParty>) -> Self {
        self.responsible_parties.push(v.into());
        self
    }
    ///Set the `subjects` field.
    pub fn subjects(mut self, v: impl Into<AssessmentSubject>) -> Self {
        self.subjects.push(v.into());
        self
    }
    ///Set the `identified-subject` field.
    pub fn identified_subject(mut self, v: impl Into<IdentifiedSubject>) -> Self {
        self.identified_subject = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<RelatedTask, BuildError> {
        let task_uuid = self
            .task_uuid
            .ok_or_else(|| BuildError::MissingField("required field `task-uuid` not set"))?;
        Ok(RelatedTask {
            task_uuid,
            props: self.props,
            links: self.links,
            responsible_parties: self.responsible_parties,
            subjects: self.subjects,
            identified_subject: self.identified_subject,
            remarks: self.remarks,
        })
    }
}
impl RelatedTask {
    /// Return a new builder for this type.
    pub fn builder() -> RelatedTaskBuilder {
        RelatedTaskBuilder::new()
    }
}
///A pointer, by ID, to an externally-defined threat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ThreatId {
    ///Specifies the source of the threat information.
    pub system: url::Url,
    ///An optional location for the threat data, from which this ID originates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<crate::primitives::UriReference>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<url::Url>,
}
///Describes an existing mitigating factor that may affect the overall determination of the risk, with an optional link to an implementation statement in the SSP.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MitigatingFactor {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this mitigating factor elsewhere inthis or other OSCAL instances. The locally definedUUIDof themitigating factorcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this implementation statement elsewhere inthis or other OSCAL instancess. The locally definedUUIDof theimplementation statementcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_uuid: Option<uuid::Uuid>,
    ///A human-readable description of this mitigating factor.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<SubjectReference>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct MitigatingFactorBuilder {
    uuid: Option<uuid::Uuid>,
    implementation_uuid: Option<uuid::Uuid>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    subjects: Vec<SubjectReference>,
}
impl MitigatingFactorBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            implementation_uuid: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            subjects: Vec::new(),
        }
    }
}
impl Default for MitigatingFactorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl MitigatingFactorBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `implementation-uuid` field.
    pub fn implementation_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.implementation_uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `subjects` field.
    pub fn subjects(mut self, v: impl Into<SubjectReference>) -> Self {
        self.subjects.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<MitigatingFactor, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(MitigatingFactor {
            uuid,
            implementation_uuid: self.implementation_uuid,
            description,
            props: self.props,
            links: self.links,
            subjects: self.subjects,
        })
    }
}
impl MitigatingFactor {
    /// Return a new builder for this type.
    pub fn builder() -> MitigatingFactorBuilder {
        MitigatingFactorBuilder::new()
    }
}
///Identifies an individual risk response that this log entry is for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedResponse {
    ///Amachine-orientedidentifier reference to a unique risk response.
    pub response_uuid: uuid::Uuid,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct RelatedResponseBuilder {
    response_uuid: Option<uuid::Uuid>,
    props: Vec<Property>,
    links: Vec<Link>,
    related_tasks: Vec<RelatedTask>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl RelatedResponseBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            response_uuid: None,
            props: Vec::new(),
            links: Vec::new(),
            related_tasks: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for RelatedResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RelatedResponseBuilder {
    ///Set the `response-uuid` field.
    pub fn response_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.response_uuid = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<RelatedResponse, BuildError> {
        let response_uuid = self
            .response_uuid
            .ok_or_else(|| BuildError::MissingField("required field `response-uuid` not set"))?;
        Ok(RelatedResponse {
            response_uuid,
            props: self.props,
            links: self.links,
            related_tasks: self.related_tasks,
            remarks: self.remarks,
        })
    }
}
impl RelatedResponse {
    /// Return a new builder for this type.
    pub fn builder() -> RelatedResponseBuilder {
        RelatedResponseBuilder::new()
    }
}
///Identifies an individual risk response that occurred as part of managing an identified risk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Entry {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this risk log entry elsewhere inthis or other OSCAL instances. The locally definedUUIDof therisk log entrycan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The title for this risk log entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///A human-readable description of what was done regarding the risk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///Identifies the start date and time of the event.
    pub start: chrono::DateTime<chrono::Utc>,
    ///Identifies the end date and time of the event. If the event is a point in time, the start and end will be the same date and time.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_change: Option<String>,
    ///Identifies an individual risk response that this log entry is for.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_response: Vec<RelatedResponse>,
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
    status_change: Option<String>,
    related_response: Vec<RelatedResponse>,
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
            status_change: None,
            related_response: Vec::new(),
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `status-change` field.
    pub fn status_change(mut self, v: impl Into<String>) -> Self {
        self.status_change = Some(v.into());
        self
    }
    ///Set the `related-response` field.
    pub fn related_response(mut self, v: impl Into<RelatedResponse>) -> Self {
        self.related_response.push(v.into());
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
            status_change: self.status_change,
            related_response: self.related_response,
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
///A log of all risk-related tasks taken.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiskLog {
    ///Identifies an individual risk response that occurred as part of managing an identified risk.
    #[serde(default)]
    pub entry: Vec<Entry>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct RiskLogBuilder {
    entry: Vec<Entry>,
}
impl RiskLogBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self { entry: Vec::new() }
    }
}
impl Default for RiskLogBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RiskLogBuilder {
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
    pub fn build(self) -> ::core::result::Result<RiskLog, BuildError> {
        Ok(RiskLog { entry: self.entry })
    }
}
impl RiskLog {
    /// Return a new builder for this type.
    pub fn builder() -> RiskLogBuilder {
        RiskLogBuilder::new()
    }
}
///An identified risk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Risk {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this risk elsewhere inthis or other OSCAL instances. The locally definedUUIDof theriskcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The title for this risk.
    pub title: crate::primitives::MarkupLine,
    ///A human-readable summary of the identified risk, to include a statement of how the risk impacts the system.
    pub description: crate::primitives::MarkupMultiline,
    ///An summary of impact for how the risk affects the system.
    pub statement: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    pub status: String,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<Origin>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threat_ids: Vec<ThreatId>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characterizations: Vec<Characterization>,
    ///Describes an existing mitigating factor that may affect the overall determination of the risk, with an optional link to an implementation statement in the SSP.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mitigating_factor: Vec<MitigatingFactor>,
    ///The date/time by which the risk must be resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remediations: Vec<Response>,
    ///A log of all risk-related tasks taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_log: Option<RiskLog>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_observations: Vec<RelatedObservation>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct RiskBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    statement: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    status: Option<String>,
    origins: Vec<Origin>,
    threat_ids: Vec<ThreatId>,
    characterizations: Vec<Characterization>,
    mitigating_factor: Vec<MitigatingFactor>,
    deadline: Option<chrono::DateTime<chrono::Utc>>,
    remediations: Vec<Response>,
    risk_log: Option<RiskLog>,
    related_observations: Vec<RelatedObservation>,
}
impl RiskBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            statement: None,
            props: Vec::new(),
            links: Vec::new(),
            status: None,
            origins: Vec::new(),
            threat_ids: Vec::new(),
            characterizations: Vec::new(),
            mitigating_factor: Vec::new(),
            deadline: None,
            remediations: Vec::new(),
            risk_log: None,
            related_observations: Vec::new(),
        }
    }
}
impl Default for RiskBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RiskBuilder {
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.description = Some(v.into());
        self
    }
    ///Set the `statement` field.
    pub fn statement(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.statement = Some(v.into());
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
    ///Set the `status` field.
    pub fn status(mut self, v: impl Into<String>) -> Self {
        self.status = Some(v.into());
        self
    }
    ///Set the `origins` field.
    pub fn origins(mut self, v: impl Into<Origin>) -> Self {
        self.origins.push(v.into());
        self
    }
    ///Set the `threat-ids` field.
    pub fn threat_ids(mut self, v: impl Into<ThreatId>) -> Self {
        self.threat_ids.push(v.into());
        self
    }
    ///Set the `characterizations` field.
    pub fn characterizations(mut self, v: impl Into<Characterization>) -> Self {
        self.characterizations.push(v.into());
        self
    }
    ///Set the `mitigating-factor` field.
    pub fn mitigating_factor(mut self, v: impl Into<MitigatingFactor>) -> Self {
        self.mitigating_factor.push(v.into());
        self
    }
    ///Set the `deadline` field.
    pub fn deadline(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.deadline = Some(v.into());
        self
    }
    ///Set the `remediations` field.
    pub fn remediations(mut self, v: impl Into<Response>) -> Self {
        self.remediations.push(v.into());
        self
    }
    ///Set the `risk-log` field.
    pub fn risk_log(mut self, v: impl Into<RiskLog>) -> Self {
        self.risk_log = Some(v.into());
        self
    }
    ///Set the `related-observations` field.
    pub fn related_observations(mut self, v: impl Into<RelatedObservation>) -> Self {
        self.related_observations.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Risk, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        let statement = self
            .statement
            .ok_or_else(|| BuildError::MissingField("required field `statement` not set"))?;
        let status = self
            .status
            .ok_or_else(|| BuildError::MissingField("required field `status` not set"))?;
        Ok(Risk {
            uuid,
            title,
            description,
            statement,
            props: self.props,
            links: self.links,
            status,
            origins: self.origins,
            threat_ids: self.threat_ids,
            characterizations: self.characterizations,
            mitigating_factor: self.mitigating_factor,
            deadline: self.deadline,
            remediations: self.remediations,
            risk_log: self.risk_log,
            related_observations: self.related_observations,
        })
    }
}
impl Risk {
    /// Return a new builder for this type.
    pub fn builder() -> RiskBuilder {
        RiskBuilder::new()
    }
}
///Used to indicate who created a log entry in what role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LoggedBy {
    ///Amachine-orientedidentifier reference to the party who is making the log entry.
    pub party_uuid: uuid::Uuid,
    ///A point to the role-id of the role in which the party is making the log entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
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
pub struct LoggedByBuilder {
    party_uuid: Option<uuid::Uuid>,
    role_id: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl LoggedByBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            party_uuid: None,
            role_id: None,
            remarks: None,
        }
    }
}
impl Default for LoggedByBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl LoggedByBuilder {
    ///Set the `party-uuid` field.
    pub fn party_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.party_uuid = Some(v.into());
        self
    }
    ///Set the `role-id` field.
    pub fn role_id(mut self, v: impl Into<String>) -> Self {
        self.role_id = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<LoggedBy, BuildError> {
        let party_uuid = self
            .party_uuid
            .ok_or_else(|| BuildError::MissingField("required field `party-uuid` not set"))?;
        Ok(LoggedBy {
            party_uuid,
            role_id: self.role_id,
            remarks: self.remarks,
        })
    }
}
impl LoggedBy {
    /// Return a new builder for this type.
    pub fn builder() -> LoggedByBuilder {
        LoggedByBuilder::new()
    }
}
///Describes the status of the associated risk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiskStatus {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///An individual characteristic that is part of a larger set produced by the same actor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Facet {
    ///The name of the risk metric within the specified system.
    pub name: String,
    ///Specifies the naming system under which this risk metric is organized, which allows for the same names to be used in different systems controlled by different parties. This avoids the potential of a name clash.
    pub system: url::Url,
    ///Indicates the value of the facet.
    pub value: String,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct FacetBuilder {
    name: Option<String>,
    system: Option<url::Url>,
    value: Option<String>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl FacetBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            name: None,
            system: None,
            value: None,
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for FacetBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl FacetBuilder {
    ///Set the `name` field.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    ///Set the `system` field.
    pub fn system(mut self, v: impl Into<url::Url>) -> Self {
        self.system = Some(v.into());
        self
    }
    ///Set the `value` field.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Facet, BuildError> {
        let name = self
            .name
            .ok_or_else(|| BuildError::MissingField("required field `name` not set"))?;
        let system = self
            .system
            .ok_or_else(|| BuildError::MissingField("required field `system` not set"))?;
        let value = self
            .value
            .ok_or_else(|| BuildError::MissingField("required field `value` not set"))?;
        Ok(Facet {
            name,
            system,
            value,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl Facet {
    /// Return a new builder for this type.
    pub fn builder() -> FacetBuilder {
        FacetBuilder::new()
    }
}
///A collection of descriptive data about the containing object from a specific origin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Characterization {
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    pub origin: Origin,
    ///An individual characteristic that is part of a larger set produced by the same actor.
    #[serde(default)]
    pub facet: Vec<Facet>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct CharacterizationBuilder {
    props: Vec<Property>,
    links: Vec<Link>,
    origin: Option<Origin>,
    facet: Vec<Facet>,
}
impl CharacterizationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            props: Vec::new(),
            links: Vec::new(),
            origin: None,
            facet: Vec::new(),
        }
    }
}
impl Default for CharacterizationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl CharacterizationBuilder {
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
        self.origin = Some(v.into());
        self
    }
    ///Set the `facet` field.
    pub fn facet(mut self, v: impl Into<Facet>) -> Self {
        self.facet.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Characterization, BuildError> {
        let origin = self
            .origin
            .ok_or_else(|| BuildError::MissingField("required field `origin` not set"))?;
        Ok(Characterization {
            props: self.props,
            links: self.links,
            origin,
            facet: self.facet,
        })
    }
}
impl Characterization {
    /// Return a new builder for this type.
    pub fn builder() -> CharacterizationBuilder {
        CharacterizationBuilder::new()
    }
}
///Identifies an asset required to achieve remediation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RequiredAsset {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this required asset elsewhere inthis or other OSCAL instances. The locally definedUUIDof theassetcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<SubjectReference>,
    ///The title for this required asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///A human-readable description of this required asset.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct RequiredAssetBuilder {
    uuid: Option<uuid::Uuid>,
    subjects: Vec<SubjectReference>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl RequiredAssetBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            subjects: Vec::new(),
            title: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for RequiredAssetBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RequiredAssetBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `subjects` field.
    pub fn subjects(mut self, v: impl Into<SubjectReference>) -> Self {
        self.subjects.push(v.into());
        self
    }
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    pub fn build(self) -> ::core::result::Result<RequiredAsset, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(RequiredAsset {
            uuid,
            subjects: self.subjects,
            title: self.title,
            description,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl RequiredAsset {
    /// Return a new builder for this type.
    pub fn builder() -> RequiredAssetBuilder {
        RequiredAssetBuilder::new()
    }
}
///Describes either recommended or an actual plan for addressing the risk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Response {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this remediation elsewhere inthis or other OSCAL instances. The locally definedUUIDof therisk responsecan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///Identifies whether this is a recommendation, such as from an assessor or tool, or an actual plan accepted by the system owner.
    pub lifecycle: String,
    ///The title for this response activity.
    pub title: crate::primitives::MarkupLine,
    ///A human-readable description of this response plan.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<Origin>,
    ///Identifies an asset required to achieve remediation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_asset: Vec<RequiredAsset>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<Task>,
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
pub struct ResponseBuilder {
    uuid: Option<uuid::Uuid>,
    lifecycle: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    origins: Vec<Origin>,
    required_asset: Vec<RequiredAsset>,
    tasks: Vec<Task>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ResponseBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            lifecycle: None,
            title: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            origins: Vec::new(),
            required_asset: Vec::new(),
            tasks: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ResponseBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `lifecycle` field.
    pub fn lifecycle(mut self, v: impl Into<String>) -> Self {
        self.lifecycle = Some(v.into());
        self
    }
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `origins` field.
    pub fn origins(mut self, v: impl Into<Origin>) -> Self {
        self.origins.push(v.into());
        self
    }
    ///Set the `required-asset` field.
    pub fn required_asset(mut self, v: impl Into<RequiredAsset>) -> Self {
        self.required_asset.push(v.into());
        self
    }
    ///Set the `tasks` field.
    pub fn tasks(mut self, v: impl Into<Task>) -> Self {
        self.tasks.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Response, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let lifecycle = self
            .lifecycle
            .ok_or_else(|| BuildError::MissingField("required field `lifecycle` not set"))?;
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(Response {
            uuid,
            lifecycle,
            title,
            description,
            props: self.props,
            links: self.links,
            origins: self.origins,
            required_asset: self.required_asset,
            tasks: self.tasks,
            remarks: self.remarks,
        })
    }
}
impl Response {
    /// Return a new builder for this type.
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::new()
    }
}
///A partition of an assessment plan or results or a child of another part.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentPart {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this part elsewhere inthis or other OSCAL instances. The locally definedUUIDof thepartcan be used to reference the data item locally or globally (e.g., in an ported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    ///A textual label that uniquely identifies the part's semantic type.
    pub name: String,
    ///A namespace qualifying the part's name. This allows different organizations to associate distinct semantics with the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<url::Url>,
    ///A textual label that provides a sub-type or characterization of the part'sname. This can be used to further distinguish or discriminate between the semantics of multiple parts of the same control with the samenameandns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    ///A name given to the part, which may be used by a tool for display and navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///Permits multiple paragraphs, lists, tables etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prose: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<AssessmentPart>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct AssessmentPartBuilder {
    uuid: Option<uuid::Uuid>,
    name: Option<String>,
    ns: Option<url::Url>,
    class: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    props: Vec<Property>,
    prose: Option<crate::primitives::MarkupMultiline>,
    parts: Vec<AssessmentPart>,
    links: Vec<Link>,
}
impl AssessmentPartBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            name: None,
            ns: None,
            class: None,
            title: None,
            props: Vec::new(),
            prose: None,
            parts: Vec::new(),
            links: Vec::new(),
        }
    }
}
impl Default for AssessmentPartBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AssessmentPartBuilder {
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
    ///Set the `ns` field.
    pub fn ns(mut self, v: impl Into<url::Url>) -> Self {
        self.ns = Some(v.into());
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
    ///Set the `props` field.
    pub fn props(mut self, v: impl Into<Property>) -> Self {
        self.props.push(v.into());
        self
    }
    ///Set the `prose` field.
    pub fn prose(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.prose = Some(v.into());
        self
    }
    ///Set the `parts` field.
    pub fn parts(mut self, v: impl Into<AssessmentPart>) -> Self {
        self.parts.push(v.into());
        self
    }
    ///Set the `links` field.
    pub fn links(mut self, v: impl Into<Link>) -> Self {
        self.links.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<AssessmentPart, BuildError> {
        let name = self
            .name
            .ok_or_else(|| BuildError::MissingField("required field `name` not set"))?;
        Ok(AssessmentPart {
            uuid: self.uuid,
            name,
            ns: self.ns,
            class: self.class,
            title: self.title,
            props: self.props,
            prose: self.prose,
            parts: self.parts,
            links: self.links,
        })
    }
}
impl AssessmentPart {
    /// Return a new builder for this type.
    pub fn builder() -> AssessmentPartBuilder {
        AssessmentPartBuilder::new()
    }
}
///An annotated, markup-based textual element of a control's or catalog group's definition, or a child of another part.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Part {
    ///A unique identifier for the part.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A textual label that uniquely identifies the part's semantic type, which exists in a value space qualified by thens.
    pub name: String,
    ///An optional namespace qualifying the part'sname. This allows different organizations to associate distinct semantics with the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<url::Url>,
    ///An optional textual providing a sub-type or characterization of the part'sname, or a category to which the part belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    ///An optional name given to the part, which may be used by a tool for display and navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///Permits multiple paragraphs, lists, tables etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prose: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<Part>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct PartBuilder {
    id: Option<String>,
    name: Option<String>,
    ns: Option<url::Url>,
    class: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    props: Vec<Property>,
    prose: Option<crate::primitives::MarkupMultiline>,
    parts: Vec<Part>,
    links: Vec<Link>,
}
impl PartBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            ns: None,
            class: None,
            title: None,
            props: Vec::new(),
            prose: None,
            parts: Vec::new(),
            links: Vec::new(),
        }
    }
}
impl Default for PartBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl PartBuilder {
    ///Set the `id` field.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    ///Set the `name` field.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    ///Set the `ns` field.
    pub fn ns(mut self, v: impl Into<url::Url>) -> Self {
        self.ns = Some(v.into());
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
    ///Set the `props` field.
    pub fn props(mut self, v: impl Into<Property>) -> Self {
        self.props.push(v.into());
        self
    }
    ///Set the `prose` field.
    pub fn prose(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.prose = Some(v.into());
        self
    }
    ///Set the `parts` field.
    pub fn parts(mut self, v: impl Into<Part>) -> Self {
        self.parts.push(v.into());
        self
    }
    ///Set the `links` field.
    pub fn links(mut self, v: impl Into<Link>) -> Self {
        self.links.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Part, BuildError> {
        let name = self
            .name
            .ok_or_else(|| BuildError::MissingField("required field `name` not set"))?;
        Ok(Part {
            id: self.id,
            name,
            ns: self.ns,
            class: self.class,
            title: self.title,
            props: self.props,
            prose: self.prose,
            parts: self.parts,
            links: self.links,
        })
    }
}
impl Part {
    /// Return a new builder for this type.
    pub fn builder() -> PartBuilder {
        PartBuilder::new()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterChoice1 {
    ParameterValue(Vec<String>),
    ParameterSelection(Option<ParameterSelection>),
}
///Parameters provide a mechanism for the dynamic assignment of value(s) in a control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Parameter {
    ///A unique identifier for the parameter.
    pub id: String,
    ///A textual label that provides a characterization of the type, purpose, use or scope of the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    ///(deprecated)Another parameter invoking this one. This construct has been deprecated and should not be used.
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
    pub parameter_choice1: Option<ParameterChoice1>,
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
pub struct ParameterBuilder {
    id: Option<String>,
    class: Option<String>,
    depends_on: Option<String>,
    props: Vec<Property>,
    links: Vec<Link>,
    label: Option<crate::primitives::MarkupLine>,
    usage: Option<crate::primitives::MarkupMultiline>,
    constraints: Vec<ParameterConstraint>,
    guidelines: Vec<ParameterGuideline>,
    parameter_choice1: Option<ParameterChoice1>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ParameterBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            id: None,
            class: None,
            depends_on: None,
            props: Vec::new(),
            links: Vec::new(),
            label: None,
            usage: None,
            constraints: Vec::new(),
            guidelines: Vec::new(),
            parameter_choice1: None,
            remarks: None,
        }
    }
}
impl Default for ParameterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ParameterBuilder {
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
    ///Set the `parameter_choice1` field.
    pub fn parameter_choice1(mut self, v: impl Into<ParameterChoice1>) -> Self {
        self.parameter_choice1 = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Parameter, BuildError> {
        let id = self
            .id
            .ok_or_else(|| BuildError::MissingField("required field `id` not set"))?;
        Ok(Parameter {
            id,
            class: self.class,
            depends_on: self.depends_on,
            props: self.props,
            links: self.links,
            label: self.label,
            usage: self.usage,
            constraints: self.constraints,
            guidelines: self.guidelines,
            parameter_choice1: self.parameter_choice1,
            remarks: self.remarks,
        })
    }
}
impl Parameter {
    /// Return a new builder for this type.
    pub fn builder() -> ParameterBuilder {
        ParameterBuilder::new()
    }
}
///A test expression which is expected to be evaluated by a tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Test {
    ///A formal (executable) expression of a constraint.
    pub expression: String,
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
pub struct TestBuilder {
    expression: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl TestBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            expression: None,
            remarks: None,
        }
    }
}
impl Default for TestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl TestBuilder {
    ///Set the `expression` field.
    pub fn expression(mut self, v: impl Into<String>) -> Self {
        self.expression = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Test, BuildError> {
        let expression = self
            .expression
            .ok_or_else(|| BuildError::MissingField("required field `expression` not set"))?;
        Ok(Test {
            expression,
            remarks: self.remarks,
        })
    }
}
impl Test {
    /// Return a new builder for this type.
    pub fn builder() -> TestBuilder {
        TestBuilder::new()
    }
}
///A formal or informal expression of a constraint or test.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterConstraint {
    ///A textual summary of the constraint to be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///A test expression which is expected to be evaluated by a tool.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test: Vec<Test>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct ParameterConstraintBuilder {
    description: Option<crate::primitives::MarkupMultiline>,
    test: Vec<Test>,
}
impl ParameterConstraintBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            description: None,
            test: Vec::new(),
        }
    }
}
impl Default for ParameterConstraintBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ParameterConstraintBuilder {
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.description = Some(v.into());
        self
    }
    ///Set the `test` field.
    pub fn test(mut self, v: impl Into<Test>) -> Self {
        self.test.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<ParameterConstraint, BuildError> {
        Ok(ParameterConstraint {
            description: self.description,
            test: self.test,
        })
    }
}
impl ParameterConstraint {
    /// Return a new builder for this type.
    pub fn builder() -> ParameterConstraintBuilder {
        ParameterConstraintBuilder::new()
    }
}
///A prose statement that provides a recommendation for the use of a parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterGuideline {
    ///Prose permits multiple paragraphs, lists, tables etc.
    pub prose: crate::primitives::MarkupMultiline,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct ParameterGuidelineBuilder {
    prose: Option<crate::primitives::MarkupMultiline>,
}
impl ParameterGuidelineBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self { prose: None }
    }
}
impl Default for ParameterGuidelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ParameterGuidelineBuilder {
    ///Set the `prose` field.
    pub fn prose(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.prose = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<ParameterGuideline, BuildError> {
        let prose = self
            .prose
            .ok_or_else(|| BuildError::MissingField("required field `prose` not set"))?;
        Ok(ParameterGuideline { prose })
    }
}
impl ParameterGuideline {
    /// Return a new builder for this type.
    pub fn builder() -> ParameterGuidelineBuilder {
        ParameterGuidelineBuilder::new()
    }
}
///A parameter value or set of values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterValue {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///Presenting a choice among alternatives.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterSelection {
    ///Describes the number of selections that must occur. Without this setting, only one value should be assumed to be permitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub how_many: Option<String>,
    ///A value selection among several such options.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choice: Vec<crate::primitives::MarkupLine>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct ParameterSelectionBuilder {
    how_many: Option<String>,
    choice: Vec<crate::primitives::MarkupLine>,
}
impl ParameterSelectionBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            how_many: None,
            choice: Vec::new(),
        }
    }
}
impl Default for ParameterSelectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ParameterSelectionBuilder {
    ///Set the `how-many` field.
    pub fn how_many(mut self, v: impl Into<String>) -> Self {
        self.how_many = Some(v.into());
        self
    }
    ///Set the `choice` field.
    pub fn choice(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.choice.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<ParameterSelection, BuildError> {
        Ok(ParameterSelection {
            how_many: self.how_many,
            choice: self.choice,
        })
    }
}
impl ParameterSelection {
    /// Return a new builder for this type.
    pub fn builder() -> ParameterSelectionBuilder {
        ParameterSelectionBuilder::new()
    }
}
///Include all controls from the imported catalog or profile resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IncludeAll {}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct IncludeAllBuilder {}
impl IncludeAllBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for IncludeAllBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl IncludeAllBuilder {
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<IncludeAll, BuildError> {
        Ok(IncludeAll {})
    }
}
impl IncludeAll {
    /// Return a new builder for this type.
    pub fn builder() -> IncludeAllBuilder {
        IncludeAllBuilder::new()
    }
}
///Selecting a control by its ID given as a literal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WithId {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///Selecting a set of controls by matching their IDs with a wildcard pattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Matching {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
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
pub struct MatchingBuilder {
    pattern: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl MatchingBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            pattern: None,
            remarks: None,
        }
    }
}
impl Default for MatchingBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl MatchingBuilder {
    ///Set the `pattern` field.
    pub fn pattern(mut self, v: impl Into<String>) -> Self {
        self.pattern = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Matching, BuildError> {
        Ok(Matching {
            pattern: self.pattern,
            remarks: self.remarks,
        })
    }
}
impl Matching {
    /// Return a new builder for this type.
    pub fn builder() -> MatchingBuilder {
        MatchingBuilder::new()
    }
}
///An entry in a sequential list of revisions to the containing document, expected to be in reverse chronological order (i.e. latest first).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Revision {
    ///A name given to the document revision, which may be used by a tool for display and navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<chrono::DateTime<chrono::Utc>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    ///
    pub version: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oscal_version: Option<String>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct RevisionBuilder {
    title: Option<crate::primitives::MarkupLine>,
    published: Option<chrono::DateTime<chrono::Utc>>,
    last_modified: Option<chrono::DateTime<chrono::Utc>>,
    version: Option<String>,
    oscal_version: Option<String>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl RevisionBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            title: None,
            published: None,
            last_modified: None,
            version: None,
            oscal_version: None,
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for RevisionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RevisionBuilder {
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `published` field.
    pub fn published(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.published = Some(v.into());
        self
    }
    ///Set the `last-modified` field.
    pub fn last_modified(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.last_modified = Some(v.into());
        self
    }
    ///Set the `version` field.
    pub fn version(mut self, v: impl Into<String>) -> Self {
        self.version = Some(v.into());
        self
    }
    ///Set the `oscal-version` field.
    pub fn oscal_version(mut self, v: impl Into<String>) -> Self {
        self.oscal_version = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Revision, BuildError> {
        let version = self
            .version
            .ok_or_else(|| BuildError::MissingField("required field `version` not set"))?;
        Ok(Revision {
            title: self.title,
            published: self.published,
            last_modified: self.last_modified,
            version,
            oscal_version: self.oscal_version,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl Revision {
    /// Return a new builder for this type.
    pub fn builder() -> RevisionBuilder {
        RevisionBuilder::new()
    }
}
///Defines a function, which might be assigned to a party in a specific situation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Role {
    ///A unique identifier for the role.
    pub id: String,
    ///A name given to the role, which may be used by a tool for display and navigation.
    pub title: crate::primitives::MarkupLine,
    ///A short common name, abbreviation, or acronym for the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    ///A summary of the role's purpose and associated responsibilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct RoleBuilder {
    id: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    short_name: Option<String>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl RoleBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            short_name: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for RoleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RoleBuilder {
    ///Set the `id` field.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `short-name` field.
    pub fn short_name(mut self, v: impl Into<String>) -> Self {
        self.short_name = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    pub fn build(self) -> ::core::result::Result<Role, BuildError> {
        let id = self
            .id
            .ok_or_else(|| BuildError::MissingField("required field `id` not set"))?;
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        Ok(Role {
            id,
            title,
            short_name: self.short_name,
            description: self.description,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl Role {
    /// Return a new builder for this type.
    pub fn builder() -> RoleBuilder {
        RoleBuilder::new()
    }
}
///A physical point of presence, which may be associated with people, organizations, or other concepts within the current or linked OSCAL document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Location {
    ///A unique ID for the location, for reference.
    pub uuid: uuid::Uuid,
    ///A name given to the location, which may be used by a tool for display and navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<String>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telephone_numbers: Vec<TelephoneNumber>,
    ///The uniform resource locator (URL) for a web site or other resource associated with the location.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<url::Url>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct LocationBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    address: Option<Address>,
    email_addresses: Vec<String>,
    telephone_numbers: Vec<TelephoneNumber>,
    url: Vec<url::Url>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl LocationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            address: None,
            email_addresses: Vec::new(),
            telephone_numbers: Vec::new(),
            url: Vec::new(),
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for LocationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl LocationBuilder {
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
    ///Set the `address` field.
    pub fn address(mut self, v: impl Into<Address>) -> Self {
        self.address = Some(v.into());
        self
    }
    ///Set the `email-addresses` field.
    pub fn email_addresses(mut self, v: impl Into<String>) -> Self {
        self.email_addresses.push(v.into());
        self
    }
    ///Set the `telephone-numbers` field.
    pub fn telephone_numbers(mut self, v: impl Into<TelephoneNumber>) -> Self {
        self.telephone_numbers.push(v.into());
        self
    }
    ///Set the `url` field.
    pub fn url(mut self, v: impl Into<url::Url>) -> Self {
        self.url.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Location, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        Ok(Location {
            uuid,
            title: self.title,
            address: self.address,
            email_addresses: self.email_addresses,
            telephone_numbers: self.telephone_numbers,
            url: self.url,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl Location {
    /// Return a new builder for this type.
    pub fn builder() -> LocationBuilder {
        LocationBuilder::new()
    }
}
///An identifier for a person or organization using a designated scheme. e.g. an Open Researcher and Contributor ID (ORCID).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExternalId {
    ///Indicates the type of external identifier.
    pub scheme: url::Url,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartyChoice1 {
    Address(Vec<Address>),
    LocationUuid(Vec<uuid::Uuid>),
}
///An organization or person, which may be associated with roles or other concepts within the current or linked OSCAL document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Party {
    ///A unique identifier for the party.
    pub uuid: uuid::Uuid,
    ///A category describing the kind of party the object describes.
    #[serde(rename = "type")]
    pub type_: String,
    ///The full name of the party. This is typically the legal name associated with the party.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A short common name, abbreviation, or acronym for the party.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    ///An identifier for a person or organization using a designated scheme. e.g. an Open Researcher and Contributor ID (ORCID).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_id: Vec<String>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<String>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telephone_numbers: Vec<TelephoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_choice1: Option<PartyChoice1>,
    ///A reference to anotherpartyby UUID, typically an organization, that this subject is associated with.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member_of_organization: Vec<uuid::Uuid>,
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
pub struct PartyBuilder {
    uuid: Option<uuid::Uuid>,
    type_: Option<String>,
    name: Option<String>,
    short_name: Option<String>,
    external_id: Vec<String>,
    props: Vec<Property>,
    links: Vec<Link>,
    email_addresses: Vec<String>,
    telephone_numbers: Vec<TelephoneNumber>,
    party_choice1: Option<PartyChoice1>,
    member_of_organization: Vec<uuid::Uuid>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl PartyBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            type_: None,
            name: None,
            short_name: None,
            external_id: Vec::new(),
            props: Vec::new(),
            links: Vec::new(),
            email_addresses: Vec::new(),
            telephone_numbers: Vec::new(),
            party_choice1: None,
            member_of_organization: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for PartyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl PartyBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `type` field.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    ///Set the `name` field.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    ///Set the `short-name` field.
    pub fn short_name(mut self, v: impl Into<String>) -> Self {
        self.short_name = Some(v.into());
        self
    }
    ///Set the `external-id` field.
    pub fn external_id(mut self, v: impl Into<String>) -> Self {
        self.external_id.push(v.into());
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
    ///Set the `email-addresses` field.
    pub fn email_addresses(mut self, v: impl Into<String>) -> Self {
        self.email_addresses.push(v.into());
        self
    }
    ///Set the `telephone-numbers` field.
    pub fn telephone_numbers(mut self, v: impl Into<TelephoneNumber>) -> Self {
        self.telephone_numbers.push(v.into());
        self
    }
    ///Set the `party_choice1` field.
    pub fn party_choice1(mut self, v: impl Into<PartyChoice1>) -> Self {
        self.party_choice1 = Some(v.into());
        self
    }
    ///Set the `member-of-organization` field.
    pub fn member_of_organization(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.member_of_organization.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Party, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let type_ = self
            .type_
            .ok_or_else(|| BuildError::MissingField("required field `type` not set"))?;
        Ok(Party {
            uuid,
            type_,
            name: self.name,
            short_name: self.short_name,
            external_id: self.external_id,
            props: self.props,
            links: self.links,
            email_addresses: self.email_addresses,
            telephone_numbers: self.telephone_numbers,
            party_choice1: self.party_choice1,
            member_of_organization: self.member_of_organization,
            remarks: self.remarks,
        })
    }
}
impl Party {
    /// Return a new builder for this type.
    pub fn builder() -> PartyBuilder {
        PartyBuilder::new()
    }
}
///Provides information about the containing document, and defines concepts that are shared across the document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    ///A name given to the document, which may be used by a tool for display and navigation.
    pub title: crate::primitives::MarkupLine,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<chrono::DateTime<chrono::Utc>>,
    ///
    pub last_modified: chrono::DateTime<chrono::Utc>,
    ///
    pub version: String,
    ///
    pub oscal_version: String,
    ///An entry in a sequential list of revisions to the containing document, expected to be in reverse chronological order (i.e. latest first).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub revision: Vec<Revision>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_ids: Vec<DocumentId>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///Defines a function, which might be assigned to a party in a specific situation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<Role>,
    ///A physical point of presence, which may be associated with people, organizations, or other concepts within the current or linked OSCAL document.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<Location>,
    ///An organization or person, which may be associated with roles or other concepts within the current or linked OSCAL document.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party: Vec<Party>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_parties: Vec<ResponsibleParty>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
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
pub struct MetadataBuilder {
    title: Option<crate::primitives::MarkupLine>,
    published: Option<chrono::DateTime<chrono::Utc>>,
    last_modified: Option<chrono::DateTime<chrono::Utc>>,
    version: Option<String>,
    oscal_version: Option<String>,
    revision: Vec<Revision>,
    document_ids: Vec<DocumentId>,
    props: Vec<Property>,
    links: Vec<Link>,
    role: Vec<Role>,
    location: Vec<Location>,
    party: Vec<Party>,
    responsible_parties: Vec<ResponsibleParty>,
    actions: Vec<Action>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl MetadataBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            title: None,
            published: None,
            last_modified: None,
            version: None,
            oscal_version: None,
            revision: Vec::new(),
            document_ids: Vec::new(),
            props: Vec::new(),
            links: Vec::new(),
            role: Vec::new(),
            location: Vec::new(),
            party: Vec::new(),
            responsible_parties: Vec::new(),
            actions: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for MetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl MetadataBuilder {
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `published` field.
    pub fn published(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.published = Some(v.into());
        self
    }
    ///Set the `last-modified` field.
    pub fn last_modified(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.last_modified = Some(v.into());
        self
    }
    ///Set the `version` field.
    pub fn version(mut self, v: impl Into<String>) -> Self {
        self.version = Some(v.into());
        self
    }
    ///Set the `oscal-version` field.
    pub fn oscal_version(mut self, v: impl Into<String>) -> Self {
        self.oscal_version = Some(v.into());
        self
    }
    ///Set the `revision` field.
    pub fn revision(mut self, v: impl Into<Revision>) -> Self {
        self.revision.push(v.into());
        self
    }
    ///Set the `document-ids` field.
    pub fn document_ids(mut self, v: impl Into<DocumentId>) -> Self {
        self.document_ids.push(v.into());
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
    ///Set the `role` field.
    pub fn role(mut self, v: impl Into<Role>) -> Self {
        self.role.push(v.into());
        self
    }
    ///Set the `location` field.
    pub fn location(mut self, v: impl Into<Location>) -> Self {
        self.location.push(v.into());
        self
    }
    ///Set the `party` field.
    pub fn party(mut self, v: impl Into<Party>) -> Self {
        self.party.push(v.into());
        self
    }
    ///Set the `responsible-parties` field.
    pub fn responsible_parties(mut self, v: impl Into<ResponsibleParty>) -> Self {
        self.responsible_parties.push(v.into());
        self
    }
    ///Set the `actions` field.
    pub fn actions(mut self, v: impl Into<Action>) -> Self {
        self.actions.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Metadata, BuildError> {
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        let last_modified = self
            .last_modified
            .ok_or_else(|| BuildError::MissingField("required field `last-modified` not set"))?;
        let version = self
            .version
            .ok_or_else(|| BuildError::MissingField("required field `version` not set"))?;
        let oscal_version = self
            .oscal_version
            .ok_or_else(|| BuildError::MissingField("required field `oscal-version` not set"))?;
        Ok(Metadata {
            title,
            published: self.published,
            last_modified,
            version,
            oscal_version,
            revision: self.revision,
            document_ids: self.document_ids,
            props: self.props,
            links: self.links,
            role: self.role,
            location: self.location,
            party: self.party,
            responsible_parties: self.responsible_parties,
            actions: self.actions,
            remarks: self.remarks,
        })
    }
}
impl Metadata {
    /// Return a new builder for this type.
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::new()
    }
}
///Reference to a location by UUID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocationUuid {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<uuid::Uuid>,
}
///Reference to a party by UUID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PartyUuid {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<uuid::Uuid>,
}
///Reference to a role by UUID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RoleId {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///An optional citation consisting of end note text using structured markup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Citation {
    ///A line of citation text.
    pub text: crate::primitives::MarkupLine,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct CitationBuilder {
    text: Option<crate::primitives::MarkupLine>,
    props: Vec<Property>,
    links: Vec<Link>,
}
impl CitationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            text: None,
            props: Vec::new(),
            links: Vec::new(),
        }
    }
}
impl Default for CitationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl CitationBuilder {
    ///Set the `text` field.
    pub fn text(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.text = Some(v.into());
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
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Citation, BuildError> {
        let text = self
            .text
            .ok_or_else(|| BuildError::MissingField("required field `text` not set"))?;
        Ok(Citation {
            text,
            props: self.props,
            links: self.links,
        })
    }
}
impl Citation {
    /// Return a new builder for this type.
    pub fn builder() -> CitationBuilder {
        CitationBuilder::new()
    }
}
///A URL-based pointer to an external resource with an optional hash for verification and change detection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Rlink {
    ///A resolvable URL pointing to the referenced resource.
    pub href: crate::primitives::UriReference,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hashes: Vec<OscalHash>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct RlinkBuilder {
    href: Option<crate::primitives::UriReference>,
    media_type: Option<String>,
    hashes: Vec<OscalHash>,
}
impl RlinkBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            href: None,
            media_type: None,
            hashes: Vec::new(),
        }
    }
}
impl Default for RlinkBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl RlinkBuilder {
    ///Set the `href` field.
    pub fn href(mut self, v: impl Into<crate::primitives::UriReference>) -> Self {
        self.href = Some(v.into());
        self
    }
    ///Set the `media-type` field.
    pub fn media_type(mut self, v: impl Into<String>) -> Self {
        self.media_type = Some(v.into());
        self
    }
    ///Set the `hashes` field.
    pub fn hashes(mut self, v: impl Into<OscalHash>) -> Self {
        self.hashes.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Rlink, BuildError> {
        let href = self
            .href
            .ok_or_else(|| BuildError::MissingField("required field `href` not set"))?;
        Ok(Rlink {
            href,
            media_type: self.media_type,
            hashes: self.hashes,
        })
    }
}
impl Rlink {
    /// Return a new builder for this type.
    pub fn builder() -> RlinkBuilder {
        RlinkBuilder::new()
    }
}
///A resource encoded using the Base64 alphabet defined byRFC 2045.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Base64 {
    ///Name of the file before it was encoded as Base64 to be embedded in aresource. This is the name that will be assigned to the file when the file is decoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///A resource associated with content in the containing document instance. A resource may be directly included in the document using base64 encoding or may point to one or more equivalent internet resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Resource {
    ///A unique identifier for a resource.
    pub uuid: uuid::Uuid,
    ///An optional name given to the resource, which may be used by a tool for display and navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///An optional short summary of the resource used to indicate the purpose of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_ids: Vec<DocumentId>,
    ///An optional citation consisting of end note text using structured markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation: Option<Citation>,
    ///A URL-based pointer to an external resource with an optional hash for verification and change detection.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rlink: Vec<Rlink>,
    ///A resource encoded using the Base64 alphabet defined byRFC 2045.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
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
pub struct ResourceBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    document_ids: Vec<DocumentId>,
    citation: Option<Citation>,
    rlink: Vec<Rlink>,
    base64: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ResourceBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            props: Vec::new(),
            document_ids: Vec::new(),
            citation: None,
            rlink: Vec::new(),
            base64: None,
            remarks: None,
        }
    }
}
impl Default for ResourceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ResourceBuilder {
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.description = Some(v.into());
        self
    }
    ///Set the `props` field.
    pub fn props(mut self, v: impl Into<Property>) -> Self {
        self.props.push(v.into());
        self
    }
    ///Set the `document-ids` field.
    pub fn document_ids(mut self, v: impl Into<DocumentId>) -> Self {
        self.document_ids.push(v.into());
        self
    }
    ///Set the `citation` field.
    pub fn citation(mut self, v: impl Into<Citation>) -> Self {
        self.citation = Some(v.into());
        self
    }
    ///Set the `rlink` field.
    pub fn rlink(mut self, v: impl Into<Rlink>) -> Self {
        self.rlink.push(v.into());
        self
    }
    ///Set the `base64` field.
    pub fn base64(mut self, v: impl Into<String>) -> Self {
        self.base64 = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Resource, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        Ok(Resource {
            uuid,
            title: self.title,
            description: self.description,
            props: self.props,
            document_ids: self.document_ids,
            citation: self.citation,
            rlink: self.rlink,
            base64: self.base64,
            remarks: self.remarks,
        })
    }
}
impl Resource {
    /// Return a new builder for this type.
    pub fn builder() -> ResourceBuilder {
        ResourceBuilder::new()
    }
}
///A collection of resources that may be referenced from within the OSCAL document instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackMatter {
    ///A resource associated with content in the containing document instance. A resource may be directly included in the document using base64 encoding or may point to one or more equivalent internet resources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource: Vec<Resource>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct BackMatterBuilder {
    resource: Vec<Resource>,
}
impl BackMatterBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            resource: Vec::new(),
        }
    }
}
impl Default for BackMatterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl BackMatterBuilder {
    ///Set the `resource` field.
    pub fn resource(mut self, v: impl Into<Resource>) -> Self {
        self.resource.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<BackMatter, BuildError> {
        Ok(BackMatter {
            resource: self.resource,
        })
    }
}
impl BackMatter {
    /// Return a new builder for this type.
    pub fn builder() -> BackMatterBuilder {
        BackMatterBuilder::new()
    }
}
///An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Property {
    ///A textual label, within a namespace, that identifies a specific attribute, characteristic, or quality of the property's containing object.
    pub name: String,
    ///A unique identifier for a property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    ///A namespace qualifying the property's name. This allows different organizations to associate distinct semantics with the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<url::Url>,
    ///Indicates the value of the attribute, characteristic, or quality.
    pub value: String,
    ///A textual label that provides a sub-type or characterization of the property'sname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    ///An identifier for relating distinct sets of properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
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
pub struct PropertyBuilder {
    name: Option<String>,
    uuid: Option<uuid::Uuid>,
    ns: Option<url::Url>,
    value: Option<String>,
    class: Option<String>,
    group: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl PropertyBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            name: None,
            uuid: None,
            ns: None,
            value: None,
            class: None,
            group: None,
            remarks: None,
        }
    }
}
impl Default for PropertyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl PropertyBuilder {
    ///Set the `name` field.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `ns` field.
    pub fn ns(mut self, v: impl Into<url::Url>) -> Self {
        self.ns = Some(v.into());
        self
    }
    ///Set the `value` field.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    ///Set the `class` field.
    pub fn class(mut self, v: impl Into<String>) -> Self {
        self.class = Some(v.into());
        self
    }
    ///Set the `group` field.
    pub fn group(mut self, v: impl Into<String>) -> Self {
        self.group = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Property, BuildError> {
        let name = self
            .name
            .ok_or_else(|| BuildError::MissingField("required field `name` not set"))?;
        let value = self
            .value
            .ok_or_else(|| BuildError::MissingField("required field `value` not set"))?;
        Ok(Property {
            name,
            uuid: self.uuid,
            ns: self.ns,
            value,
            class: self.class,
            group: self.group,
            remarks: self.remarks,
        })
    }
}
impl Property {
    /// Return a new builder for this type.
    pub fn builder() -> PropertyBuilder {
        PropertyBuilder::new()
    }
}
///A reference to a local or remote resource, that has a specific relation to the containing object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Link {
    ///A resolvable URL reference to a resource.
    pub href: crate::primitives::UriReference,
    ///Describes the type of relationship provided by the link's hypertext reference. This can be an indicator of the link's purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    ///In case where thehrefpoints to aback-matter/resource, this value will indicate the URIfragmentto append to anyrlinkassociated with the resource. This value MUST beURI encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_fragment: Option<String>,
    ///A textual label to associate with the link, which may be used for presentation in a tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<crate::primitives::MarkupLine>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct LinkBuilder {
    href: Option<crate::primitives::UriReference>,
    rel: Option<String>,
    resource_fragment: Option<String>,
    text: Option<crate::primitives::MarkupLine>,
}
impl LinkBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            href: None,
            rel: None,
            resource_fragment: None,
            text: None,
        }
    }
}
impl Default for LinkBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl LinkBuilder {
    ///Set the `href` field.
    pub fn href(mut self, v: impl Into<crate::primitives::UriReference>) -> Self {
        self.href = Some(v.into());
        self
    }
    ///Set the `rel` field.
    pub fn rel(mut self, v: impl Into<String>) -> Self {
        self.rel = Some(v.into());
        self
    }
    ///Set the `resource-fragment` field.
    pub fn resource_fragment(mut self, v: impl Into<String>) -> Self {
        self.resource_fragment = Some(v.into());
        self
    }
    ///Set the `text` field.
    pub fn text(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.text = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Link, BuildError> {
        let href = self
            .href
            .ok_or_else(|| BuildError::MissingField("required field `href` not set"))?;
        Ok(Link {
            href,
            rel: self.rel,
            resource_fragment: self.resource_fragment,
            text: self.text,
        })
    }
}
impl Link {
    /// Return a new builder for this type.
    pub fn builder() -> LinkBuilder {
        LinkBuilder::new()
    }
}
///A reference to a set of persons and/or organizations that have responsibility for performing the referenced role in the context of the containing object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResponsibleParty {
    ///A reference to aroleperformed by aparty.
    pub role_id: String,
    ///
    #[serde(default)]
    pub party_uuids: Vec<uuid::Uuid>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
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
pub struct ResponsiblePartyBuilder {
    role_id: Option<String>,
    party_uuids: Vec<uuid::Uuid>,
    props: Vec<Property>,
    links: Vec<Link>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ResponsiblePartyBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            role_id: None,
            party_uuids: Vec::new(),
            props: Vec::new(),
            links: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ResponsiblePartyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ResponsiblePartyBuilder {
    ///Set the `role-id` field.
    pub fn role_id(mut self, v: impl Into<String>) -> Self {
        self.role_id = Some(v.into());
        self
    }
    ///Set the `party-uuids` field.
    pub fn party_uuids(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.party_uuids.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<ResponsibleParty, BuildError> {
        let role_id = self
            .role_id
            .ok_or_else(|| BuildError::MissingField("required field `role-id` not set"))?;
        Ok(ResponsibleParty {
            role_id,
            party_uuids: self.party_uuids,
            props: self.props,
            links: self.links,
            remarks: self.remarks,
        })
    }
}
impl ResponsibleParty {
    /// Return a new builder for this type.
    pub fn builder() -> ResponsiblePartyBuilder {
        ResponsiblePartyBuilder::new()
    }
}
///An action applied by a role within a given party to the content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Action {
    ///A unique identifier that can be used to reference this defined action elsewhere in an OSCAL document. A UUID should be consistently used for a given location across revisions of the document.
    pub uuid: uuid::Uuid,
    ///The date and time when the action occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    ///The type of action documented by the assembly, such as an approval.
    #[serde(rename = "type")]
    pub type_: String,
    ///Specifies the action type system used.
    pub system: url::Url,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_parties: Vec<ResponsibleParty>,
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
pub struct ActionBuilder {
    uuid: Option<uuid::Uuid>,
    date: Option<chrono::DateTime<chrono::Utc>>,
    type_: Option<String>,
    system: Option<url::Url>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_parties: Vec<ResponsibleParty>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ActionBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            date: None,
            type_: None,
            system: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_parties: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ActionBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `date` field.
    pub fn date(mut self, v: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.date = Some(v.into());
        self
    }
    ///Set the `type` field.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    ///Set the `system` field.
    pub fn system(mut self, v: impl Into<url::Url>) -> Self {
        self.system = Some(v.into());
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
    ///Set the `responsible-parties` field.
    pub fn responsible_parties(mut self, v: impl Into<ResponsibleParty>) -> Self {
        self.responsible_parties.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Action, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let type_ = self
            .type_
            .ok_or_else(|| BuildError::MissingField("required field `type` not set"))?;
        let system = self
            .system
            .ok_or_else(|| BuildError::MissingField("required field `system` not set"))?;
        Ok(Action {
            uuid,
            date: self.date,
            type_,
            system,
            props: self.props,
            links: self.links,
            responsible_parties: self.responsible_parties,
            remarks: self.remarks,
        })
    }
}
impl Action {
    /// Return a new builder for this type.
    pub fn builder() -> ActionBuilder {
        ActionBuilder::new()
    }
}
///A reference to a role with responsibility for performing a function relative to the containing object, optionally associated with a set of persons and/or organizations that perform that role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResponsibleRole {
    ///Ahuman-orientedidentifier reference to aroleperformed.
    pub role_id: String,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party_uuids: Vec<uuid::Uuid>,
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
pub struct ResponsibleRoleBuilder {
    role_id: Option<String>,
    props: Vec<Property>,
    links: Vec<Link>,
    party_uuids: Vec<uuid::Uuid>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ResponsibleRoleBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            role_id: None,
            props: Vec::new(),
            links: Vec::new(),
            party_uuids: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ResponsibleRoleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ResponsibleRoleBuilder {
    ///Set the `role-id` field.
    pub fn role_id(mut self, v: impl Into<String>) -> Self {
        self.role_id = Some(v.into());
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
    ///Set the `party-uuids` field.
    pub fn party_uuids(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.party_uuids.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<ResponsibleRole, BuildError> {
        let role_id = self
            .role_id
            .ok_or_else(|| BuildError::MissingField("required field `role-id` not set"))?;
        Ok(ResponsibleRole {
            role_id,
            props: self.props,
            links: self.links,
            party_uuids: self.party_uuids,
            remarks: self.remarks,
        })
    }
}
impl ResponsibleRole {
    /// Return a new builder for this type.
    pub fn builder() -> ResponsibleRoleBuilder {
        ResponsibleRoleBuilder::new()
    }
}
///A representation of a cryptographic digest generated over a resource using a specified hash algorithm.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OscalHash {
    ///The digest method by which a hash is derived.
    pub algorithm: String,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///Additional commentary about the containing object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Remarks {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<crate::primitives::MarkupMultiline>,
}
///The date and time the document was last made available.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Published {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<chrono::DateTime<chrono::Utc>>,
}
///The date and time the document was last stored for later retrieval.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LastModified {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<chrono::DateTime<chrono::Utc>>,
}
///Used to distinguish a specific revision of an OSCAL document from other previous and future versions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Version {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///The OSCAL model version the document was authored against and will conform to as valid.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OscalVersion {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///An email address as defined byRFC 5322 Section 3.4.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EmailAddress {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///A telephone service number as defined byITU-T E.164.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TelephoneNumber {
    ///Indicates the type of phone number.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///A postal address for the location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Address {
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addr_lines: Vec<String>,
    ///City, town or geographical region for the mailing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///State, province or analogous geographical region for a mailing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    ///Postal or ZIP code for mailing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code for the mailing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct AddressBuilder {
    addr_lines: Vec<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
}
impl AddressBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            addr_lines: Vec::new(),
            city: None,
            state: None,
            postal_code: None,
            country: None,
        }
    }
}
impl Default for AddressBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AddressBuilder {
    ///Set the `addr-lines` field.
    pub fn addr_lines(mut self, v: impl Into<String>) -> Self {
        self.addr_lines.push(v.into());
        self
    }
    ///Set the `city` field.
    pub fn city(mut self, v: impl Into<String>) -> Self {
        self.city = Some(v.into());
        self
    }
    ///Set the `state` field.
    pub fn state(mut self, v: impl Into<String>) -> Self {
        self.state = Some(v.into());
        self
    }
    ///Set the `postal-code` field.
    pub fn postal_code(mut self, v: impl Into<String>) -> Self {
        self.postal_code = Some(v.into());
        self
    }
    ///Set the `country` field.
    pub fn country(mut self, v: impl Into<String>) -> Self {
        self.country = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Address, BuildError> {
        Ok(Address {
            addr_lines: self.addr_lines,
            city: self.city,
            state: self.state,
            postal_code: self.postal_code,
            country: self.country,
        })
    }
}
impl Address {
    /// Return a new builder for this type.
    pub fn builder() -> AddressBuilder {
        AddressBuilder::new()
    }
}
///A single line of an address.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddrLine {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///A document identifier qualified by an identifierscheme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DocumentId {
    ///Qualifies the kind of document identifier using a URI. If the scheme is not provided the value of the element will be interpreted as a string of characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<url::Url>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///A defined component that can be part of an implemented system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemComponent {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this component elsewhere inthis or other OSCAL instances. The locally definedUUIDof thecomponentcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///A human readable name for the system component.
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
    ///Describes the operational status of the system component.
    pub status: Status,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_roles: Vec<ResponsibleRole>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocols: Vec<Protocol>,
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
pub struct SystemComponentBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    purpose: Option<crate::primitives::MarkupLine>,
    props: Vec<Property>,
    links: Vec<Link>,
    status: Option<Status>,
    responsible_roles: Vec<ResponsibleRole>,
    protocols: Vec<Protocol>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SystemComponentBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            purpose: None,
            props: Vec::new(),
            links: Vec::new(),
            status: None,
            responsible_roles: Vec::new(),
            protocols: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for SystemComponentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SystemComponentBuilder {
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `status` field.
    pub fn status(mut self, v: impl Into<Status>) -> Self {
        self.status = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<SystemComponent, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        let status = self
            .status
            .ok_or_else(|| BuildError::MissingField("required field `status` not set"))?;
        Ok(SystemComponent {
            uuid,
            title,
            description,
            purpose: self.purpose,
            props: self.props,
            links: self.links,
            status,
            responsible_roles: self.responsible_roles,
            protocols: self.protocols,
            remarks: self.remarks,
        })
    }
}
impl SystemComponent {
    /// Return a new builder for this type.
    pub fn builder() -> SystemComponentBuilder {
        SystemComponentBuilder::new()
    }
}
///Information about the protocol used to provide a service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Protocol {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this service protocol information elsewhere inthis or other OSCAL instances. The locally definedUUIDof theservice protocolcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    ///The common name of the protocol, which should be the appropriate "service name" from theIANA Service Name and Transport Protocol Port Number Registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A human readable name for the protocol (e.g., Transport Layer Security).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub port_ranges: Vec<PortRange>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct ProtocolBuilder {
    uuid: Option<uuid::Uuid>,
    name: Option<String>,
    title: Option<crate::primitives::MarkupLine>,
    port_ranges: Vec<PortRange>,
}
impl ProtocolBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            name: None,
            title: None,
            port_ranges: Vec::new(),
        }
    }
}
impl Default for ProtocolBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ProtocolBuilder {
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
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `port-ranges` field.
    pub fn port_ranges(mut self, v: impl Into<PortRange>) -> Self {
        self.port_ranges.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Protocol, BuildError> {
        Ok(Protocol {
            uuid: self.uuid,
            name: self.name,
            title: self.title,
            port_ranges: self.port_ranges,
        })
    }
}
impl Protocol {
    /// Return a new builder for this type.
    pub fn builder() -> ProtocolBuilder {
        ProtocolBuilder::new()
    }
}
///Where applicable this is the transport layer protocol port range an IPv4-based or IPv6-based service uses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PortRange {
    ///Indicates the starting port number in a port range for a transport layer protocol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    ///Indicates the ending port number in a port range for a transport layer protocol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    ///Indicates the transport type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
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
pub struct PortRangeBuilder {
    start: Option<u64>,
    end: Option<u64>,
    transport: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl PortRangeBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
            transport: None,
            remarks: None,
        }
    }
}
impl Default for PortRangeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl PortRangeBuilder {
    ///Set the `start` field.
    pub fn start(mut self, v: impl Into<u64>) -> Self {
        self.start = Some(v.into());
        self
    }
    ///Set the `end` field.
    pub fn end(mut self, v: impl Into<u64>) -> Self {
        self.end = Some(v.into());
        self
    }
    ///Set the `transport` field.
    pub fn transport(mut self, v: impl Into<String>) -> Self {
        self.transport = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<PortRange, BuildError> {
        Ok(PortRange {
            start: self.start,
            end: self.end,
            transport: self.transport,
            remarks: self.remarks,
        })
    }
}
impl PortRange {
    /// Return a new builder for this type.
    pub fn builder() -> PortRangeBuilder {
        PortRangeBuilder::new()
    }
}
///Indicates the degree to which the a given control is implemented.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementationStatus {
    ///Identifies the implementation status of the control or control objective.
    pub state: String,
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
pub struct ImplementationStatusBuilder {
    state: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ImplementationStatusBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            state: None,
            remarks: None,
        }
    }
}
impl Default for ImplementationStatusBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ImplementationStatusBuilder {
    ///Set the `state` field.
    pub fn state(mut self, v: impl Into<String>) -> Self {
        self.state = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<ImplementationStatus, BuildError> {
        let state = self
            .state
            .ok_or_else(|| BuildError::MissingField("required field `state` not set"))?;
        Ok(ImplementationStatus {
            state,
            remarks: self.remarks,
        })
    }
}
impl ImplementationStatus {
    /// Return a new builder for this type.
    pub fn builder() -> ImplementationStatusBuilder {
        ImplementationStatusBuilder::new()
    }
}
///A type of user that interacts with the system based on an associated role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemUser {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this user class elsewhere inthis or other OSCAL instances. The locally definedUUIDof thesystem usercan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///A name given to the user, which may be used by a tool for display and navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::primitives::MarkupLine>,
    ///A short common name, abbreviation, or acronym for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    ///A summary of the user's purpose within the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_ids: Vec<String>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorized_privileges: Vec<AuthorizedPrivilege>,
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
pub struct SystemUserBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    short_name: Option<String>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    role_ids: Vec<String>,
    authorized_privileges: Vec<AuthorizedPrivilege>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SystemUserBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            short_name: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            role_ids: Vec::new(),
            authorized_privileges: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for SystemUserBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SystemUserBuilder {
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
    ///Set the `short-name` field.
    pub fn short_name(mut self, v: impl Into<String>) -> Self {
        self.short_name = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `role-ids` field.
    pub fn role_ids(mut self, v: impl Into<String>) -> Self {
        self.role_ids.push(v.into());
        self
    }
    ///Set the `authorized-privileges` field.
    pub fn authorized_privileges(mut self, v: impl Into<AuthorizedPrivilege>) -> Self {
        self.authorized_privileges.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<SystemUser, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        Ok(SystemUser {
            uuid,
            title: self.title,
            short_name: self.short_name,
            description: self.description,
            props: self.props,
            links: self.links,
            role_ids: self.role_ids,
            authorized_privileges: self.authorized_privileges,
            remarks: self.remarks,
        })
    }
}
impl SystemUser {
    /// Return a new builder for this type.
    pub fn builder() -> SystemUserBuilder {
        SystemUserBuilder::new()
    }
}
///Identifies a specific system privilege held by the user, along with an associated description and/or rationale for the privilege.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AuthorizedPrivilege {
    ///A human readable name for the privilege.
    pub title: crate::primitives::MarkupLine,
    ///A summary of the privilege's purpose within the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default)]
    pub functions_performed: Vec<String>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct AuthorizedPrivilegeBuilder {
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    functions_performed: Vec<String>,
}
impl AuthorizedPrivilegeBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            functions_performed: Vec::new(),
        }
    }
}
impl Default for AuthorizedPrivilegeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AuthorizedPrivilegeBuilder {
    ///Set the `title` field.
    pub fn title(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.title = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.description = Some(v.into());
        self
    }
    ///Set the `functions-performed` field.
    pub fn functions_performed(mut self, v: impl Into<String>) -> Self {
        self.functions_performed.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<AuthorizedPrivilege, BuildError> {
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        Ok(AuthorizedPrivilege {
            title,
            description: self.description,
            functions_performed: self.functions_performed,
        })
    }
}
impl AuthorizedPrivilege {
    /// Return a new builder for this type.
    pub fn builder() -> AuthorizedPrivilegeBuilder {
        AuthorizedPrivilegeBuilder::new()
    }
}
///Describes a function performed for a given authorized privilege by this user class.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FunctionPerformed {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///The set of components that are implemented in a given system inventory item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementedComponent {
    ///Amachine-orientedidentifier reference to acomponentthat is implemented as part of an inventory item.
    pub component_uuid: uuid::Uuid,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_parties: Vec<ResponsibleParty>,
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
pub struct ImplementedComponentBuilder {
    component_uuid: Option<uuid::Uuid>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_parties: Vec<ResponsibleParty>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ImplementedComponentBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            component_uuid: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_parties: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ImplementedComponentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ImplementedComponentBuilder {
    ///Set the `component-uuid` field.
    pub fn component_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.component_uuid = Some(v.into());
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
    ///Set the `responsible-parties` field.
    pub fn responsible_parties(mut self, v: impl Into<ResponsibleParty>) -> Self {
        self.responsible_parties.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<ImplementedComponent, BuildError> {
        let component_uuid = self
            .component_uuid
            .ok_or_else(|| BuildError::MissingField("required field `component-uuid` not set"))?;
        Ok(ImplementedComponent {
            component_uuid,
            props: self.props,
            links: self.links,
            responsible_parties: self.responsible_parties,
            remarks: self.remarks,
        })
    }
}
impl ImplementedComponent {
    /// Return a new builder for this type.
    pub fn builder() -> ImplementedComponentBuilder {
        ImplementedComponentBuilder::new()
    }
}
///A single managed inventory item within the system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InventoryItem {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this inventory item elsewhere inthis or other OSCAL instances. The locally definedUUIDof theinventory itemcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///A summary of the inventory item stating its purpose within the system.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_parties: Vec<ResponsibleParty>,
    ///The set of components that are implemented in a given system inventory item.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implemented_component: Vec<ImplementedComponent>,
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
pub struct InventoryItemBuilder {
    uuid: Option<uuid::Uuid>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_parties: Vec<ResponsibleParty>,
    implemented_component: Vec<ImplementedComponent>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl InventoryItemBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_parties: Vec::new(),
            implemented_component: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for InventoryItemBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl InventoryItemBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `responsible-parties` field.
    pub fn responsible_parties(mut self, v: impl Into<ResponsibleParty>) -> Self {
        self.responsible_parties.push(v.into());
        self
    }
    ///Set the `implemented-component` field.
    pub fn implemented_component(mut self, v: impl Into<ImplementedComponent>) -> Self {
        self.implemented_component.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<InventoryItem, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(InventoryItem {
            uuid,
            description,
            props: self.props,
            links: self.links,
            responsible_parties: self.responsible_parties,
            implemented_component: self.implemented_component,
            remarks: self.remarks,
        })
    }
}
impl InventoryItem {
    /// Return a new builder for this type.
    pub fn builder() -> InventoryItemBuilder {
        InventoryItemBuilder::new()
    }
}
///Identifies the parameter that will be set by the enclosed value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SetParameter {
    ///
    pub param_id: String,
    ///A parameter value or set of values.
    #[serde(default)]
    pub value: Vec<String>,
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
pub struct SetParameterBuilder {
    param_id: Option<String>,
    value: Vec<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SetParameterBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            param_id: None,
            value: Vec::new(),
            remarks: None,
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
    ///Set the `value` field.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<SetParameter, BuildError> {
        let param_id = self
            .param_id
            .ok_or_else(|| BuildError::MissingField("required field `param-id` not set"))?;
        Ok(SetParameter {
            param_id,
            value: self.value,
            remarks: self.remarks,
        })
    }
}
impl SetParameter {
    /// Return a new builder for this type.
    pub fn builder() -> SetParameterBuilder {
        SetParameterBuilder::new()
    }
}
///Ahuman-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this system identification property elsewhere inthis or other OSCAL instances. When referencing an externally definedsystem identification, thesystem identificationmust be used in the context of the external / imported OSCAL instance (e.g., uri-reference). This string should be assignedper-subject, which means it should be consistently used to identify the same system across revisions of the document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemId {
    ///Identifies the identification system from which the provided identifier was assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_type: Option<url::Url>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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
        Ok(TermsAndConditions { parts: self.parts })
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
            .ok_or_else(|| BuildError::MissingField("required field `metadata` not set"))?;
        let import_ssp = self
            .import_ssp
            .ok_or_else(|| BuildError::MissingField("required field `import-ssp` not set"))?;
        let reviewed_controls = self.reviewed_controls.ok_or_else(|| {
            BuildError::MissingField("required field `reviewed-controls` not set")
        })?;
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
            .ok_or_else(|| BuildError::MissingField("required field `metadata` not set"))?;
        let import_ap = self
            .import_ap
            .ok_or_else(|| BuildError::MissingField("required field `import-ap` not set"))?;
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        let start = self
            .start
            .ok_or_else(|| BuildError::MissingField("required field `start` not set"))?;
        let reviewed_controls = self.reviewed_controls.ok_or_else(|| {
            BuildError::MissingField("required field `reviewed-controls` not set")
        })?;
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
        Self {
            href: None,
            remarks: None,
        }
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
            .ok_or_else(|| BuildError::MissingField("required field `metadata` not set"))?;
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
    ///Nested sub-groups within this group.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Group>,
    ///Controls within this group.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub controls: Vec<Control>,
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
    groups: Vec<Group>,
    controls: Vec<Control>,
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
            groups: Vec::new(),
            controls: Vec::new(),
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
    ///Add a sub-group.
    pub fn groups(mut self, v: impl Into<Group>) -> Self {
        self.groups.push(v.into());
        self
    }
    ///Add a control.
    pub fn controls(mut self, v: impl Into<Control>) -> Self {
        self.controls.push(v.into());
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
            groups: self.groups,
            controls: self.controls,
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
            .ok_or_else(|| BuildError::MissingField("required field `metadata` not set"))?;
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
        Ok(Combine {
            method: self.method,
        })
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
            .ok_or_else(|| BuildError::MissingField("required field `control-id` not set"))?;
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
    pub fn insert_controls_choice1(mut self, v: impl Into<InsertControlsChoice1>) -> Self {
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
    pub fn import_component_definitions(mut self, v: impl Into<ImportComponentDefinition>) -> Self {
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
            .ok_or_else(|| BuildError::MissingField("required field `metadata` not set"))?;
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
        Self {
            href: None,
            remarks: None,
        }
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    pub fn control_implementations(mut self, v: impl Into<ControlImplementation>) -> Self {
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
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    pub fn incorporates_components(mut self, v: impl Into<IncorporatesComponent>) -> Self {
        self.incorporates_components.push(v.into());
        self
    }
    ///Set the `control-implementations` field.
    pub fn control_implementations(mut self, v: impl Into<ControlImplementation>) -> Self {
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
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
            .ok_or_else(|| BuildError::MissingField("required field `component-uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    pub fn implemented_requirements(mut self, v: impl Into<ImplementedRequirement>) -> Self {
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
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
            .ok_or_else(|| BuildError::MissingField("required field `control-id` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
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
///A system security plan, such as those described in NIST SP 800-18.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemSecurityPlan {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this system security plan (SSP) elsewhere inthis or other OSCAL instances. The locally definedUUIDof theSSPcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance).This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///
    pub metadata: Metadata,
    ///
    pub import_profile: ImportProfile,
    ///
    pub system_characteristics: SystemCharacteristics,
    ///
    pub system_implementation: SystemImplementation,
    ///
    pub control_implementation: ControlImplementation,
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
pub struct SystemSecurityPlanBuilder {
    uuid: Option<uuid::Uuid>,
    metadata: Option<Metadata>,
    import_profile: Option<ImportProfile>,
    system_characteristics: Option<SystemCharacteristics>,
    system_implementation: Option<SystemImplementation>,
    control_implementation: Option<ControlImplementation>,
    back_matter: Option<BackMatter>,
}
impl SystemSecurityPlanBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            metadata: None,
            import_profile: None,
            system_characteristics: None,
            system_implementation: None,
            control_implementation: None,
            back_matter: None,
        }
    }
}
impl Default for SystemSecurityPlanBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SystemSecurityPlanBuilder {
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
    ///Set the `import-profile` field.
    pub fn import_profile(mut self, v: impl Into<ImportProfile>) -> Self {
        self.import_profile = Some(v.into());
        self
    }
    ///Set the `system-characteristics` field.
    pub fn system_characteristics(mut self, v: impl Into<SystemCharacteristics>) -> Self {
        self.system_characteristics = Some(v.into());
        self
    }
    ///Set the `system-implementation` field.
    pub fn system_implementation(mut self, v: impl Into<SystemImplementation>) -> Self {
        self.system_implementation = Some(v.into());
        self
    }
    ///Set the `control-implementation` field.
    pub fn control_implementation(mut self, v: impl Into<ControlImplementation>) -> Self {
        self.control_implementation = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<SystemSecurityPlan, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let metadata = self
            .metadata
            .ok_or_else(|| BuildError::MissingField("required field `metadata` not set"))?;
        let import_profile = self
            .import_profile
            .ok_or_else(|| BuildError::MissingField("required field `import-profile` not set"))?;
        let system_characteristics = self.system_characteristics.ok_or_else(|| {
            BuildError::MissingField("required field `system-characteristics` not set")
        })?;
        let system_implementation = self.system_implementation.ok_or_else(|| {
            BuildError::MissingField("required field `system-implementation` not set")
        })?;
        let control_implementation = self.control_implementation.ok_or_else(|| {
            BuildError::MissingField("required field `control-implementation` not set")
        })?;
        Ok(SystemSecurityPlan {
            uuid,
            metadata,
            import_profile,
            system_characteristics,
            system_implementation,
            control_implementation,
            back_matter: self.back_matter,
        })
    }
}
impl SystemSecurityPlan {
    /// Return a new builder for this type.
    pub fn builder() -> SystemSecurityPlanBuilder {
        SystemSecurityPlanBuilder::new()
    }
}
///Used to import the OSCAL profile representing the system's control baseline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportProfile {
    ///A resolvable URL reference to the profile or catalog to use as the system's control baseline.
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
pub struct ImportProfileBuilder {
    href: Option<crate::primitives::UriReference>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ImportProfileBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            href: None,
            remarks: None,
        }
    }
}
impl Default for ImportProfileBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ImportProfileBuilder {
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
    pub fn build(self) -> ::core::result::Result<ImportProfile, BuildError> {
        let href = self
            .href
            .ok_or_else(|| BuildError::MissingField("required field `href` not set"))?;
        Ok(ImportProfile {
            href,
            remarks: self.remarks,
        })
    }
}
impl ImportProfile {
    /// Return a new builder for this type.
    pub fn builder() -> ImportProfileBuilder {
        ImportProfileBuilder::new()
    }
}
///Contains the characteristics of the system, such as its name, purpose, and security impact level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemCharacteristics {
    ///
    #[serde(default)]
    pub system_ids: Vec<SystemId>,
    ///The full name of the system.
    pub system_name: String,
    ///A short name for the system, such as an acronym, that is suitable for display in a data table or summary list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_name_short: Option<String>,
    ///A summary of the system.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_authorized: Option<chrono::NaiveDate>,
    ///The overall information system sensitivity categorization, such as defined byFIPS-199.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_sensitivity_level: Option<String>,
    ///
    pub system_information: SystemInformation,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_impact_level: Option<SecurityImpactLevel>,
    ///
    pub status: Status,
    ///
    pub authorization_boundary: AuthorizationBoundary,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_architecture: Option<NetworkArchitecture>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow: Option<DataFlow>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_parties: Vec<ResponsibleParty>,
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
pub struct SystemCharacteristicsBuilder {
    system_ids: Vec<SystemId>,
    system_name: Option<String>,
    system_name_short: Option<String>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    date_authorized: Option<chrono::NaiveDate>,
    security_sensitivity_level: Option<String>,
    system_information: Option<SystemInformation>,
    security_impact_level: Option<SecurityImpactLevel>,
    status: Option<Status>,
    authorization_boundary: Option<AuthorizationBoundary>,
    network_architecture: Option<NetworkArchitecture>,
    data_flow: Option<DataFlow>,
    responsible_parties: Vec<ResponsibleParty>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SystemCharacteristicsBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            system_ids: Vec::new(),
            system_name: None,
            system_name_short: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            date_authorized: None,
            security_sensitivity_level: None,
            system_information: None,
            security_impact_level: None,
            status: None,
            authorization_boundary: None,
            network_architecture: None,
            data_flow: None,
            responsible_parties: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for SystemCharacteristicsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SystemCharacteristicsBuilder {
    ///Set the `system-ids` field.
    pub fn system_ids(mut self, v: impl Into<SystemId>) -> Self {
        self.system_ids.push(v.into());
        self
    }
    ///Set the `system-name` field.
    pub fn system_name(mut self, v: impl Into<String>) -> Self {
        self.system_name = Some(v.into());
        self
    }
    ///Set the `system-name-short` field.
    pub fn system_name_short(mut self, v: impl Into<String>) -> Self {
        self.system_name_short = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `date-authorized` field.
    pub fn date_authorized(mut self, v: impl Into<chrono::NaiveDate>) -> Self {
        self.date_authorized = Some(v.into());
        self
    }
    ///Set the `security-sensitivity-level` field.
    pub fn security_sensitivity_level(mut self, v: impl Into<String>) -> Self {
        self.security_sensitivity_level = Some(v.into());
        self
    }
    ///Set the `system-information` field.
    pub fn system_information(mut self, v: impl Into<SystemInformation>) -> Self {
        self.system_information = Some(v.into());
        self
    }
    ///Set the `security-impact-level` field.
    pub fn security_impact_level(mut self, v: impl Into<SecurityImpactLevel>) -> Self {
        self.security_impact_level = Some(v.into());
        self
    }
    ///Set the `status` field.
    pub fn status(mut self, v: impl Into<Status>) -> Self {
        self.status = Some(v.into());
        self
    }
    ///Set the `authorization-boundary` field.
    pub fn authorization_boundary(mut self, v: impl Into<AuthorizationBoundary>) -> Self {
        self.authorization_boundary = Some(v.into());
        self
    }
    ///Set the `network-architecture` field.
    pub fn network_architecture(mut self, v: impl Into<NetworkArchitecture>) -> Self {
        self.network_architecture = Some(v.into());
        self
    }
    ///Set the `data-flow` field.
    pub fn data_flow(mut self, v: impl Into<DataFlow>) -> Self {
        self.data_flow = Some(v.into());
        self
    }
    ///Set the `responsible-parties` field.
    pub fn responsible_parties(mut self, v: impl Into<ResponsibleParty>) -> Self {
        self.responsible_parties.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<SystemCharacteristics, BuildError> {
        let system_name = self
            .system_name
            .ok_or_else(|| BuildError::MissingField("required field `system-name` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        let system_information = self.system_information.ok_or_else(|| {
            BuildError::MissingField("required field `system-information` not set")
        })?;
        let status = self
            .status
            .ok_or_else(|| BuildError::MissingField("required field `status` not set"))?;
        let authorization_boundary = self.authorization_boundary.ok_or_else(|| {
            BuildError::MissingField("required field `authorization-boundary` not set")
        })?;
        Ok(SystemCharacteristics {
            system_ids: self.system_ids,
            system_name,
            system_name_short: self.system_name_short,
            description,
            props: self.props,
            links: self.links,
            date_authorized: self.date_authorized,
            security_sensitivity_level: self.security_sensitivity_level,
            system_information,
            security_impact_level: self.security_impact_level,
            status,
            authorization_boundary,
            network_architecture: self.network_architecture,
            data_flow: self.data_flow,
            responsible_parties: self.responsible_parties,
            remarks: self.remarks,
        })
    }
}
impl SystemCharacteristics {
    /// Return a new builder for this type.
    pub fn builder() -> SystemCharacteristicsBuilder {
        SystemCharacteristicsBuilder::new()
    }
}
///A set of information type identifiers qualified by the given identificationsystemused, such as NIST SP 800-60.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Categorization {
    ///Specifies the information type identification system used.
    pub system: url::Url,
    ///Ahuman-oriented,globally uniqueidentifier qualified by the given identificationsystemused, such as NIST SP 800-60. This identifier hascross-instancescope and can be used to reference this system elsewhere inthis or other OSCAL instances. This id should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_type_id: Vec<String>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct CategorizationBuilder {
    system: Option<url::Url>,
    information_type_id: Vec<String>,
}
impl CategorizationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            system: None,
            information_type_id: Vec::new(),
        }
    }
}
impl Default for CategorizationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl CategorizationBuilder {
    ///Set the `system` field.
    pub fn system(mut self, v: impl Into<url::Url>) -> Self {
        self.system = Some(v.into());
        self
    }
    ///Set the `information-type-id` field.
    pub fn information_type_id(mut self, v: impl Into<String>) -> Self {
        self.information_type_id.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Categorization, BuildError> {
        let system = self
            .system
            .ok_or_else(|| BuildError::MissingField("required field `system` not set"))?;
        Ok(Categorization {
            system,
            information_type_id: self.information_type_id,
        })
    }
}
impl Categorization {
    /// Return a new builder for this type.
    pub fn builder() -> CategorizationBuilder {
        CategorizationBuilder::new()
    }
}
///Contains details about one information type that is stored, processed, or transmitted by the system, such as privacy information, and those defined inNIST SP 800-60.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InformationType {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this information type elsewhere inthis or other OSCAL instances. The locally definedUUIDof theinformation typecan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    ///A human readable name for the information type. This title should be meaningful within the context of the system.
    pub title: crate::primitives::MarkupLine,
    ///A summary of how this information type is used within the system.
    pub description: crate::primitives::MarkupMultiline,
    ///A set of information type identifiers qualified by the given identificationsystemused, such as NIST SP 800-60.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categorization: Vec<Categorization>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidentiality_impact: Option<Impact>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrity_impact: Option<Impact>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_impact: Option<Impact>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct InformationTypeBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    description: Option<crate::primitives::MarkupMultiline>,
    categorization: Vec<Categorization>,
    props: Vec<Property>,
    links: Vec<Link>,
    confidentiality_impact: Option<Impact>,
    integrity_impact: Option<Impact>,
    availability_impact: Option<Impact>,
}
impl InformationTypeBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            description: None,
            categorization: Vec::new(),
            props: Vec::new(),
            links: Vec::new(),
            confidentiality_impact: None,
            integrity_impact: None,
            availability_impact: None,
        }
    }
}
impl Default for InformationTypeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl InformationTypeBuilder {
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
        self.description = Some(v.into());
        self
    }
    ///Set the `categorization` field.
    pub fn categorization(mut self, v: impl Into<Categorization>) -> Self {
        self.categorization.push(v.into());
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
    ///Set the `confidentiality-impact` field.
    pub fn confidentiality_impact(mut self, v: impl Into<Impact>) -> Self {
        self.confidentiality_impact = Some(v.into());
        self
    }
    ///Set the `integrity-impact` field.
    pub fn integrity_impact(mut self, v: impl Into<Impact>) -> Self {
        self.integrity_impact = Some(v.into());
        self
    }
    ///Set the `availability-impact` field.
    pub fn availability_impact(mut self, v: impl Into<Impact>) -> Self {
        self.availability_impact = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<InformationType, BuildError> {
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(InformationType {
            uuid: self.uuid,
            title,
            description,
            categorization: self.categorization,
            props: self.props,
            links: self.links,
            confidentiality_impact: self.confidentiality_impact,
            integrity_impact: self.integrity_impact,
            availability_impact: self.availability_impact,
        })
    }
}
impl InformationType {
    /// Return a new builder for this type.
    pub fn builder() -> InformationTypeBuilder {
        InformationTypeBuilder::new()
    }
}
///Contains details about all information types that are stored, processed, or transmitted by the system, such as privacy information, and those defined inNIST SP 800-60.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemInformation {
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///Contains details about one information type that is stored, processed, or transmitted by the system, such as privacy information, and those defined inNIST SP 800-60.
    #[serde(default)]
    pub information_type: Vec<InformationType>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct SystemInformationBuilder {
    props: Vec<Property>,
    links: Vec<Link>,
    information_type: Vec<InformationType>,
}
impl SystemInformationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            props: Vec::new(),
            links: Vec::new(),
            information_type: Vec::new(),
        }
    }
}
impl Default for SystemInformationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SystemInformationBuilder {
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
    ///Set the `information-type` field.
    pub fn information_type(mut self, v: impl Into<InformationType>) -> Self {
        self.information_type.push(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<SystemInformation, BuildError> {
        Ok(SystemInformation {
            props: self.props,
            links: self.links,
            information_type: self.information_type,
        })
    }
}
impl SystemInformation {
    /// Return a new builder for this type.
    pub fn builder() -> SystemInformationBuilder {
        SystemInformationBuilder::new()
    }
}
///The expected level of impact resulting from the described information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Impact {
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    pub base: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_justification: Option<crate::primitives::MarkupMultiline>,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct ImpactBuilder {
    props: Vec<Property>,
    links: Vec<Link>,
    base: Option<String>,
    selected: Option<String>,
    adjustment_justification: Option<crate::primitives::MarkupMultiline>,
}
impl ImpactBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            props: Vec::new(),
            links: Vec::new(),
            base: None,
            selected: None,
            adjustment_justification: None,
        }
    }
}
impl Default for ImpactBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ImpactBuilder {
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
    ///Set the `base` field.
    pub fn base(mut self, v: impl Into<String>) -> Self {
        self.base = Some(v.into());
        self
    }
    ///Set the `selected` field.
    pub fn selected(mut self, v: impl Into<String>) -> Self {
        self.selected = Some(v.into());
        self
    }
    ///Set the `adjustment-justification` field.
    pub fn adjustment_justification(
        mut self,
        v: impl Into<crate::primitives::MarkupMultiline>,
    ) -> Self {
        self.adjustment_justification = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Impact, BuildError> {
        let base = self
            .base
            .ok_or_else(|| BuildError::MissingField("required field `base` not set"))?;
        Ok(Impact {
            props: self.props,
            links: self.links,
            base,
            selected: self.selected,
            adjustment_justification: self.adjustment_justification,
        })
    }
}
impl Impact {
    /// Return a new builder for this type.
    pub fn builder() -> ImpactBuilder {
        ImpactBuilder::new()
    }
}
///The prescribed base (Confidentiality, Integrity, or Availability) security impact level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Base {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///The selected (Confidentiality, Integrity, or Availability) security impact level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Selected {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///If the selected security level is different from the base security level, this contains the justification for the change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AdjustmentJustification {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<crate::primitives::MarkupMultiline>,
}
///The overall level of expected impact resulting from unauthorized disclosure, modification, or loss of access to information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SecurityImpactLevel {
    ///A target-level of confidentiality for the system, based on the sensitivity of information within the system.
    pub security_objective_confidentiality: String,
    ///A target-level of integrity for the system, based on the sensitivity of information within the system.
    pub security_objective_integrity: String,
    ///A target-level of availability for the system, based on the sensitivity of information within the system.
    pub security_objective_availability: String,
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct SecurityImpactLevelBuilder {
    security_objective_confidentiality: Option<String>,
    security_objective_integrity: Option<String>,
    security_objective_availability: Option<String>,
}
impl SecurityImpactLevelBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            security_objective_confidentiality: None,
            security_objective_integrity: None,
            security_objective_availability: None,
        }
    }
}
impl Default for SecurityImpactLevelBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SecurityImpactLevelBuilder {
    ///Set the `security-objective-confidentiality` field.
    pub fn security_objective_confidentiality(mut self, v: impl Into<String>) -> Self {
        self.security_objective_confidentiality = Some(v.into());
        self
    }
    ///Set the `security-objective-integrity` field.
    pub fn security_objective_integrity(mut self, v: impl Into<String>) -> Self {
        self.security_objective_integrity = Some(v.into());
        self
    }
    ///Set the `security-objective-availability` field.
    pub fn security_objective_availability(mut self, v: impl Into<String>) -> Self {
        self.security_objective_availability = Some(v.into());
        self
    }
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<SecurityImpactLevel, BuildError> {
        let security_objective_confidentiality =
            self.security_objective_confidentiality.ok_or_else(|| {
                BuildError::MissingField(
                    "required field `security-objective-confidentiality` not set",
                )
            })?;
        let security_objective_integrity = self.security_objective_integrity.ok_or_else(|| {
            BuildError::MissingField("required field `security-objective-integrity` not set")
        })?;
        let security_objective_availability =
            self.security_objective_availability.ok_or_else(|| {
                BuildError::MissingField("required field `security-objective-availability` not set")
            })?;
        Ok(SecurityImpactLevel {
            security_objective_confidentiality,
            security_objective_integrity,
            security_objective_availability,
        })
    }
}
impl SecurityImpactLevel {
    /// Return a new builder for this type.
    pub fn builder() -> SecurityImpactLevelBuilder {
        SecurityImpactLevelBuilder::new()
    }
}
///The date the system received its authorization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DateAuthorized {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<chrono::NaiveDate>,
}
///A description of this system's authorization boundary, optionally supplemented by diagrams that illustrate the authorization boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AuthorizationBoundary {
    ///A summary of the system's authorization boundary.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagrams: Vec<Diagram>,
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
pub struct AuthorizationBoundaryBuilder {
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    diagrams: Vec<Diagram>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl AuthorizationBoundaryBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            diagrams: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for AuthorizationBoundaryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AuthorizationBoundaryBuilder {
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `diagrams` field.
    pub fn diagrams(mut self, v: impl Into<Diagram>) -> Self {
        self.diagrams.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<AuthorizationBoundary, BuildError> {
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(AuthorizationBoundary {
            description,
            props: self.props,
            links: self.links,
            diagrams: self.diagrams,
            remarks: self.remarks,
        })
    }
}
impl AuthorizationBoundary {
    /// Return a new builder for this type.
    pub fn builder() -> AuthorizationBoundaryBuilder {
        AuthorizationBoundaryBuilder::new()
    }
}
///A graphic that provides a visual representation the system, or some aspect of it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Diagram {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this diagram elsewhere inthis or other OSCAL instances. The locally definedUUIDof thediagramcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///A summary of the diagram.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///A brief caption to annotate the diagram.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::primitives::MarkupLine>,
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
pub struct DiagramBuilder {
    uuid: Option<uuid::Uuid>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    caption: Option<crate::primitives::MarkupLine>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl DiagramBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            caption: None,
            remarks: None,
        }
    }
}
impl Default for DiagramBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl DiagramBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `caption` field.
    pub fn caption(mut self, v: impl Into<crate::primitives::MarkupLine>) -> Self {
        self.caption = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<Diagram, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        Ok(Diagram {
            uuid,
            description: self.description,
            props: self.props,
            links: self.links,
            caption: self.caption,
            remarks: self.remarks,
        })
    }
}
impl Diagram {
    /// Return a new builder for this type.
    pub fn builder() -> DiagramBuilder {
        DiagramBuilder::new()
    }
}
///A description of the system's network architecture, optionally supplemented by diagrams that illustrate the network architecture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NetworkArchitecture {
    ///A summary of the system's network architecture.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagrams: Vec<Diagram>,
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
pub struct NetworkArchitectureBuilder {
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    diagrams: Vec<Diagram>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl NetworkArchitectureBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            diagrams: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for NetworkArchitectureBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl NetworkArchitectureBuilder {
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `diagrams` field.
    pub fn diagrams(mut self, v: impl Into<Diagram>) -> Self {
        self.diagrams.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<NetworkArchitecture, BuildError> {
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(NetworkArchitecture {
            description,
            props: self.props,
            links: self.links,
            diagrams: self.diagrams,
            remarks: self.remarks,
        })
    }
}
impl NetworkArchitecture {
    /// Return a new builder for this type.
    pub fn builder() -> NetworkArchitectureBuilder {
        NetworkArchitectureBuilder::new()
    }
}
///A description of the logical flow of information within the system and across its boundaries, optionally supplemented by diagrams that illustrate these flows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DataFlow {
    ///A summary of the system's data flow.
    pub description: crate::primitives::MarkupMultiline,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagrams: Vec<Diagram>,
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
pub struct DataFlowBuilder {
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    diagrams: Vec<Diagram>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl DataFlowBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            diagrams: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for DataFlowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl DataFlowBuilder {
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `diagrams` field.
    pub fn diagrams(mut self, v: impl Into<Diagram>) -> Self {
        self.diagrams.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<DataFlow, BuildError> {
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(DataFlow {
            description,
            props: self.props,
            links: self.links,
            diagrams: self.diagrams,
            remarks: self.remarks,
        })
    }
}
impl DataFlow {
    /// Return a new builder for this type.
    pub fn builder() -> DataFlowBuilder {
        DataFlowBuilder::new()
    }
}
///A description of another authorized system from which this system inherits capabilities that satisfy security requirements. Another term for this concept is acommon control provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LeveragedAuthorization {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope and can be used to reference this leveraged authorization elsewhere inthis or other OSCAL instances. The locally definedUUIDof theleveraged authorizationcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///A human readable name for the leveraged authorization in the context of the system.
    pub title: crate::primitives::MarkupLine,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///Amachine-orientedidentifier reference to thepartythat manages the leveraged system.
    pub party_uuid: uuid::Uuid,
    ///
    pub date_authorized: chrono::NaiveDate,
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
pub struct LeveragedAuthorizationBuilder {
    uuid: Option<uuid::Uuid>,
    title: Option<crate::primitives::MarkupLine>,
    props: Vec<Property>,
    links: Vec<Link>,
    party_uuid: Option<uuid::Uuid>,
    date_authorized: Option<chrono::NaiveDate>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl LeveragedAuthorizationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            title: None,
            props: Vec::new(),
            links: Vec::new(),
            party_uuid: None,
            date_authorized: None,
            remarks: None,
        }
    }
}
impl Default for LeveragedAuthorizationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl LeveragedAuthorizationBuilder {
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
    ///Set the `party-uuid` field.
    pub fn party_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.party_uuid = Some(v.into());
        self
    }
    ///Set the `date-authorized` field.
    pub fn date_authorized(mut self, v: impl Into<chrono::NaiveDate>) -> Self {
        self.date_authorized = Some(v.into());
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
    pub fn build(self) -> ::core::result::Result<LeveragedAuthorization, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let title = self
            .title
            .ok_or_else(|| BuildError::MissingField("required field `title` not set"))?;
        let party_uuid = self
            .party_uuid
            .ok_or_else(|| BuildError::MissingField("required field `party-uuid` not set"))?;
        let date_authorized = self
            .date_authorized
            .ok_or_else(|| BuildError::MissingField("required field `date-authorized` not set"))?;
        Ok(LeveragedAuthorization {
            uuid,
            title,
            props: self.props,
            links: self.links,
            party_uuid,
            date_authorized,
            remarks: self.remarks,
        })
    }
}
impl LeveragedAuthorization {
    /// Return a new builder for this type.
    pub fn builder() -> LeveragedAuthorizationBuilder {
        LeveragedAuthorizationBuilder::new()
    }
}
///Provides information as to how the system is implemented.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemImplementation {
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///A description of another authorized system from which this system inherits capabilities that satisfy security requirements. Another term for this concept is acommon control provider.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub leveraged_authorization: Vec<LeveragedAuthorization>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<SystemUser>,
    ///
    #[serde(default)]
    pub components: Vec<SystemComponent>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inventory_items: Vec<InventoryItem>,
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
pub struct SystemImplementationBuilder {
    props: Vec<Property>,
    links: Vec<Link>,
    leveraged_authorization: Vec<LeveragedAuthorization>,
    users: Vec<SystemUser>,
    components: Vec<SystemComponent>,
    inventory_items: Vec<InventoryItem>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SystemImplementationBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            props: Vec::new(),
            links: Vec::new(),
            leveraged_authorization: Vec::new(),
            users: Vec::new(),
            components: Vec::new(),
            inventory_items: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for SystemImplementationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SystemImplementationBuilder {
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
    ///Set the `leveraged-authorization` field.
    pub fn leveraged_authorization(mut self, v: impl Into<LeveragedAuthorization>) -> Self {
        self.leveraged_authorization.push(v.into());
        self
    }
    ///Set the `users` field.
    pub fn users(mut self, v: impl Into<SystemUser>) -> Self {
        self.users.push(v.into());
        self
    }
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
    pub fn build(self) -> ::core::result::Result<SystemImplementation, BuildError> {
        Ok(SystemImplementation {
            props: self.props,
            links: self.links,
            leveraged_authorization: self.leveraged_authorization,
            users: self.users,
            components: self.components,
            inventory_items: self.inventory_items,
            remarks: self.remarks,
        })
    }
}
impl SystemImplementation {
    /// Return a new builder for this type.
    pub fn builder() -> SystemImplementationBuilder {
        SystemImplementationBuilder::new()
    }
}
///Describes a capability which may be inherited by a leveraging system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Provided {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this provided entry elsewhere inthis or other OSCAL instances. The locally definedUUIDof theprovidedentry can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///An implementation statement that describes the aspects of the control or control statement implementation that can be provided to another system leveraging this system.
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
pub struct ProvidedBuilder {
    uuid: Option<uuid::Uuid>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_roles: Vec<ResponsibleRole>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ProvidedBuilder {
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
impl Default for ProvidedBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ProvidedBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    pub fn build(self) -> ::core::result::Result<Provided, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(Provided {
            uuid,
            description,
            props: self.props,
            links: self.links,
            responsible_roles: self.responsible_roles,
            remarks: self.remarks,
        })
    }
}
impl Provided {
    /// Return a new builder for this type.
    pub fn builder() -> ProvidedBuilder {
        ProvidedBuilder::new()
    }
}
///Describes a control implementation responsibility imposed on a leveraging system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Responsibility {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this responsibility elsewhere inthis or other OSCAL instances. The locally definedUUIDof theresponsibilitycan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_uuid: Option<String>,
    ///An implementation statement that describes the aspects of the control or control statement implementation that a leveraging system must implement to satisfy the control provided by a leveraged system.
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
pub struct ResponsibilityBuilder {
    uuid: Option<uuid::Uuid>,
    provided_uuid: Option<String>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_roles: Vec<ResponsibleRole>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ResponsibilityBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            provided_uuid: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_roles: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ResponsibilityBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ResponsibilityBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `provided-uuid` field.
    pub fn provided_uuid(mut self, v: impl Into<String>) -> Self {
        self.provided_uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    pub fn build(self) -> ::core::result::Result<Responsibility, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(Responsibility {
            uuid,
            provided_uuid: self.provided_uuid,
            description,
            props: self.props,
            links: self.links,
            responsible_roles: self.responsible_roles,
            remarks: self.remarks,
        })
    }
}
impl Responsibility {
    /// Return a new builder for this type.
    pub fn builder() -> ResponsibilityBuilder {
        ResponsibilityBuilder::new()
    }
}
///Identifies content intended for external consumption, such as with leveraged organizations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Export {
    ///An implementation statement that describes the aspects of the control or control statement implementation that can be available to another system leveraging this system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::primitives::MarkupMultiline>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    ///Describes a capability which may be inherited by a leveraging system.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provided: Vec<Provided>,
    ///Describes a control implementation responsibility imposed on a leveraging system.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsibility: Vec<Responsibility>,
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
pub struct ExportBuilder {
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    provided: Vec<Provided>,
    responsibility: Vec<Responsibility>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ExportBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            provided: Vec::new(),
            responsibility: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ExportBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ExportBuilder {
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `provided` field.
    pub fn provided(mut self, v: impl Into<Provided>) -> Self {
        self.provided.push(v.into());
        self
    }
    ///Set the `responsibility` field.
    pub fn responsibility(mut self, v: impl Into<Responsibility>) -> Self {
        self.responsibility.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<Export, BuildError> {
        Ok(Export {
            description: self.description,
            props: self.props,
            links: self.links,
            provided: self.provided,
            responsibility: self.responsibility,
            remarks: self.remarks,
        })
    }
}
impl Export {
    /// Return a new builder for this type.
    pub fn builder() -> ExportBuilder {
        ExportBuilder::new()
    }
}
///Describes a control implementation inherited by a leveraging system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Inherited {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this inherited entry elsewhere inthis or other OSCAL instances. The locally definedUUIDof theinherited control implementationcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_uuid: Option<String>,
    ///An implementation statement that describes the aspects of a control or control statement implementation that a leveraging system is inheriting from a leveraged system.
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
}
/// Builder for [`#struct_name`].
///
/// Construct via [`#struct_name::builder()`], chain setters, then call
/// [`#builder_name::build()`].
#[must_use]
#[derive(Debug)]
pub struct InheritedBuilder {
    uuid: Option<uuid::Uuid>,
    provided_uuid: Option<String>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_roles: Vec<ResponsibleRole>,
}
impl InheritedBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            provided_uuid: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_roles: Vec::new(),
        }
    }
}
impl Default for InheritedBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl InheritedBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `provided-uuid` field.
    pub fn provided_uuid(mut self, v: impl Into<String>) -> Self {
        self.provided_uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    /// Consume the builder and return a fully constructed [`#struct_name`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::MissingField`] if any required field was not set.
    pub fn build(self) -> ::core::result::Result<Inherited, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(Inherited {
            uuid,
            provided_uuid: self.provided_uuid,
            description,
            props: self.props,
            links: self.links,
            responsible_roles: self.responsible_roles,
        })
    }
}
impl Inherited {
    /// Return a new builder for this type.
    pub fn builder() -> InheritedBuilder {
        InheritedBuilder::new()
    }
}
///Describes how this system satisfies a responsibility imposed by a leveraged system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Satisfied {
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this satisfied control implementation entry elsewhere inthis or other OSCAL instances. The locally definedUUIDof thecontrol implementationcan be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility_uuid: Option<String>,
    ///An implementation statement that describes the aspects of a control or control statement implementation that a leveraging system is implementing based on a requirement from a leveraged system.
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
pub struct SatisfiedBuilder {
    uuid: Option<uuid::Uuid>,
    responsibility_uuid: Option<String>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    responsible_roles: Vec<ResponsibleRole>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl SatisfiedBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            uuid: None,
            responsibility_uuid: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            responsible_roles: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for SatisfiedBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SatisfiedBuilder {
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `responsibility-uuid` field.
    pub fn responsibility_uuid(mut self, v: impl Into<String>) -> Self {
        self.responsibility_uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    pub fn build(self) -> ::core::result::Result<Satisfied, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(Satisfied {
            uuid,
            responsibility_uuid: self.responsibility_uuid,
            description,
            props: self.props,
            links: self.links,
            responsible_roles: self.responsible_roles,
            remarks: self.remarks,
        })
    }
}
impl Satisfied {
    /// Return a new builder for this type.
    pub fn builder() -> SatisfiedBuilder {
        SatisfiedBuilder::new()
    }
}
///Defines how the referenced component implements a set of controls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ByComponent {
    ///Amachine-orientedidentifier reference to thecomponentthat is implementing a given control.
    pub component_uuid: uuid::Uuid,
    ///Amachine-oriented,globally uniqueidentifier withcross-instancescope that can be used to reference this by-component entry elsewhere inthis or other OSCAL instances. The locally definedUUIDof theby-componententry can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assignedper-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: uuid::Uuid,
    ///An implementation statement that describes how a control or a control statement is implemented within the referenced system component.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_status: Option<ImplementationStatus>,
    ///Identifies content intended for external consumption, such as with leveraged organizations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<Export>,
    ///Describes a control implementation inherited by a leveraging system.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inherited: Vec<Inherited>,
    ///Describes how this system satisfies a responsibility imposed by a leveraged system.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub satisfied: Vec<Satisfied>,
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
pub struct ByComponentBuilder {
    component_uuid: Option<uuid::Uuid>,
    uuid: Option<uuid::Uuid>,
    description: Option<crate::primitives::MarkupMultiline>,
    props: Vec<Property>,
    links: Vec<Link>,
    set_parameters: Vec<SetParameter>,
    implementation_status: Option<ImplementationStatus>,
    export: Option<Export>,
    inherited: Vec<Inherited>,
    satisfied: Vec<Satisfied>,
    responsible_roles: Vec<ResponsibleRole>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl ByComponentBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self {
            component_uuid: None,
            uuid: None,
            description: None,
            props: Vec::new(),
            links: Vec::new(),
            set_parameters: Vec::new(),
            implementation_status: None,
            export: None,
            inherited: Vec::new(),
            satisfied: Vec::new(),
            responsible_roles: Vec::new(),
            remarks: None,
        }
    }
}
impl Default for ByComponentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ByComponentBuilder {
    ///Set the `component-uuid` field.
    pub fn component_uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.component_uuid = Some(v.into());
        self
    }
    ///Set the `uuid` field.
    pub fn uuid(mut self, v: impl Into<uuid::Uuid>) -> Self {
        self.uuid = Some(v.into());
        self
    }
    ///Set the `description` field.
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
    ///Set the `implementation-status` field.
    pub fn implementation_status(mut self, v: impl Into<ImplementationStatus>) -> Self {
        self.implementation_status = Some(v.into());
        self
    }
    ///Set the `export` field.
    pub fn export(mut self, v: impl Into<Export>) -> Self {
        self.export = Some(v.into());
        self
    }
    ///Set the `inherited` field.
    pub fn inherited(mut self, v: impl Into<Inherited>) -> Self {
        self.inherited.push(v.into());
        self
    }
    ///Set the `satisfied` field.
    pub fn satisfied(mut self, v: impl Into<Satisfied>) -> Self {
        self.satisfied.push(v.into());
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
    pub fn build(self) -> ::core::result::Result<ByComponent, BuildError> {
        let component_uuid = self
            .component_uuid
            .ok_or_else(|| BuildError::MissingField("required field `component-uuid` not set"))?;
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
        Ok(ByComponent {
            component_uuid,
            uuid,
            description,
            props: self.props,
            links: self.links,
            set_parameters: self.set_parameters,
            implementation_status: self.implementation_status,
            export: self.export,
            inherited: self.inherited,
            satisfied: self.satisfied,
            responsible_roles: self.responsible_roles,
            remarks: self.remarks,
        })
    }
}
impl ByComponent {
    /// Return a new builder for this type.
    pub fn builder() -> ByComponentBuilder {
        ByComponentBuilder::new()
    }
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
            .ok_or_else(|| BuildError::MissingField("required field `metadata` not set"))?;
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
            .ok_or_else(|| BuildError::MissingField("required field `finding-uuid` not set"))?;
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
    pub fn description(mut self, v: impl Into<crate::primitives::MarkupMultiline>) -> Self {
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
            .ok_or_else(|| BuildError::MissingField("required field `description` not set"))?;
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
