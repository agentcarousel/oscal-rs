// @generated — do not edit by hand.
// Run `cargo run -p oscal-metaschema-gen` to regenerate.

//!Generated OSCAL types for the `ssp` model.
#![allow(clippy::doc_markdown)]
use serde::{Deserialize, Serialize};
use super::common::*;
/// Error returned when a required field was not set before calling `build()`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BuildError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),
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
    pub fn system_characteristics(
        mut self,
        v: impl Into<SystemCharacteristics>,
    ) -> Self {
        self.system_characteristics = Some(v.into());
        self
    }
    ///Set the `system-implementation` field.
    pub fn system_implementation(mut self, v: impl Into<SystemImplementation>) -> Self {
        self.system_implementation = Some(v.into());
        self
    }
    ///Set the `control-implementation` field.
    pub fn control_implementation(
        mut self,
        v: impl Into<ControlImplementation>,
    ) -> Self {
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
            .ok_or_else(|| BuildError::MissingField(
                "required field `metadata` not set",
            ))?;
        let import_profile = self
            .import_profile
            .ok_or_else(|| BuildError::MissingField(
                "required field `import-profile` not set",
            ))?;
        let system_characteristics = self
            .system_characteristics
            .ok_or_else(|| BuildError::MissingField(
                "required field `system-characteristics` not set",
            ))?;
        let system_implementation = self
            .system_implementation
            .ok_or_else(|| BuildError::MissingField(
                "required field `system-implementation` not set",
            ))?;
        let control_implementation = self
            .control_implementation
            .ok_or_else(|| BuildError::MissingField(
                "required field `control-implementation` not set",
            ))?;
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
        Self { href: None, remarks: None }
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
    pub fn authorization_boundary(
        mut self,
        v: impl Into<AuthorizationBoundary>,
    ) -> Self {
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
            .ok_or_else(|| BuildError::MissingField(
                "required field `system-name` not set",
            ))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
        let system_information = self
            .system_information
            .ok_or_else(|| BuildError::MissingField(
                "required field `system-information` not set",
            ))?;
        let status = self
            .status
            .ok_or_else(|| BuildError::MissingField("required field `status` not set"))?;
        let authorization_boundary = self
            .authorization_boundary
            .ok_or_else(|| BuildError::MissingField(
                "required field `authorization-boundary` not set",
            ))?;
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
    pub fn description(
        mut self,
        v: impl Into<crate::primitives::MarkupMultiline>,
    ) -> Self {
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
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Base {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///The selected (Confidentiality, Integrity, or Availability) security impact level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Selected {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///If the selected security level is different from the base security level, this contains the justification for the change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
        let security_objective_confidentiality = self
            .security_objective_confidentiality
            .ok_or_else(|| BuildError::MissingField(
                "required field `security-objective-confidentiality` not set",
            ))?;
        let security_objective_integrity = self
            .security_objective_integrity
            .ok_or_else(|| BuildError::MissingField(
                "required field `security-objective-integrity` not set",
            ))?;
        let security_objective_availability = self
            .security_objective_availability
            .ok_or_else(|| BuildError::MissingField(
                "required field `security-objective-availability` not set",
            ))?;
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
///Describes the operational status of the system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Status {
    ///The current operating status.
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
pub struct StatusBuilder {
    state: Option<String>,
    remarks: Option<crate::primitives::MarkupMultiline>,
}
impl StatusBuilder {
    /// Create an empty builder with all fields unset.
    pub fn new() -> Self {
        Self { state: None, remarks: None }
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
///The date the system received its authorization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
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
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
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
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
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
            .ok_or_else(|| BuildError::MissingField(
                "required field `party-uuid` not set",
            ))?;
        let date_authorized = self
            .date_authorized
            .ok_or_else(|| BuildError::MissingField(
                "required field `date-authorized` not set",
            ))?;
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
    pub fn leveraged_authorization(
        mut self,
        v: impl Into<LeveragedAuthorization>,
    ) -> Self {
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
    pub fn build(self) -> ::core::result::Result<Provided, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
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
    pub fn build(self) -> ::core::result::Result<Responsibility, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
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
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
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
    pub fn build(self) -> ::core::result::Result<Satisfied, BuildError> {
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
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
            .ok_or_else(|| BuildError::MissingField(
                "required field `component-uuid` not set",
            ))?;
        let uuid = self
            .uuid
            .ok_or_else(|| BuildError::MissingField("required field `uuid` not set"))?;
        let description = self
            .description
            .ok_or_else(|| BuildError::MissingField(
                "required field `description` not set",
            ))?;
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
