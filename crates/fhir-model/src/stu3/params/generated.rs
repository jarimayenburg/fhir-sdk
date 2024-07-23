//! Generated code! Take a look at the generator-crate for changing this file!
#![allow(clippy::too_many_lines)]
use crate::ResourceWithSearchParameters;
use super::super::resources;
/// Search parameters for the Account resource
#[derive(Clone, Debug)]
pub enum AccountSearchParameter {
    /// How much is in account?
    Balance,
    /// Account number
    Identifier,
    /// Human-readable label
    Name,
    /// Who is responsible?
    Owner,
    /// What is account tied to?
    Patient,
    /// Transaction window
    Period,
    /// active | inactive | entered-in-error
    Status,
    /// What is account tied to?
    Subject,
    /// E.g. patient, expense, depreciation
    Type,
}
/// Search parameters for the ActivityDefinition resource
#[derive(Clone, Debug)]
pub enum ActivityDefinitionSearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// The activity definition publication date
    Date,
    /// What resource is being referenced
    DependsOn,
    /// What resource is being referenced
    DerivedFrom,
    /// The description of the activity definition
    Description,
    /// The time during which the activity definition is intended to be in use
    Effective,
    /// External identifier for the activity definition
    Identifier,
    /// Intended jurisdiction for the activity definition
    Jurisdiction,
    /// Computationally friendly name of the activity definition
    Name,
    /// What resource is being referenced
    Predecessor,
    /// Name of the publisher of the activity definition
    Publisher,
    /// The current status of the activity definition
    Status,
    /// What resource is being referenced
    Successor,
    /// The human-friendly name of the activity definition
    Title,
    /// Topics associated with the module
    Topic,
    /// The uri that identifies the activity definition
    Url,
    /// The business version of the activity definition
    Version,
}
/// Search parameters for the AdverseEvent resource
#[derive(Clone, Debug)]
pub enum AdverseEventSearchParameter {
    /** AE | PAE
An adverse event is an event that caused harm to a patient,  an adverse reaction is a something that is a subject-specific event that is a result of an exposure to a medication, food, device or environmental substance, a potential adverse event is something that occurred and that could have caused harm to a patient but did not*/
    Category,
    /// When the event occurred
    Date,
    /// Location where adverse event occurred
    Location,
    /// Adverse Reaction Events linked to exposure to substance
    Reaction,
    /// Who recorded the adverse event
    Recorder,
    /// Mild | Moderate | Severe
    Seriousness,
    /// AdverseEvent.study
    Study,
    /// Subject or group impacted by event
    Subject,
    /// Refers to the specific entity that caused the adverse event
    Substance,
    /// actual | potential
    Type,
}
/// Search parameters for the AllergyIntolerance resource
#[derive(Clone, Debug)]
pub enum AllergyIntoleranceSearchParameter {
    /// Source of the information about the allergy
    Asserter,
    /// food | medication | environment | biologic
    Category,
    /// active | inactive | resolved
    ClinicalStatus,
    /// Code that identifies the allergy or intolerance
    Code,
    /// low | high | unable-to-assess
    Criticality,
    /// Date record was believed accurate
    Date,
    /// External ids for this item
    Identifier,
    /// Date(/time) of last known occurrence of a reaction
    LastDate,
    /// Clinical symptoms/signs associated with the Event
    Manifestation,
    /// Date(/time) when manifestations showed
    Onset,
    /// Who the sensitivity is for
    Patient,
    /// Who recorded the sensitivity
    Recorder,
    /// How the subject was exposed to the substance
    Route,
    /// mild | moderate | severe (of event as a whole)
    Severity,
    /// allergy | intolerance - Underlying mechanism (if known)
    Type,
    /// unconfirmed | confirmed | refuted | entered-in-error
    VerificationStatus,
}
/// Search parameters for the Appointment resource
#[derive(Clone, Debug)]
pub enum AppointmentSearchParameter {
    /// Any one of the individuals participating in the appointment
    Actor,
    /// The style of appointment or patient that has been booked in the slot (not service type)
    AppointmentType,
    /// Appointment date/time.
    Date,
    /// An Identifier of the Appointment
    Identifier,
    /// The ReferralRequest provided as information to allocate to the Encounter
    Incomingreferral,
    /// This location is listed in the participants of the appointment
    Location,
    /// The Participation status of the subject, or other participant on the appointment. Can be used to locate participants that have not responded to meeting requests.
    PartStatus,
    /// One of the individuals of the appointment is this patient
    Patient,
    /// One of the individuals of the appointment is this practitioner
    Practitioner,
    /// The specific service that is to be performed during this appointment
    ServiceType,
    /// The overall status of the appointment
    Status,
}
/// Search parameters for the AppointmentResponse resource
#[derive(Clone, Debug)]
pub enum AppointmentResponseSearchParameter {
    /// The Person, Location/HealthcareService or Device that this appointment response replies for
    Actor,
    /// The appointment that the response is attached to
    Appointment,
    /// An Identifier in this appointment response
    Identifier,
    /// This Response is for this Location
    Location,
    /// The participants acceptance status for this appointment
    PartStatus,
    /// This Response is for this Patient
    Patient,
    /// This Response is for this Practitioner
    Practitioner,
}
/// Search parameters for the AuditEvent resource
#[derive(Clone, Debug)]
pub enum AuditEventSearchParameter {
    /// Type of action performed during the event
    Action,
    /// Identifier for the network access point of the user device
    Address,
    /// Direct reference to resource
    Agent,
    /// Human-meaningful name for the agent
    AgentName,
    /// Agent role in the event
    AgentRole,
    /// Alternative User id e.g. authentication
    Altid,
    /// Time when the event occurred on source
    Date,
    /// Specific instance of resource
    Entity,
    /// Specific instance of object
    EntityId,
    /// Descriptor for entity
    EntityName,
    /// What role the entity played
    EntityRole,
    /// Type of entity involved
    EntityType,
    /// Whether the event succeeded or failed
    Outcome,
    /// Direct reference to resource
    Patient,
    /// Policy that authorized event
    Policy,
    /// Logical source location within the enterprise
    Site,
    /// The identity of source detecting the event
    Source,
    /// More specific type/id for the event
    Subtype,
    /// Type/identifier of event
    Type,
    /// Unique identifier for the user
    User,
}
/// Search parameters for the Basic resource
#[derive(Clone, Debug)]
pub enum BasicSearchParameter {
    /// Who created
    Author,
    /// Kind of Resource
    Code,
    /// When created
    Created,
    /// Business identifier
    Identifier,
    /// Identifies the focus of this resource
    Patient,
    /// Identifies the focus of this resource
    Subject,
}
/// Search parameters for the Binary resource
#[derive(Clone, Debug)]
pub enum BinarySearchParameter {
    /// MimeType of the binary content
    Contenttype,
}
/// Search parameters for the BodySite resource
#[derive(Clone, Debug)]
pub enum BodySiteSearchParameter {
    /// Named anatomical location
    Code,
    /// Identifier for this instance of the anatomical location
    Identifier,
    /// Patient to whom bodysite belongs
    Patient,
}
/// Search parameters for the Bundle resource
#[derive(Clone, Debug)]
pub enum BundleSearchParameter {
    /// The first resource in the bundle, if the bundle type is "document" - this is a composition, and this parameter provides access to searches its contents
    Composition,
    /// Persistent identifier for the bundle
    Identifier,
    /// The first resource in the bundle, if the bundle type is "message" - this is a message header, and this parameter provides access to search its contents
    Message,
    /// document | message | transaction | transaction-response | batch | batch-response | history | searchset | collection
    Type,
}
/// Search parameters for the CapabilityStatement resource
#[derive(Clone, Debug)]
pub enum CapabilityStatementSearchParameter {
    /// The capability statement publication date
    Date,
    /// The description of the capability statement
    Description,
    /// Event code in a capability statement
    Event,
    /// The version of FHIR
    Fhirversion,
    /// formats supported (xml | json | ttl | mime type)
    Format,
    /// Implementation guides supported
    Guide,
    /// Intended jurisdiction for the capability statement
    Jurisdiction,
    /// Mode - restful (server/client) or messaging (sender/receiver)
    Mode,
    /// Computationally friendly name of the capability statement
    Name,
    /// Name of the publisher of the capability statement
    Publisher,
    /// Name of a resource mentioned in a capability statement
    Resource,
    /// A profile id invoked in a capability statement
    ResourceProfile,
    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    SecurityService,
    /// Part of a the name of a software application
    Software,
    /// The current status of the capability statement
    Status,
    /// Profiles for use cases supported
    SupportedProfile,
    /// The human-friendly name of the capability statement
    Title,
    /// The uri that identifies the capability statement
    Url,
    /// The business version of the capability statement
    Version,
}
/// Search parameters for the CarePlan resource
#[derive(Clone, Debug)]
pub enum CarePlanSearchParameter {
    /// Time period plan covers
    Date,
    /// External Ids for this plan
    Identifier,
    /// Who care plan is for
    Patient,
    /// Detail type of activity
    ActivityCode,
    /// Specified date occurs within period specified by CarePlan.activity.timingSchedule
    ActivityDate,
    /// Activity details defined in specific resource
    ActivityReference,
    /// Fulfills care plan
    BasedOn,
    /// Who's involved in plan?
    CareTeam,
    /// Type of plan
    Category,
    /// Health issues this plan addresses
    Condition,
    /// Created in context of
    Context,
    /// Protocol or definition
    Definition,
    /// Created in context of
    Encounter,
    /// Desired outcome of plan
    Goal,
    /// proposal | plan | order | option
    Intent,
    /// Part of referenced CarePlan
    PartOf,
    /// Matches if the practitioner is listed as a performer in any of the "simple" activities.  (For performers of the detailed activities, chain through the activitydetail search parameter.)
    Performer,
    /// CarePlan replaced by this CarePlan
    Replaces,
    /// draft | active | suspended | completed | entered-in-error | cancelled | unknown
    Status,
    /// Who care plan is for
    Subject,
}
/// Search parameters for the CareTeam resource
#[derive(Clone, Debug)]
pub enum CareTeamSearchParameter {
    /// Time period team covers
    Date,
    /// External Ids for this team
    Identifier,
    /// Who care team is for
    Patient,
    /// Type of team
    Category,
    /// Encounter or episode associated with CareTeam
    Context,
    /// Encounter or episode associated with CareTeam
    Encounter,
    /// Who is involved
    Participant,
    /// proposed | active | suspended | inactive | entered-in-error
    Status,
    /// Who care team is for
    Subject,
}
/// Search parameters for the ChargeItem resource
#[derive(Clone, Debug)]
pub enum ChargeItemSearchParameter {
    /// Account to place this charge
    Account,
    /// A code that identifies the charge, like a billing code
    Code,
    /// Encounter / Episode associated with event
    Context,
    /// Date the charge item was entered
    EnteredDate,
    /// Individual who was entering
    Enterer,
    /// Factor overriding the associated rules
    FactorOverride,
    /// Business Identifier for item
    Identifier,
    /// When the charged service was applied
    Occurrence,
    /// Individual who was performing
    ParticipantActor,
    /// What type of performance was done
    ParticipantRole,
    /// Individual service was done for/to
    Patient,
    /// Organization providing the charged sevice
    PerformingOrganization,
    /// Price overriding the associated rules
    PriceOverride,
    /// Quantity of which the charge item has been serviced
    Quantity,
    /// Organization requesting the charged service
    RequestingOrganization,
    /// Which rendered service is being charged?
    Service,
    /// Individual service was done for/to
    Subject,
}
/// Search parameters for the Claim resource
#[derive(Clone, Debug)]
pub enum ClaimSearchParameter {
    /// Member of the CareTeam
    CareTeam,
    /// The creation date for the Claim
    Created,
    /// Encounters associated with a billed line item
    Encounter,
    /// The party responsible for the entry of the Claim
    Enterer,
    /// Facility responsible for the goods and services
    Facility,
    /// The primary identifier of the financial resource
    Identifier,
    /// The target payor/insurer for the Claim
    Insurer,
    /// The reference to the providing organization
    Organization,
    /// Patient receiving the services
    Patient,
    /// The party receiving any payment for the Claim
    Payee,
    /// Processing priority requested
    Priority,
    /// Provider responsible for the Claim
    Provider,
    /// The kind of financial resource
    Use,
}
/// Search parameters for the ClaimResponse resource
#[derive(Clone, Debug)]
pub enum ClaimResponseSearchParameter {
    /// The creation date
    Created,
    /// The contents of the disposition message
    Disposition,
    /// The identity of the claimresponse
    Identifier,
    /// The organization who generated this resource
    Insurer,
    /// The processing outcome
    Outcome,
    /// The subject of care.
    Patient,
    /// The expected paymentDate
    PaymentDate,
    /// The claim reference
    Request,
    /// The Provider of the claim
    RequestProvider,
}
/// Search parameters for the ClinicalImpression resource
#[derive(Clone, Debug)]
pub enum ClinicalImpressionSearchParameter {
    /// When the assessment was documented
    Date,
    /// Patient or group assessed
    Patient,
    /// Action taken as part of assessment procedure
    Action,
    /// The clinician performing the assessment
    Assessor,
    /// Encounter or Episode created from
    Context,
    /// What was found
    FindingCode,
    /// What was found
    FindingRef,
    /// Business identifier
    Identifier,
    /// Record of a specific investigation
    Investigation,
    /// Reference to last assessment
    Previous,
    /// Relevant impressions of patient state
    Problem,
    /// draft | completed | entered-in-error
    Status,
    /// Patient or group assessed
    Subject,
}
/// Search parameters for the CodeSystem resource
#[derive(Clone, Debug)]
pub enum CodeSystemSearchParameter {
    /// A code defined in the code system
    Code,
    /// not-present | example | fragment | complete
    ContentMode,
    /// The code system publication date
    Date,
    /// The description of the code system
    Description,
    /// External identifier for the code system
    Identifier,
    /// Intended jurisdiction for the code system
    Jurisdiction,
    /// A language in which a designation is provided
    Language,
    /// Computationally friendly name of the code system
    Name,
    /// Name of the publisher of the code system
    Publisher,
    /// The current status of the code system
    Status,
    /// The system for any codes defined by this code system (same as 'url')
    System,
    /// The human-friendly name of the code system
    Title,
    /// The uri that identifies the code system
    Url,
    /// The business version of the code system
    Version,
}
/// Search parameters for the Communication resource
#[derive(Clone, Debug)]
pub enum CommunicationSearchParameter {
    /// Request fulfilled by this communication
    BasedOn,
    /// Message category
    Category,
    /// Encounter or episode leading to message
    Context,
    /// Instantiates protocol or definition
    Definition,
    /// Encounter leading to message
    Encounter,
    /// Unique identifier
    Identifier,
    /// A channel of communication
    Medium,
    /// Part of this action
    PartOf,
    /// Focus of message
    Patient,
    /// When received
    Received,
    /// Message recipient
    Recipient,
    /// Message sender
    Sender,
    /// When sent
    Sent,
    /// preparation | in-progress | suspended | aborted | completed | entered-in-error
    Status,
    /// Focus of message
    Subject,
}
/// Search parameters for the CommunicationRequest resource
#[derive(Clone, Debug)]
pub enum CommunicationRequestSearchParameter {
    /// When request transitioned to being actionable
    Authored,
    /// Fulfills plan or proposal
    BasedOn,
    /// Message category
    Category,
    /// Encounter or episode leading to message
    Context,
    /// Encounter leading to message
    Encounter,
    /// Composite request this is part of
    GroupIdentifier,
    /// Unique identifier
    Identifier,
    /// A channel of communication
    Medium,
    /// When scheduled
    Occurrence,
    /// Focus of message
    Patient,
    /// Message urgency
    Priority,
    /// Message recipient
    Recipient,
    /// Request(s) replaced by this request
    Replaces,
    /// Individual making the request
    Requester,
    /// Message sender
    Sender,
    /// draft | active | suspended | cancelled | completed | entered-in-error | unknown
    Status,
    /// Focus of message
    Subject,
}
/// Search parameters for the CompartmentDefinition resource
#[derive(Clone, Debug)]
pub enum CompartmentDefinitionSearchParameter {
    /// Patient | Encounter | RelatedPerson | Practitioner | Device
    Code,
    /// The compartment definition publication date
    Date,
    /// The description of the compartment definition
    Description,
    /// Intended jurisdiction for the compartment definition
    Jurisdiction,
    /// Computationally friendly name of the compartment definition
    Name,
    /// Name of the publisher of the compartment definition
    Publisher,
    /// Name of resource type
    Resource,
    /// The current status of the compartment definition
    Status,
    /// The human-friendly name of the compartment definition
    Title,
    /// The uri that identifies the compartment definition
    Url,
}
/// Search parameters for the Composition resource
#[derive(Clone, Debug)]
pub enum CompositionSearchParameter {
    /// Composition editing time
    Date,
    /// Logical identifier of composition (version-independent)
    Identifier,
    /// Who and/or what the composition is about
    Patient,
    /// Kind of composition (LOINC if possible)
    Type,
    /// Who attested the composition
    Attester,
    /// Who and/or what authored the composition
    Author,
    /// Categorization of Composition
    Class,
    /// As defined by affinity domain
    Confidentiality,
    /// Code(s) that apply to the event being documented
    Context,
    /// Context of the Composition
    Encounter,
    /// A reference to data that supports this section
    Entry,
    /// The period covered by the documentation
    Period,
    /// Target of the relationship
    RelatedId,
    /// Target of the relationship
    RelatedRef,
    /// Classification of section (recommended)
    Section,
    /// preliminary | final | amended | entered-in-error
    Status,
    /// Who and/or what the composition is about
    Subject,
    /// Human Readable name/title
    Title,
}
/// Search parameters for the ConceptMap resource
#[derive(Clone, Debug)]
pub enum ConceptMapSearchParameter {
    /// The concept map publication date
    Date,
    /// Reference to property mapping depends on
    Dependson,
    /// The description of the concept map
    Description,
    /// External identifier for the concept map
    Identifier,
    /// Intended jurisdiction for the concept map
    Jurisdiction,
    /// Computationally friendly name of the concept map
    Name,
    /// Canonical URL for other concept map
    Other,
    /// Reference to property mapping depends on
    Product,
    /// Name of the publisher of the concept map
    Publisher,
    /// Identifies the source of the concepts which are being mapped
    Source,
    /// Identifies element being mapped
    SourceCode,
    /// Code System (if value set crosses code systems)
    SourceSystem,
    /// Identifies the source of the concepts which are being mapped
    SourceUri,
    /// The current status of the concept map
    Status,
    /// Provides context to the mappings
    Target,
    /// Code that identifies the target element
    TargetCode,
    /// System of the target (if necessary)
    TargetSystem,
    /// Provides context to the mappings
    TargetUri,
    /// The human-friendly name of the concept map
    Title,
    /// The uri that identifies the concept map
    Url,
    /// The business version of the concept map
    Version,
}
/// Search parameters for the Condition resource
#[derive(Clone, Debug)]
pub enum ConditionSearchParameter {
    /// Code for the condition
    Code,
    /// A unique identifier of the condition record
    Identifier,
    /// Who has the condition?
    Patient,
    /// Abatement as age or age range
    AbatementAge,
    /// Abatement boolean (boolean is true or non-boolean values are present)
    AbatementBoolean,
    /// Date-related abatements (dateTime and period)
    AbatementDate,
    /// Abatement as a string
    AbatementString,
    /// Date record was believed accurate
    AssertedDate,
    /// Person who asserts this condition
    Asserter,
    /// Anatomical location, if relevant
    BodySite,
    /// The category of the condition
    Category,
    /// The clinical status of the condition
    ClinicalStatus,
    /// Encounter or episode when condition first asserted
    Context,
    /// Encounter when condition first asserted
    Encounter,
    /// Manifestation/symptom
    Evidence,
    /// Supporting information found elsewhere
    EvidenceDetail,
    /// Onsets as age or age range
    OnsetAge,
    /// Date related onsets (dateTime and Period)
    OnsetDate,
    /// Onsets as a string
    OnsetInfo,
    /// The severity of the condition
    Severity,
    /// Simple summary (disease specific)
    Stage,
    /// Who has the condition?
    Subject,
    /// provisional | differential | confirmed | refuted | entered-in-error | unknown
    VerificationStatus,
}
/// Search parameters for the Consent resource
#[derive(Clone, Debug)]
pub enum ConsentSearchParameter {
    /// When this Consent was created or indexed
    Date,
    /// Identifier for this record (external references)
    Identifier,
    /// Who the consent applies to
    Patient,
    /// Actions controlled by this consent
    Action,
    /// Resource for the actor (or group, by role)
    Actor,
    /// Classification of the consent statement - for indexing/retrieval
    Category,
    /// Who is agreeing to the policy and exceptions
    Consentor,
    /// The actual data reference
    Data,
    /// Custodian of the consent
    Organization,
    /// Period that this consent applies
    Period,
    /// Context of activities for which the agreement is made
    Purpose,
    /// Security Labels that define affected resources
    Securitylabel,
    /// Source from which this consent is taken
    Source,
    /// draft | proposed | active | rejected | inactive | entered-in-error
    Status,
}
/// Search parameters for the Contract resource
#[derive(Clone, Debug)]
pub enum ContractSearchParameter {
    /// Agent to the Contact
    Agent,
    /// The authority of the contract
    Authority,
    /// The domain of the contract
    Domain,
    /// The identity of the contract
    Identifier,
    /// The date/time the contract was issued
    Issued,
    /// The identity of the subject of the contract (if a patient)
    Patient,
    /// Contract Signatory Party
    Signer,
    /// The identity of the subject of the contract
    Subject,
    /// The identity of the topic of the contract terms
    TermTopic,
}
/// Search parameters for the Coverage resource
#[derive(Clone, Debug)]
pub enum CoverageSearchParameter {
    /// Covered party
    Beneficiary,
    /// Class identifier
    Class,
    /// Dependent number
    Dependent,
    /// Group identifier
    Group,
    /// The primary identifier of the insured and the coverage
    Identifier,
    /// The identity of the insurer or party paying for services
    Payor,
    /// A plan or policy identifier
    Plan,
    /// Reference to the policyholder
    PolicyHolder,
    /// Sequence number
    Sequence,
    /// Sub-class identifier
    Subclass,
    /// Sub-group identifier
    Subgroup,
    /// Sub-plan identifier
    Subplan,
    /// Reference to the subscriber
    Subscriber,
    /// The kind of coverage (health plan, auto, Workers Compensation)
    Type,
}
/// Search parameters for the DataElement resource
#[derive(Clone, Debug)]
pub enum DataElementSearchParameter {
    /// A code for the data element (server may choose to do subsumption)
    Code,
    /// The data element publication date
    Date,
    /// Text search in the description of the data element.  This corresponds to the definition of the first DataElement.element.
    Description,
    /// External identifier for the data element
    Identifier,
    /// Intended jurisdiction for the data element
    Jurisdiction,
    /// Computationally friendly name of the data element
    Name,
    /// Name of the publisher of the data element
    Publisher,
    /// The current status of the data element
    Status,
    /// The stringency of the data element definition
    Stringency,
    /// The human-friendly name of the data element
    Title,
    /// The uri that identifies the data element
    Url,
    /// The business version of the data element
    Version,
}
/// Search parameters for the DetectedIssue resource
#[derive(Clone, Debug)]
pub enum DetectedIssueSearchParameter {
    /// When identified
    Date,
    /// Unique id for the detected issue
    Identifier,
    /// Associated patient
    Patient,
    /// The provider or device that identified the issue
    Author,
    /// Issue Category, e.g. drug-drug, duplicate therapy, etc.
    Category,
    /// Problem resource
    Implicated,
}
/// Search parameters for the Device resource
#[derive(Clone, Debug)]
pub enum DeviceSearchParameter {
    /// A server defined search that may match any of the string fields in the Device.udi.name  or Device.type.coding.display or  Device.type.text
    DeviceName,
    /// Instance id from manufacturer, owner, and others
    Identifier,
    /// A location, where the resource is found
    Location,
    /// The manufacturer of the device
    Manufacturer,
    /// The model of the device
    Model,
    /// The organization responsible for the device
    Organization,
    /// Patient information, if the resource is affixed to a person
    Patient,
    /// active | inactive | entered-in-error | unknown
    Status,
    /// The type of the device
    Type,
    /// UDI Barcode (RFID or other technology) string either in HRF format or AIDC format converted to base64 string.
    UdiCarrier,
    /// The udi Device Identifier (DI)
    UdiDi,
    /// Network address to contact device
    Url,
}
/// Search parameters for the DeviceComponent resource
#[derive(Clone, Debug)]
pub enum DeviceComponentSearchParameter {
    /// The identifier of the component
    Identifier,
    /// The parent DeviceComponent resource
    Parent,
    /// The device source
    Source,
    /// The device component type
    Type,
}
/// Search parameters for the DeviceMetric resource
#[derive(Clone, Debug)]
pub enum DeviceMetricSearchParameter {
    /// The category of the metric
    Category,
    /// The identifier of the metric
    Identifier,
    /// The parent DeviceMetric resource
    Parent,
    /// The device resource
    Source,
    /// The component type
    Type,
}
/// Search parameters for the DeviceRequest resource
#[derive(Clone, Debug)]
pub enum DeviceRequestSearchParameter {
    /// Code for what is being requested/ordered
    Code,
    /// Business identifier for request/order
    Identifier,
    /// Individual the service is ordered for
    Patient,
    /// Encounter or Episode during which request was created
    Encounter,
    /// When the request transitioned to being actionable
    AuthoredOn,
    /// Plan/proposal/order fulfilled by this request
    BasedOn,
    /// Protocol or definition followed by this request
    Definition,
    /// Reference to resource that is being requested/ordered
    Device,
    /// When service should occur
    EventDate,
    /// Composite request this is part of
    GroupIdentifier,
    /// proposal | plan | original-order |reflex-order
    Intent,
    /// Desired performer for service
    Performer,
    /// Request takes the place of referenced completed or terminated requests
    Priorrequest,
    /// Who/what is requesting service 
    Requester,
    /// entered-in-error | draft | active |suspended | completed 
    Status,
    /// Individual the service is ordered for
    Subject,
}
/// Search parameters for the DeviceUseStatement resource
#[derive(Clone, Debug)]
pub enum DeviceUseStatementSearchParameter {
    /// Search by subject - a patient
    Patient,
    /// Search by device
    Device,
    /// Search by identifier
    Identifier,
    /// Search by subject
    Subject,
}
/// Search parameters for the DiagnosticReport resource
#[derive(Clone, Debug)]
pub enum DiagnosticReportSearchParameter {
    /// The code for the report as a whole, as opposed to codes for the atomic results, which are the names on the observation resource referred to from the result
    Code,
    /// The clinically relevant time of the report
    Date,
    /// An identifier for the report
    Identifier,
    /// The subject of the report if a patient
    Patient,
    /// The Encounter when the order was made
    Encounter,
    /// Reference to the procedure request.
    BasedOn,
    /// Which diagnostic discipline/department created the report
    Category,
    /// Healthcare event (Episode of Care or Encounter) related to the report
    Context,
    /// A coded diagnosis on the report
    Diagnosis,
    /// A reference to the image source.
    Image,
    /// When the report was issued
    Issued,
    /// Who was the source of the report (organization)
    Performer,
    /// Link to an atomic result (observation resource)
    Result,
    /// The specimen details
    Specimen,
    /// The status of the report
    Status,
    /// The subject of the report
    Subject,
}
/// Search parameters for the DocumentManifest resource
#[derive(Clone, Debug)]
pub enum DocumentManifestSearchParameter {
    /// Unique Identifier for the set of documents
    Identifier,
    /// The subject of the set of documents
    Patient,
    /// Kind of document set
    Type,
    /// Who and/or what authored the manifest
    Author,
    /// Contents of this set of documents
    ContentRef,
    /// When this document manifest created
    Created,
    /// Human-readable description (title)
    Description,
    /// Intended to get notified about this set of documents
    Recipient,
    /// Identifiers of things that are related
    RelatedId,
    /// Related Resource
    RelatedRef,
    /// The source system/application/software
    Source,
    /// current | superseded | entered-in-error
    Status,
    /// The subject of the set of documents
    Subject,
}
/// Search parameters for the DocumentReference resource
#[derive(Clone, Debug)]
pub enum DocumentReferenceSearchParameter {
    /// Master Version Specific Identifier
    Identifier,
    /// Who/what is the subject of the document
    Patient,
    /// Kind of document (LOINC if possible)
    Type,
    /// Context of the document  content
    Encounter,
    /// Who/what authenticated the document
    Authenticator,
    /// Who and/or what authored the document
    Author,
    /// Categorization of document
    Class,
    /// Document creation time
    Created,
    /// Organization which maintains the document
    Custodian,
    /// Human-readable description (title)
    Description,
    /// Main clinical acts documented
    Event,
    /// Kind of facility where patient was seen
    Facility,
    /// Format/content rules for the document
    Format,
    /// When this document reference was created
    Indexed,
    /// Human language of the content (BCP-47)
    Language,
    /// Uri where the data can be found
    Location,
    /// Time of service that is being documented
    Period,
    /// Identifier of related objects or events
    RelatedId,
    /// Related Resource
    RelatedRef,
    /// Target of the relationship
    Relatesto,
    /// replaces | transforms | signs | appends
    Relation,
    /// Combination of relation and relatesTo
    Relationship,
    /// Document security-tags
    Securitylabel,
    /// Additional details about where the content was created (e.g. clinical specialty)
    Setting,
    /// current | superseded | entered-in-error
    Status,
    /// Who/what is the subject of the document
    Subject,
}
/// Search parameters for the EligibilityRequest resource
#[derive(Clone, Debug)]
pub enum EligibilityRequestSearchParameter {
    /// The creation date for the EOB
    Created,
    /// The party who is responsible for the request
    Enterer,
    /// Facility responsible for the goods and services
    Facility,
    /// The business identifier of the Eligibility
    Identifier,
    /// The reference to the providing organization
    Organization,
    /// The reference to the patient
    Patient,
    /// The reference to the provider
    Provider,
}
/// Search parameters for the EligibilityResponse resource
#[derive(Clone, Debug)]
pub enum EligibilityResponseSearchParameter {
    /// The creation date
    Created,
    /// The contents of the disposition message
    Disposition,
    /// The business identifier
    Identifier,
    /// The organization which generated this resource
    Insurer,
    /// The processing outcome
    Outcome,
    /// The EligibilityRequest reference
    Request,
    /// The EligibilityRequest organization
    RequestOrganization,
    /// The EligibilityRequest provider
    RequestProvider,
}
/// Search parameters for the Encounter resource
#[derive(Clone, Debug)]
pub enum EncounterSearchParameter {
    /// A date within the period the Encounter lasted
    Date,
    /// Identifier(s) by which this encounter is known
    Identifier,
    /// The patient ro group present at the encounter
    Patient,
    /// Specific type of encounter
    Type,
    /// The appointment that scheduled this encounter
    Appointment,
    /// inpatient | outpatient | ambulatory | emergency +
    Class,
    /// Reason the encounter takes place (resource)
    Diagnosis,
    /// Episode(s) of care that this encounter should be recorded against
    Episodeofcare,
    /// The ReferralRequest that initiated this encounter
    Incomingreferral,
    /// Length of encounter in days
    Length,
    /// Location the encounter takes place
    Location,
    /// Time period during which the patient was present at the location
    LocationPeriod,
    /// Another Encounter this encounter is part of
    PartOf,
    /// Persons involved in the encounter other than the patient
    Participant,
    /// Role of participant in encounter
    ParticipantType,
    /// Persons involved in the encounter other than the patient
    Practitioner,
    /// Reason the encounter takes place (code)
    Reason,
    /// The custodian organization of this Encounter record
    ServiceProvider,
    /// Wheelchair, translator, stretcher, etc.
    SpecialArrangement,
    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +
    Status,
    /// The patient ro group present at the encounter
    Subject,
}
/// Search parameters for the Endpoint resource
#[derive(Clone, Debug)]
pub enum EndpointSearchParameter {
    /// Protocol/Profile/Standard to be used with this endpoint connection
    ConnectionType,
    /// Identifies this endpoint across multiple systems
    Identifier,
    /// A name that this endpoint can be identified by
    Name,
    /// The organization that is managing the endpoint
    Organization,
    /// The type of content that may be used at this endpoint (e.g. XDS Discharge summaries)
    PayloadType,
    /// The current status of the Endpoint (usually expected to be active)
    Status,
}
/// Search parameters for the EnrollmentRequest resource
#[derive(Clone, Debug)]
pub enum EnrollmentRequestSearchParameter {
    /// The business identifier of the Enrollment
    Identifier,
    /// The organization who generated this resource
    Organization,
    /// The party to be enrolled
    Patient,
    /// The party to be enrolled
    Subject,
}
/// Search parameters for the EnrollmentResponse resource
#[derive(Clone, Debug)]
pub enum EnrollmentResponseSearchParameter {
    /// The business identifier of the EnrollmentResponse
    Identifier,
    /// The organization who generated this resource
    Organization,
    /// The reference to the claim
    Request,
}
/// Search parameters for the EpisodeOfCare resource
#[derive(Clone, Debug)]
pub enum EpisodeOfCareSearchParameter {
    /// The provided date search value falls within the episode of care's period
    Date,
    /// Business Identifier(s) relevant for this EpisodeOfCare
    Identifier,
    /// The patient who is the focus of this episode of care
    Patient,
    /// Type/class  - e.g. specialist referral, disease management
    Type,
    /// Care manager/care co-ordinator for the patient
    CareManager,
    /// Conditions/problems/diagnoses this episode of care is for
    Condition,
    /// Incoming Referral Request
    Incomingreferral,
    /// The organization that has assumed the specific responsibilities of this EpisodeOfCare
    Organization,
    /// The current status of the Episode of Care as provided (does not check the status history collection)
    Status,
}
/// Search parameters for the ExpansionProfile resource
#[derive(Clone, Debug)]
pub enum ExpansionProfileSearchParameter {
    /// The expansion profile publication date
    Date,
    /// The description of the expansion profile
    Description,
    /// External identifier for the expansion profile
    Identifier,
    /// Intended jurisdiction for the expansion profile
    Jurisdiction,
    /// Computationally friendly name of the expansion profile
    Name,
    /// Name of the publisher of the expansion profile
    Publisher,
    /// The current status of the expansion profile
    Status,
    /// The uri that identifies the expansion profile
    Url,
    /// The business version of the expansion profile
    Version,
}
/// Search parameters for the ExplanationOfBenefit resource
#[derive(Clone, Debug)]
pub enum ExplanationOfBenefitSearchParameter {
    /// Member of the CareTeam
    CareTeam,
    /// The reference to the claim
    Claim,
    /// The plan under which the claim was adjudicated
    Coverage,
    /// The creation date for the EOB
    Created,
    /// The contents of the disposition message
    Disposition,
    /// Encounters associated with a billed line item
    Encounter,
    /// The party responsible for the entry of the Claim
    Enterer,
    /// Facility responsible for the goods and services
    Facility,
    /// The business identifier of the Explanation of Benefit
    Identifier,
    /// The reference to the providing organization
    Organization,
    /// The reference to the patient
    Patient,
    /// The party receiving any payment for the Claim
    Payee,
    /// The reference to the provider
    Provider,
}
/// Search parameters for the FamilyMemberHistory resource
#[derive(Clone, Debug)]
pub enum FamilyMemberHistorySearchParameter {
    /// A search by a condition code
    Code,
    /// When history was captured/updated
    Date,
    /// A search by a record identifier
    Identifier,
    /// The identity of a subject to list family member history items for
    Patient,
    /// Instantiates protocol or definition
    Definition,
    /// A search by a gender code of a family member
    Gender,
    /// A search by a relationship type
    Relationship,
    /// partial | completed | entered-in-error | health-unknown
    Status,
}
/// Search parameters for the Flag resource
#[derive(Clone, Debug)]
pub enum FlagSearchParameter {
    /// Time period when flag is active
    Date,
    /// The identity of a subject to list flags for
    Patient,
    /// Alert relevant during encounter
    Encounter,
    /// Flag creator
    Author,
    /// Business identifier
    Identifier,
    /// The identity of a subject to list flags for
    Subject,
}
/// Search parameters for the Goal resource
#[derive(Clone, Debug)]
pub enum GoalSearchParameter {
    /// External Ids for this goal
    Identifier,
    /// Who this goal is intended for
    Patient,
    /// E.g. Treatment, dietary, behavioral, etc.
    Category,
    /// When goal pursuit begins
    StartDate,
    /// proposed | accepted | planned | in-progress | on-target | ahead-of-target | behind-target | sustaining | achieved | on-hold | cancelled | entered-in-error | rejected
    Status,
    /// Who this goal is intended for
    Subject,
    /// Reach goal on or before
    TargetDate,
}
/// Search parameters for the GraphDefinition resource
#[derive(Clone, Debug)]
pub enum GraphDefinitionSearchParameter {
    /// The graph definition publication date
    Date,
    /// The description of the graph definition
    Description,
    /// Intended jurisdiction for the graph definition
    Jurisdiction,
    /// Computationally friendly name of the graph definition
    Name,
    /// Name of the publisher of the graph definition
    Publisher,
    /// Type of resource at which the graph starts
    Start,
    /// The current status of the graph definition
    Status,
    /// The uri that identifies the graph definition
    Url,
    /// The business version of the graph definition
    Version,
}
/// Search parameters for the Group resource
#[derive(Clone, Debug)]
pub enum GroupSearchParameter {
    /// Descriptive or actual
    Actual,
    /// Kind of characteristic
    Characteristic,
    /// A composite of both characteristic and value
    CharacteristicValue,
    /// The kind of resources contained
    Code,
    /// Group includes or excludes
    Exclude,
    /// Unique id
    Identifier,
    /// Reference to the group member
    Member,
    /// The type of resources the group contains
    Type,
    /// Value held by characteristic
    Value,
}
/// Search parameters for the GuidanceResponse resource
#[derive(Clone, Debug)]
pub enum GuidanceResponseSearchParameter {
    /// The identifier of the guidance response
    Identifier,
    /// The identity of a patient to search for guidance response results
    Patient,
    /// The identifier of the request associated with the response
    Request,
    /// The subject that the guidance response is about
    Subject,
}
/// Search parameters for the HealthcareService resource
#[derive(Clone, Debug)]
pub enum HealthcareServiceSearchParameter {
    /// The Healthcare Service is currently marked as active
    Active,
    /// Service Category of the Healthcare Service
    Category,
    /// One of the HealthcareService's characteristics
    Characteristic,
    /// Technical endpoints providing access to services operated for the location
    Endpoint,
    /// External identifiers for this item
    Identifier,
    /// The location of the Healthcare Service
    Location,
    /// A portion of the Healthcare service name
    Name,
    /// The organization that provides this Healthcare Service
    Organization,
    /// One of the Program Names serviced by this HealthcareService
    Programname,
    /// The type of service provided by this healthcare service
    Type,
}
/// Search parameters for the ImagingManifest resource
#[derive(Clone, Debug)]
pub enum ImagingManifestSearchParameter {
    /// Subject of the ImagingManifest (or a DICOM Key Object Selection which it represents)
    Patient,
    /// Author of the ImagingManifest (or a DICOM Key Object Selection which it represents)
    Author,
    /// Time of the ImagingManifest (or a DICOM Key Object Selection which it represents) authoring
    AuthoringTime,
    /// The endpoint for the study or series
    Endpoint,
    /// UID of the ImagingManifest (or a DICOM Key Object Selection which it represents)
    Identifier,
    /// ImagingStudy resource selected in the ImagingManifest (or a DICOM Key Object Selection which it represents)
    ImagingStudy,
    /// Study selected in the ImagingManifest (or a DICOM Key Object Selection which it represents)
    SelectedStudy,
}
/// Search parameters for the ImagingStudy resource
#[derive(Clone, Debug)]
pub enum ImagingStudySearchParameter {
    /// Other identifiers for the Study
    Identifier,
    /// Who the study is about
    Patient,
    /// The accession identifier for the study
    Accession,
    /// The order for the image
    Basedon,
    /// The body site studied
    Bodysite,
    /// The context of the study
    Context,
    /// The type of the instance
    DicomClass,
    /// The endpoint for te study or series
    Endpoint,
    /// The modality of the series
    Modality,
    /// The person who performed the study
    Performer,
    /// The reason for the study
    Reason,
    /// The identifier of the series of images
    Series,
    /// When the study was started
    Started,
    /// The study identifier for the image
    Study,
    /// The instance unique identifier
    Uid,
}
/// Search parameters for the Immunization resource
#[derive(Clone, Debug)]
pub enum ImmunizationSearchParameter {
    /// Vaccination  (non)-Administration Date
    Date,
    /// Business identifier
    Identifier,
    /// The patient for the vaccination record
    Patient,
    /// Dose number within series
    DoseSequence,
    /// The service delivery location or facility in which the vaccine was / was to be administered
    Location,
    /// Vaccine Lot Number
    LotNumber,
    /// Vaccine Manufacturer
    Manufacturer,
    /// Administrations which were not given
    Notgiven,
    /// The practitioner who played a role in the vaccination
    Practitioner,
    /// Additional information on reaction
    Reaction,
    /// When reaction started
    ReactionDate,
    /// Why immunization occurred
    Reason,
    /// Explanation of reason vaccination was not administered
    ReasonNotGiven,
    /// Immunization event status
    Status,
    /// Vaccine Product Administered
    VaccineCode,
}
/// Search parameters for the ImmunizationRecommendation resource
#[derive(Clone, Debug)]
pub enum ImmunizationRecommendationSearchParameter {
    /// Date recommendation created
    Date,
    /// Recommended dose number
    DoseNumber,
    /// Dose number within sequence
    DoseSequence,
    /// Business identifier
    Identifier,
    /// Patient observations supporting recommendation
    Information,
    /// Who this profile is for
    Patient,
    /// Vaccine administration status
    Status,
    /// Past immunizations supporting recommendation
    Support,
    /// Disease to be immunized against
    TargetDisease,
    /// Vaccine recommendation applies to
    VaccineType,
}
/// Search parameters for the ImplementationGuide resource
#[derive(Clone, Debug)]
pub enum ImplementationGuideSearchParameter {
    /// The implementation guide publication date
    Date,
    /// Where to find dependency
    Dependency,
    /// The description of the implementation guide
    Description,
    /// For testing purposes, not real usage
    Experimental,
    /// Intended jurisdiction for the implementation guide
    Jurisdiction,
    /// Computationally friendly name of the implementation guide
    Name,
    /// Name of the publisher of the implementation guide
    Publisher,
    /// Location of the resource
    Resource,
    /// The current status of the implementation guide
    Status,
    /// The uri that identifies the implementation guide
    Url,
    /// The business version of the implementation guide
    Version,
}
/// Search parameters for the Library resource
#[derive(Clone, Debug)]
pub enum LibrarySearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// The library publication date
    Date,
    /// What resource is being referenced
    DependsOn,
    /// What resource is being referenced
    DerivedFrom,
    /// The description of the library
    Description,
    /// The time during which the library is intended to be in use
    Effective,
    /// External identifier for the library
    Identifier,
    /// Intended jurisdiction for the library
    Jurisdiction,
    /// Computationally friendly name of the library
    Name,
    /// What resource is being referenced
    Predecessor,
    /// Name of the publisher of the library
    Publisher,
    /// The current status of the library
    Status,
    /// What resource is being referenced
    Successor,
    /// The human-friendly name of the library
    Title,
    /// Topics associated with the module
    Topic,
    /// The uri that identifies the library
    Url,
    /// The business version of the library
    Version,
}
/// Search parameters for the Linkage resource
#[derive(Clone, Debug)]
pub enum LinkageSearchParameter {
    /// Author of the Linkage
    Author,
    /// Matches on any item in the Linkage
    Item,
    /// Matches on any item in the Linkage with a type of 'source'
    Source,
}
/// Search parameters for the List resource
#[derive(Clone, Debug)]
pub enum ListSearchParameter {
    /// What the purpose of this list is
    Code,
    /// When the list was prepared
    Date,
    /// Business identifier
    Identifier,
    /// If all resources have the same subject
    Patient,
    /// Context in which list created
    Encounter,
    /// Why list is empty
    EmptyReason,
    /// Actual entry
    Item,
    /// The annotation  - text content
    Notes,
    /// Who and/or what defined the list contents (aka Author)
    Source,
    /// current | retired | entered-in-error
    Status,
    /// If all resources have the same subject
    Subject,
    /// Descriptive name for the list
    Title,
}
/// Search parameters for the Location resource
#[derive(Clone, Debug)]
pub enum LocationSearchParameter {
    /// A (part of the) address of the location
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postal code specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// Technical endpoints providing access to services operated for the location
    Endpoint,
    /// An identifier for the location
    Identifier,
    /// A portion of the location's name or alias
    Name,
    /** The coordinates expressed as [latitude]:[longitude] (using the WGS84 datum, see notes) to find locations near to (servers may search using a square rather than a circle for efficiency)

Requires the near-distance parameter to be provided also*/
    Near,
    /** A distance quantity to limit the near search to locations within a specific distance

Requires the near parameter to also be included*/
    NearDistance,
    /// Searches for locations (typically bed/room) that have an operational status (e.g. contaminated, housekeeping)
    OperationalStatus,
    /// Searches for locations that are managed by the provided organization
    Organization,
    /// A location of which this location is a part
    Partof,
    /// Searches for locations with a specific kind of status
    Status,
    /// A code for the type of location
    Type,
}
/// Search parameters for the Measure resource
#[derive(Clone, Debug)]
pub enum MeasureSearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// The measure publication date
    Date,
    /// What resource is being referenced
    DependsOn,
    /// What resource is being referenced
    DerivedFrom,
    /// The description of the measure
    Description,
    /// The time during which the measure is intended to be in use
    Effective,
    /// External identifier for the measure
    Identifier,
    /// Intended jurisdiction for the measure
    Jurisdiction,
    /// Computationally friendly name of the measure
    Name,
    /// What resource is being referenced
    Predecessor,
    /// Name of the publisher of the measure
    Publisher,
    /// The current status of the measure
    Status,
    /// What resource is being referenced
    Successor,
    /// The human-friendly name of the measure
    Title,
    /// Topics associated with the module
    Topic,
    /// The uri that identifies the measure
    Url,
    /// The business version of the measure
    Version,
}
/// Search parameters for the MeasureReport resource
#[derive(Clone, Debug)]
pub enum MeasureReportSearchParameter {
    /// External identifier of the measure report to be returned
    Identifier,
    /// The identity of a patient to search for individual measure report results for
    Patient,
    /// The status of the measure report
    Status,
}
/// Search parameters for the Media resource
#[derive(Clone, Debug)]
pub enum MediaSearchParameter {
    /// Procedure that caused this media to be created
    BasedOn,
    /// Encounter / Episode associated with media
    Context,
    /// Date attachment was first created
    Created,
    /// When Media was collected
    Date,
    /// Observing Device
    Device,
    /// Identifier(s) for the image
    Identifier,
    /// The person who generated the image
    Operator,
    /// Who/What this Media is a record of
    Patient,
    /// Body part in media
    Site,
    /// Who/What this Media is a record of
    Subject,
    /// The type of acquisition equipment/process
    Subtype,
    /// photo | video | audio
    Type,
    /// Imaging view, e.g. Lateral or Antero-posterior
    View,
}
/// Search parameters for the Medication resource
#[derive(Clone, Debug)]
pub enum MedicationSearchParameter {
    /// Codes that identify this medication
    Code,
    /// E.g. box, vial, blister-pack
    Container,
    /// powder | tablets | capsule +
    Form,
    /// The product contained
    Ingredient,
    /// The product contained
    IngredientCode,
    /// Manufacturer of the item
    Manufacturer,
    /// True if medication does not require a prescription
    OverTheCounter,
    /// The item in the package
    PackageItem,
    /// The item in the package
    PackageItemCode,
    /// active | inactive | entered-in-error
    Status,
}
/// Search parameters for the MedicationAdministration resource
#[derive(Clone, Debug)]
pub enum MedicationAdministrationSearchParameter {
    /// Return administrations of this medication code
    Code,
    /// Return administrations that share this encounter or episode of care
    Context,
    /// Return administrations with this administration device identity
    Device,
    /// Date administration happened (or did not happen)
    EffectiveTime,
    /// Return administrations with this external identifier
    Identifier,
    /// Return administrations of this medication resource
    Medication,
    /// Administrations that were not made
    NotGiven,
    /// The identity of a patient to list administrations  for
    Patient,
    /// The identify of the individual who administered the medication
    Performer,
    /// The identity of a prescription to list administrations from
    Prescription,
    /// Reasons for administering the medication
    ReasonGiven,
    /// Reasons for not administering the medication
    ReasonNotGiven,
    /// MedicationAdministration event status (for example one of active/paused/completed/nullified)
    Status,
    /// The identify of the individual or group to list administrations for
    Subject,
}
/// Search parameters for the MedicationDispense resource
#[derive(Clone, Debug)]
pub enum MedicationDispenseSearchParameter {
    /// Return dispenses of this medicine code
    Code,
    /// Return dispenses with this external identifier
    Identifier,
    /// Return dispenses of this medicine resource
    Medication,
    /// The identity of a patient to list dispenses  for
    Patient,
    /// The identity of a prescription to list dispenses from
    Prescription,
    /// Return dispenses with a specified dispense status
    Status,
    /// Returns dispenses with a specific context (episode or episode of care)
    Context,
    /// Return dispenses that should be sent to a specific destination
    Destination,
    /// Return dispenses performed by a specific individual
    Performer,
    /// The identity of a receiver to list dispenses for
    Receiver,
    /// Return dispenses with the specified responsible party
    Responsibleparty,
    /// The identity of a patient to list dispenses  for
    Subject,
    /// Return dispenses of a specific type
    Type,
    /// Returns dispenses handed over on this date
    Whenhandedover,
    /// Returns dispenses prepared on this date
    Whenprepared,
}
/// Search parameters for the MedicationRequest resource
#[derive(Clone, Debug)]
pub enum MedicationRequestSearchParameter {
    /// Return prescriptions of this medication code
    Code,
    /// Return prescriptions with this external identifier
    Identifier,
    /// Return prescriptions of this medication reference
    Medication,
    /// Returns prescriptions for a specific patient
    Patient,
    /// Status of the prescription
    Status,
    /// Return prescriptions written on this date
    Authoredon,
    /// Returns prescriptions with different categories
    Category,
    /// Return prescriptions with this encounter or episode of care identifier
    Context,
    /// Returns medication request to be administered on a specific date
    Date,
    /// Returns prescriptions intended to be dispensed by this Organization
    IntendedDispenser,
    /// Returns prescriptions with different intents
    Intent,
    /// Returns prescriptions with different priorities
    Priority,
    /// Returns prescriptions prescribed by this prescriber
    Requester,
    /// The identity of a patient to list orders  for
    Subject,
}
/// Search parameters for the MedicationStatement resource
#[derive(Clone, Debug)]
pub enum MedicationStatementSearchParameter {
    /// Return statements of this medication code
    Code,
    /// Return statements with this external identifier
    Identifier,
    /// Return statements of this medication reference
    Medication,
    /// Returns statements for a specific patient.
    Patient,
    /// Return statements that match the given status
    Status,
    /// Returns statements of this category of medicationstatement
    Category,
    /// Returns statements for a specific context (episode or episode of Care).
    Context,
    /// Date when patient was taking (or not taking) the medication
    Effective,
    /// Returns statements that are part of another event.
    PartOf,
    /// Who or where the information in the statement came from
    Source,
    /// The identity of a patient, animal or group to list statements for
    Subject,
}
/// Search parameters for the MessageDefinition resource
#[derive(Clone, Debug)]
pub enum MessageDefinitionSearchParameter {
    /// The behavior associated with the message
    Category,
    /// The message definition publication date
    Date,
    /// The description of the message definition
    Description,
    /// The event that triggers the message
    Event,
    /// A resource that is a permitted focus of the message
    Focus,
    /// External identifier for the message definition
    Identifier,
    /// Intended jurisdiction for the message definition
    Jurisdiction,
    /// Computationally friendly name of the message definition
    Name,
    /// Name of the publisher of the message definition
    Publisher,
    /// The current status of the message definition
    Status,
    /// The human-friendly name of the message definition
    Title,
    /// The uri that identifies the message definition
    Url,
    /// The business version of the message definition
    Version,
}
/// Search parameters for the MessageHeader resource
#[derive(Clone, Debug)]
pub enum MessageHeaderSearchParameter {
    /// The source of the decision
    Author,
    /// ok | transient-error | fatal-error
    Code,
    /// Name of system
    Destination,
    /// Actual destination address or id
    DestinationUri,
    /// The source of the data entry
    Enterer,
    /// Code for the event this message represents
    Event,
    /// The actual content of the message
    Focus,
    /// Intended "real-world" recipient for the data
    Receiver,
    /// Id of original message
    ResponseId,
    /// Final responsibility for event
    Responsible,
    /// Real world sender of the message
    Sender,
    /// Name of system
    Source,
    /// Actual message source address or id
    SourceUri,
    /// Particular delivery destination within the destination
    Target,
    /// Time that the message was sent
    Timestamp,
}
/// Search parameters for the NamingSystem resource
#[derive(Clone, Debug)]
pub enum NamingSystemSearchParameter {
    /// Name of an individual to contact
    Contact,
    /// The naming system publication date
    Date,
    /// The description of the naming system
    Description,
    /// oid | uuid | uri | other
    IdType,
    /// Intended jurisdiction for the naming system
    Jurisdiction,
    /// codesystem | identifier | root
    Kind,
    /// Computationally friendly name of the naming system
    Name,
    /// When is identifier valid?
    Period,
    /// Name of the publisher of the naming system
    Publisher,
    /// Use this instead
    ReplacedBy,
    /// Who maintains system namespace?
    Responsible,
    /// The current status of the naming system
    Status,
    /// Contact details for individual or organization
    Telecom,
    /// e.g. driver,  provider,  patient, bank etc.
    Type,
    /// The unique identifier
    Value,
}
/// Search parameters for the NutritionOrder resource
#[derive(Clone, Debug)]
pub enum NutritionOrderSearchParameter {
    /// Return nutrition orders with this external identifier
    Identifier,
    /// The identity of the person who requires the diet, formula or nutritional supplement
    Patient,
    /// Return nutrition orders with this encounter identifier
    Encounter,
    /// Type of module component to add to the feeding
    Additive,
    /// Return nutrition orders requested on this date
    Datetime,
    /// Type of enteral or infant formula
    Formula,
    /// Type of diet that can be consumed orally (i.e., take via the mouth).
    Oraldiet,
    /// The identify of the provider who placed the nutrition order
    Provider,
    /// Status of the nutrition order.
    Status,
    /// Type of supplement product requested
    Supplement,
}
/// Search parameters for the Observation resource
#[derive(Clone, Debug)]
pub enum ObservationSearchParameter {
    /// The code of the observation type
    Code,
    /// Obtained date/time. If the obtained element is a period, a date that falls in the period
    Date,
    /// The unique id for a particular observation
    Identifier,
    /// The subject that the observation is about (if patient)
    Patient,
    /// Encounter related to the observation
    Encounter,
    /// Reference to the test or procedure request.
    BasedOn,
    /// The classification of the type of observation
    Category,
    /// Code and coded value parameter pair
    CodeValueConcept,
    /// Code and date/time value parameter pair
    CodeValueDate,
    /// Code and quantity value parameter pair
    CodeValueQuantity,
    /// Code and string value parameter pair
    CodeValueString,
    /// The code of the observation type or component type
    ComboCode,
    /// Code and coded value parameter pair, including in components
    ComboCodeValueConcept,
    /// Code and quantity value parameter pair, including in components
    ComboCodeValueQuantity,
    /// The reason why the expected value in the element Observation.value[x] or Observation.component.value[x] is missing.
    ComboDataAbsentReason,
    /// The value or component value of the observation, if the value is a CodeableConcept
    ComboValueConcept,
    /// The value or component value of the observation, if the value is a Quantity, or a SampledData (just search on the bounds of the values in sampled data)
    ComboValueQuantity,
    /// The component code of the observation type
    ComponentCode,
    /// Component code and component coded value parameter pair
    ComponentCodeValueConcept,
    /// Component code and component quantity value parameter pair
    ComponentCodeValueQuantity,
    /// The reason why the expected value in the element Observation.component.value[x] is missing.
    ComponentDataAbsentReason,
    /// The value of the component observation, if the value is a CodeableConcept
    ComponentValueConcept,
    /// The value of the component observation, if the value is a Quantity, or a SampledData (just search on the bounds of the values in sampled data)
    ComponentValueQuantity,
    /// Healthcare event  (Episode-of-care or Encounter) related to the observation
    Context,
    /// The reason why the expected value in the element Observation.value[x] is missing.
    DataAbsentReason,
    /// The Device that generated the observation data.
    Device,
    /// The method used for the observation
    Method,
    /// Who performed the observation
    Performer,
    /// Related Observations - search on related-type and related-target together
    Related,
    /// Resource that is related to this one
    RelatedTarget,
    /// has-member | derived-from | sequel-to | replaces | qualified-by | interfered-by
    RelatedType,
    /// Specimen used for this observation
    Specimen,
    /// The status of the observation
    Status,
    /// The subject that the observation is about
    Subject,
    /// The value of the observation, if the value is a CodeableConcept
    ValueConcept,
    /// The value of the observation, if the value is a date or period of time
    ValueDate,
    /// The value of the observation, if the value is a Quantity, or a SampledData (just search on the bounds of the values in sampled data)
    ValueQuantity,
    /// The value of the observation, if the value is a string, and also searches in CodeableConcept.text
    ValueString,
}
/// Search parameters for the OperationDefinition resource
#[derive(Clone, Debug)]
pub enum OperationDefinitionSearchParameter {
    /// Marks this as a profile of the base
    Base,
    /// Name used to invoke the operation
    Code,
    /// The operation definition publication date
    Date,
    /// The description of the operation definition
    Description,
    /// Invoke on an instance?
    Instance,
    /// Intended jurisdiction for the operation definition
    Jurisdiction,
    /// operation | query
    Kind,
    /// Computationally friendly name of the operation definition
    Name,
    /// Profile on the type
    ParamProfile,
    /// Name of the publisher of the operation definition
    Publisher,
    /// The current status of the operation definition
    Status,
    /// Invoke at the system level?
    System,
    /// Invole at the type level?
    Type,
    /// The uri that identifies the operation definition
    Url,
    /// The business version of the operation definition
    Version,
}
/// Search parameters for the Organization resource
#[derive(Clone, Debug)]
pub enum OrganizationSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Active,
    /// A (part of the) address of the organization
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postal code specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// Technical endpoints providing access to services operated for the organization
    Endpoint,
    /// Any identifier for the organization (not the accreditation issuer's identifier)
    Identifier,
    /// A portion of the organization's name or alias
    Name,
    /// An organization of which this organization forms a part
    Partof,
    /// A portion of the organization's name using some kind of phonetic matching algorithm
    Phonetic,
    /// A code for the type of organization
    Type,
}
/// Search parameters for the Patient resource
#[derive(Clone, Debug)]
pub enum PatientSearchParameter {
    /// Whether the patient record is active
    Active,
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postalCode specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// The breed for animal patients
    AnimalBreed,
    /// The species for animal patients
    AnimalSpecies,
    /// The patient's date of birth
    Birthdate,
    /// The date of death has been provided and satisfies this search value
    DeathDate,
    /// This patient has been marked as deceased, or as a death date entered
    Deceased,
    /// A value in an email contact
    Email,
    /// A portion of the family name of the patient
    Family,
    /// Gender of the patient
    Gender,
    /// Patient's nominated general practitioner, not the organization that manages the record
    GeneralPractitioner,
    /// A portion of the given name of the patient
    Given,
    /// A patient identifier
    Identifier,
    /// Language code (irrespective of use value)
    Language,
    /// All patients linked to the given patient
    Link,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
    /// The organization at which this person is a patient
    Organization,
    /// A value in a phone contact
    Phone,
    /// A portion of either family or given name using some kind of phonetic matching algorithm
    Phonetic,
    /// The value in any kind of telecom details of the patient
    Telecom,
}
/// Search parameters for the PaymentNotice resource
#[derive(Clone, Debug)]
pub enum PaymentNoticeSearchParameter {
    /// Creation date fro the notice
    Created,
    /// The business identifier of the notice
    Identifier,
    /// The organization who generated this resource
    Organization,
    /// The type of payment notice
    PaymentStatus,
    /// The reference to the provider
    Provider,
    /// The Claim
    Request,
    /// The ClaimResponse
    Response,
    /// The date of the payment action
    Statusdate,
}
/// Search parameters for the PaymentReconciliation resource
#[derive(Clone, Debug)]
pub enum PaymentReconciliationSearchParameter {
    /// The creation date
    Created,
    /// The contents of the disposition message
    Disposition,
    /// The business identifier of the Explanation of Benefit
    Identifier,
    /// The organization who generated this resource
    Organization,
    /// The processing outcome
    Outcome,
    /// The reference to the claim
    Request,
    /// The organization who generated this resource
    RequestOrganization,
    /// The reference to the provider who sumbitted the claim
    RequestProvider,
}
/// Search parameters for the Person resource
#[derive(Clone, Debug)]
pub enum PersonSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postal code specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// The person's date of birth
    Birthdate,
    /// A value in an email contact
    Email,
    /// The gender of the person
    Gender,
    /// A value in a phone contact
    Phone,
    /// A portion of name using some kind of phonetic matching algorithm
    Phonetic,
    /// The value in any kind of contact
    Telecom,
    /// A person Identifier
    Identifier,
    /// Any link has this Patient, Person, RelatedPerson or Practitioner reference
    Link,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
    /// The organization at which this person record is being managed
    Organization,
    /// The Person links to this Patient
    Patient,
    /// The Person links to this Practitioner
    Practitioner,
    /// The Person links to this RelatedPerson
    Relatedperson,
}
/// Search parameters for the PlanDefinition resource
#[derive(Clone, Debug)]
pub enum PlanDefinitionSearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// The plan definition publication date
    Date,
    /// What resource is being referenced
    DependsOn,
    /// What resource is being referenced
    DerivedFrom,
    /// The description of the plan definition
    Description,
    /// The time during which the plan definition is intended to be in use
    Effective,
    /// External identifier for the plan definition
    Identifier,
    /// Intended jurisdiction for the plan definition
    Jurisdiction,
    /// Computationally friendly name of the plan definition
    Name,
    /// What resource is being referenced
    Predecessor,
    /// Name of the publisher of the plan definition
    Publisher,
    /// The current status of the plan definition
    Status,
    /// What resource is being referenced
    Successor,
    /// The human-friendly name of the plan definition
    Title,
    /// Topics associated with the module
    Topic,
    /// The uri that identifies the plan definition
    Url,
    /// The business version of the plan definition
    Version,
}
/// Search parameters for the Practitioner resource
#[derive(Clone, Debug)]
pub enum PractitionerSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postalCode specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// A value in an email contact
    Email,
    /// A portion of the family name
    Family,
    /// Gender of the practitioner
    Gender,
    /// A portion of the given name
    Given,
    /// A value in a phone contact
    Phone,
    /// A portion of either family or given name using some kind of phonetic matching algorithm
    Phonetic,
    /// The value in any kind of contact
    Telecom,
    /// Whether the practitioner record is active
    Active,
    /// One of the languages that the practitioner can communicate with
    Communication,
    /// A practitioner's Identifier
    Identifier,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
}
/// Search parameters for the PractitionerRole resource
#[derive(Clone, Debug)]
pub enum PractitionerRoleSearchParameter {
    /// A value in an email contact
    Email,
    /// A value in a phone contact
    Phone,
    /// The value in any kind of contact
    Telecom,
    /// Whether this practitioner's record is in active use
    Active,
    /// The period during which the practitioner is authorized to perform in these role(s)
    Date,
    /// Technical endpoints providing access to services operated for the practitioner with this role
    Endpoint,
    /// A practitioner's Identifier
    Identifier,
    /// One of the locations at which this practitioner provides care
    Location,
    /// The identity of the organization the practitioner represents / acts on behalf of
    Organization,
    /// Practitioner that is able to provide the defined services for the organation
    Practitioner,
    /// The practitioner can perform this role at for the organization
    Role,
    /// The list of healthcare services that this worker provides for this role's Organization/Location(s)
    Service,
    /// The practitioner has this specialty at an organization
    Specialty,
}
/// Search parameters for the Procedure resource
#[derive(Clone, Debug)]
pub enum ProcedureSearchParameter {
    /// A code to identify a  procedure
    Code,
    /// Date/Period the procedure was performed
    Date,
    /// A unique identifier for a procedure
    Identifier,
    /// Search by subject - a patient
    Patient,
    /// Search by encounter
    Encounter,
    /// A request for this procedure
    BasedOn,
    /// Classification of the procedure
    Category,
    /// Encounter or episode associated with the procedure
    Context,
    /// Instantiates protocol or definition
    Definition,
    /// Where the procedure happened
    Location,
    /// Part of referenced event
    PartOf,
    /// The reference to the practitioner
    Performer,
    /// preparation | in-progress | suspended | aborted | completed | entered-in-error | unknown
    Status,
    /// Search by subject
    Subject,
}
/// Search parameters for the ProcedureRequest resource
#[derive(Clone, Debug)]
pub enum ProcedureRequestSearchParameter {
    /// What is being requested/ordered
    Code,
    /// Identifiers assigned to this order
    Identifier,
    /// Search by subject - a patient
    Patient,
    /// An encounter in which this request is made
    Encounter,
    /// Date request signed
    Authored,
    /// What request fulfills
    BasedOn,
    /// Where procedure is going to be done
    BodySite,
    /// Encounter or Episode during which request was created
    Context,
    /// Protocol or definition
    Definition,
    /// proposal | plan | order +
    Intent,
    /// When procedure should occur
    Occurrence,
    /// Requested perfomer
    Performer,
    /// Performer role
    PerformerType,
    /// routine | urgent | asap | stat
    Priority,
    /// What request replaces
    Replaces,
    /// Individual making the request
    Requester,
    /// Composite Request ID
    Requisition,
    /// Specimen to be tested
    Specimen,
    /// draft | active | suspended | completed | entered-in-error | cancelled
    Status,
    /// Search by subject
    Subject,
}
/// Search parameters for the ProcessRequest resource
#[derive(Clone, Debug)]
pub enum ProcessRequestSearchParameter {
    /// The action requested by this resource
    Action,
    /// The business identifier of the ProcessRequest
    Identifier,
    /// The organization who generated this request
    Organization,
    /// The provider who regenerated this request
    Provider,
}
/// Search parameters for the ProcessResponse resource
#[derive(Clone, Debug)]
pub enum ProcessResponseSearchParameter {
    /// The business identifier of the Explanation of Benefit
    Identifier,
    /// The organization who generated this resource
    Organization,
    /// The reference to the claim
    Request,
    /// The Organization who is responsible the request transaction
    RequestOrganization,
    /// The Provider who is responsible the request transaction
    RequestProvider,
}
/// Search parameters for the Provenance resource
#[derive(Clone, Debug)]
pub enum ProvenanceSearchParameter {
    /// Who participated
    Agent,
    /// What the agents role was
    AgentRole,
    /// End time with inclusive boundary, if not ongoing
    End,
    /// Identity of entity
    EntityId,
    /// Identity of entity
    EntityRef,
    /// Where the activity occurred, if relevant
    Location,
    /// Target Reference(s) (usually version specific)
    Patient,
    /// When the activity was recorded / updated
    Recorded,
    /// Indication of the reason the entity signed the object(s)
    SignatureType,
    /// Starting time with inclusive boundary
    Start,
    /// Target Reference(s) (usually version specific)
    Target,
}
/// Search parameters for the Questionnaire resource
#[derive(Clone, Debug)]
pub enum QuestionnaireSearchParameter {
    /// A code that corresponds to one of its items in the questionnaire
    Code,
    /// The questionnaire publication date
    Date,
    /// The description of the questionnaire
    Description,
    /// The time during which the questionnaire is intended to be in use
    Effective,
    /// External identifier for the questionnaire
    Identifier,
    /// Intended jurisdiction for the questionnaire
    Jurisdiction,
    /// Computationally friendly name of the questionnaire
    Name,
    /// Name of the publisher of the questionnaire
    Publisher,
    /// The current status of the questionnaire
    Status,
    /// The human-friendly name of the questionnaire
    Title,
    /// The uri that identifies the questionnaire
    Url,
    /// The business version of the questionnaire
    Version,
}
/// Search parameters for the QuestionnaireResponse resource
#[derive(Clone, Debug)]
pub enum QuestionnaireResponseSearchParameter {
    /// The author of the questionnaire response
    Author,
    /// When the questionnaire response was last changed
    Authored,
    /// Plan/proposal/order fulfilled by this questionnaire response
    BasedOn,
    /// Encounter or episode associated with the questionnaire response
    Context,
    /// The unique identifier for the questionnaire response
    Identifier,
    /// Procedure or observation this questionnaire response was performed as a part of
    Parent,
    /// The patient that is the subject of the questionnaire response
    Patient,
    /// The questionnaire the answers are provided for
    Questionnaire,
    /// The individual providing the information reflected in the questionnaire respose
    Source,
    /// The status of the questionnaire response
    Status,
    /// The subject of the questionnaire response
    Subject,
}
/// Search parameters for the ReferralRequest resource
#[derive(Clone, Debug)]
pub enum ReferralRequestSearchParameter {
    /// Who the referral is about
    Patient,
    /// The type of the referral
    Type,
    /// Creation or activation date
    AuthoredOn,
    /// Request being fulfilled
    BasedOn,
    /// Part of encounter or episode of care
    Context,
    /// Instantiates protocol or definition
    Definition,
    /// Originating encounter
    Encounter,
    /// Part of common request
    GroupIdentifier,
    /// Business identifier
    Identifier,
    /// Proposal, plan or order
    Intent,
    /// When the service(s) requested in the referral should occur
    OccurrenceDate,
    /// The priority assigned to the referral
    Priority,
    /// The person that the referral was sent to
    Recipient,
    /// Request(s) replaced by this request
    Replaces,
    /// Individual making the request
    Requester,
    /// Actions requested as part of the referral
    Service,
    /// The specialty that the referral is for
    Specialty,
    /// The status of the referral
    Status,
    /// Patient referred to care or transfer
    Subject,
}
/// Search parameters for the RelatedPerson resource
#[derive(Clone, Debug)]
pub enum RelatedPersonSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postal code specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// The Related Person's date of birth
    Birthdate,
    /// A value in an email contact
    Email,
    /// Gender of the related person
    Gender,
    /// A value in a phone contact
    Phone,
    /// A portion of name using some kind of phonetic matching algorithm
    Phonetic,
    /// The value in any kind of contact
    Telecom,
    /// Indicates if the related person record is active
    Active,
    /// An Identifier of the RelatedPerson
    Identifier,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
    /// The patient this related person is related to
    Patient,
}
/// Search parameters for the RequestGroup resource
#[derive(Clone, Debug)]
pub enum RequestGroupSearchParameter {
    /// The author of the request group
    Author,
    /// The date the request group was authored
    Authored,
    /// The context the request group applies to
    Context,
    /// The definition from which the request group is realized
    Definition,
    /// The encounter the request group applies to
    Encounter,
    /// The group identifier for the request group
    GroupIdentifier,
    /// External identifiers for the request group
    Identifier,
    /// The intent of the request group
    Intent,
    /// The participant in the requests in the group
    Participant,
    /// The identity of a patient to search for request groups
    Patient,
    /// The priority of the request group
    Priority,
    /// The status of the request group
    Status,
    /// The subject that the request group is about
    Subject,
}
/// Search parameters for the ResearchStudy resource
#[derive(Clone, Debug)]
pub enum ResearchStudySearchParameter {
    /// Classifications for the study
    Category,
    /// When the study began and ended
    Date,
    /// Drugs, devices, conditions, etc. under study
    Focus,
    /// Business Identifier for study
    Identifier,
    /// Geographic region(s) for study
    Jurisdiction,
    /// Used to search for the study
    Keyword,
    /// Part of larger study
    Partof,
    /// The individual responsible for the study
    Principalinvestigator,
    /// Steps followed in executing study
    Protocol,
    /// Location involved in study execution
    Site,
    /// Organization responsible for the study
    Sponsor,
    /// draft | in-progress | suspended | stopped | completed | entered-in-error
    Status,
    /// Name for this study
    Title,
}
/// Search parameters for the ResearchSubject resource
#[derive(Clone, Debug)]
pub enum ResearchSubjectSearchParameter {
    /// Start and end of participation
    Date,
    /// Business Identifier for research subject
    Identifier,
    /// Who is part of study
    Individual,
    /// Who is part of study
    Patient,
    /// candidate | enrolled | active | suspended | withdrawn | completed
    Status,
}
/// Search parameters for the RiskAssessment resource
#[derive(Clone, Debug)]
pub enum RiskAssessmentSearchParameter {
    /// When was assessment made?
    Date,
    /// Unique identifier for the assessment
    Identifier,
    /// Who/what does assessment apply to?
    Patient,
    /// Where was assessment performed?
    Encounter,
    /// Condition assessed
    Condition,
    /// Evaluation mechanism
    Method,
    /// Who did assessment?
    Performer,
    /// Likelihood of specified outcome
    Probability,
    /// Likelihood of specified outcome as a qualitative value
    Risk,
    /// Who/what does assessment apply to?
    Subject,
}
/// Search parameters for the Schedule resource
#[derive(Clone, Debug)]
pub enum ScheduleSearchParameter {
    /// Is the schedule in active use
    Active,
    /// The individual(HealthcareService, Practitioner, Location, ...) to find a Schedule for
    Actor,
    /// Search for Schedule resources that have a period that contains this date specified
    Date,
    /// A Schedule Identifier
    Identifier,
    /// The type of appointments that can be booked into associated slot(s)
    Type,
}
/// Search parameters for the SearchParameter resource
#[derive(Clone, Debug)]
pub enum SearchParameterSearchParameter {
    /// The resource type(s) this search parameter applies to
    Base,
    /// Code used in URL
    Code,
    /// Defines how the part works
    Component,
    /// The search parameter publication date
    Date,
    /// Original Definition for the search parameter
    DerivedFrom,
    /// The description of the search parameter
    Description,
    /// Intended jurisdiction for the search parameter
    Jurisdiction,
    /// Computationally friendly name of the search parameter
    Name,
    /// Name of the publisher of the search parameter
    Publisher,
    /// The current status of the search parameter
    Status,
    /// Types of resource (if a resource reference)
    Target,
    /// number | date | string | token | reference | composite | quantity | uri
    Type,
    /// The uri that identifies the search parameter
    Url,
    /// The business version of the search parameter
    Version,
}
/// Search parameters for the Sequence resource
#[derive(Clone, Debug)]
pub enum SequenceSearchParameter {
    /// Chromosome number of the reference sequence
    Chromosome,
    /// Search parameter for region of the reference DNA sequence string. This will refer to part of a locus or part of a gene where search region will be represented in 1-based system. Since the coordinateSystem can either be 0-based or 1-based, this search query will include the result of both coordinateSystem that contains the equivalent segment of the gene or whole genome sequence. For example, a search for sequence can be represented as `coordinate=1$lt345$gt123`, this means it will search for the Sequence resource on chromosome 1 and with position >123 and <345, where in 1-based system resource, all strings within region 1:124-344 will be revealed, while in 0-based system resource, all strings within region 1:123-344 will be revealed. You may want to check detail about 0-based v.s. 1-based above.
    Coordinate,
    /// End position (0-based exclusive, which menas the acid at this position will not be included, 1-based inclusive, which means the acid at this position will be included) of the reference sequence.
    End,
    /// The unique identity for a particular sequence
    Identifier,
    /// The subject that the observation is about
    Patient,
    /// Start position (0-based inclusive, 1-based inclusive, that means the nucleic acid or amino acid at this position will be included) of the reference sequence.
    Start,
    /// Amino Acid Sequence/ DNA Sequence / RNA Sequence
    Type,
}
/// Search parameters for the ServiceDefinition resource
#[derive(Clone, Debug)]
pub enum ServiceDefinitionSearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// The service definition publication date
    Date,
    /// What resource is being referenced
    DependsOn,
    /// What resource is being referenced
    DerivedFrom,
    /// The description of the service definition
    Description,
    /// The time during which the service definition is intended to be in use
    Effective,
    /// External identifier for the service definition
    Identifier,
    /// Intended jurisdiction for the service definition
    Jurisdiction,
    /// Computationally friendly name of the service definition
    Name,
    /// What resource is being referenced
    Predecessor,
    /// Name of the publisher of the service definition
    Publisher,
    /// The current status of the service definition
    Status,
    /// What resource is being referenced
    Successor,
    /// The human-friendly name of the service definition
    Title,
    /// Topics associated with the module
    Topic,
    /// The uri that identifies the service definition
    Url,
    /// The business version of the service definition
    Version,
}
/// Search parameters for the Slot resource
#[derive(Clone, Debug)]
pub enum SlotSearchParameter {
    /// A Slot Identifier
    Identifier,
    /// The Schedule Resource that we are seeking a slot within
    Schedule,
    /// The type of appointments that can be booked into the slot
    SlotType,
    /// Appointment date/time.
    Start,
    /// The free/busy status of the appointment
    Status,
}
/// Search parameters for the Specimen resource
#[derive(Clone, Debug)]
pub enum SpecimenSearchParameter {
    /// The accession number associated with the specimen
    Accession,
    /// The code for the body site from where the specimen originated
    Bodysite,
    /// The date the specimen was collected
    Collected,
    /// Who collected the specimen
    Collector,
    /// The kind of specimen container
    Container,
    /// The unique identifier associated with the specimen container
    ContainerId,
    /// The unique identifier associated with the specimen
    Identifier,
    /// The parent of the specimen
    Parent,
    /// The patient the specimen comes from
    Patient,
    /// available | unavailable | unsatisfactory | entered-in-error
    Status,
    /// The subject of the specimen
    Subject,
    /// The specimen type
    Type,
}
/// Search parameters for the StructureDefinition resource
#[derive(Clone, Debug)]
pub enum StructureDefinitionSearchParameter {
    /// Whether the structure is abstract
    Abstract,
    /// Definition that this type is constrained/specialized from
    Base,
    /// Path that identifies the base element
    BasePath,
    /// resource | datatype | extension
    ContextType,
    /// The structure definition publication date
    Date,
    /// specialization | constraint - How relates to base definition
    Derivation,
    /// The description of the structure definition
    Description,
    /// For testing purposes, not real usage
    Experimental,
    /// Where the extension can be used in instances
    ExtContext,
    /// External identifier for the structure definition
    Identifier,
    /// Intended jurisdiction for the structure definition
    Jurisdiction,
    /// A code for the profile
    Keyword,
    /// primitive-type | complex-type | resource | logical
    Kind,
    /// Computationally friendly name of the structure definition
    Name,
    /// A path that is constrained in the profile
    Path,
    /// Name of the publisher of the structure definition
    Publisher,
    /// The current status of the structure definition
    Status,
    /// The human-friendly name of the structure definition
    Title,
    /// Type defined or constrained by this structure
    Type,
    /// The uri that identifies the structure definition
    Url,
    /// A vocabulary binding reference
    Valueset,
    /// The business version of the structure definition
    Version,
}
/// Search parameters for the StructureMap resource
#[derive(Clone, Debug)]
pub enum StructureMapSearchParameter {
    /// The structure map publication date
    Date,
    /// The description of the structure map
    Description,
    /// External identifier for the structure map
    Identifier,
    /// Intended jurisdiction for the structure map
    Jurisdiction,
    /// Computationally friendly name of the structure map
    Name,
    /// Name of the publisher of the structure map
    Publisher,
    /// The current status of the structure map
    Status,
    /// The human-friendly name of the structure map
    Title,
    /// The uri that identifies the structure map
    Url,
    /// The business version of the structure map
    Version,
}
/// Search parameters for the Subscription resource
#[derive(Clone, Debug)]
pub enum SubscriptionSearchParameter {
    /// A tag to be added to the resource matching the criteria
    AddTag,
    /// Contact details for the subscription
    Contact,
    /// The search rules used to determine when to send a notification
    Criteria,
    /// The mime-type of the notification payload
    Payload,
    /// The current state of the subscription
    Status,
    /// The type of channel for the sent notifications
    Type,
    /// The uri that will receive the notifications
    Url,
}
/// Search parameters for the Substance resource
#[derive(Clone, Debug)]
pub enum SubstanceSearchParameter {
    /// The category of the substance
    Category,
    /// The code of the substance or ingredient
    Code,
    /// Identifier of the package/container
    ContainerIdentifier,
    /// Expiry date of package or container of substance
    Expiry,
    /// Unique identifier for the substance
    Identifier,
    /// Amount of substance in the package
    Quantity,
    /// active | inactive | entered-in-error
    Status,
    /// A component of the substance
    SubstanceReference,
}
/// Search parameters for the SupplyDelivery resource
#[derive(Clone, Debug)]
pub enum SupplyDeliverySearchParameter {
    /// External identifier
    Identifier,
    /// Patient for whom the item is supplied
    Patient,
    /// Who collected the Supply
    Receiver,
    /// in-progress | completed | abandoned | entered-in-error
    Status,
    /// Dispenser
    Supplier,
}
/// Search parameters for the SupplyRequest resource
#[derive(Clone, Debug)]
pub enum SupplyRequestSearchParameter {
    /// When the request was made
    Date,
    /// Unique identifier
    Identifier,
    /// The kind of supply (central, non-stock, etc.)
    Category,
    /// Individual making the request
    Requester,
    /// draft | active | suspended +
    Status,
    /// Who is intended to fulfill the request
    Supplier,
}
/// Search parameters for the Task resource
#[derive(Clone, Debug)]
pub enum TaskSearchParameter {
    /// Search by creation date
    AuthoredOn,
    /// Search by requests this task is based on
    BasedOn,
    /// Search by business status
    BusinessStatus,
    /// Search by task code
    Code,
    /// Search by encounter or episode
    Context,
    /// Search by task focus
    Focus,
    /// Search by group identifier
    GroupIdentifier,
    /// Search for a task instance by its business identifier
    Identifier,
    /// Search by task intent
    Intent,
    /// Search by last modification date
    Modified,
    /// Search by responsible organization
    Organization,
    /// Search by task owner
    Owner,
    /// Search by task this task is part of
    PartOf,
    /// Search by patient
    Patient,
    /// Search by recommended type of performer (e.g., Requester, Performer, Scheduler).
    Performer,
    /// Search by period Task is/was underway
    Period,
    /// Search by task priority
    Priority,
    /// Search by task requester
    Requester,
    /// Search by task status
    Status,
    /// Search by subject
    Subject,
}
/// Search parameters for the TestReport resource
#[derive(Clone, Debug)]
pub enum TestReportSearchParameter {
    /// An external identifier for the test report
    Identifier,
    /// The test report generation date
    Issued,
    /// The reference to a participant in the test execution
    Participant,
    /// The result disposition of the test execution
    Result,
    /// The name of the testing organization
    Tester,
    /// The test script executed to produce this report
    Testscript,
}
/// Search parameters for the TestScript resource
#[derive(Clone, Debug)]
pub enum TestScriptSearchParameter {
    /// The test script publication date
    Date,
    /// The description of the test script
    Description,
    /// External identifier for the test script
    Identifier,
    /// Intended jurisdiction for the test script
    Jurisdiction,
    /// Computationally friendly name of the test script
    Name,
    /// Name of the publisher of the test script
    Publisher,
    /// The current status of the test script
    Status,
    /// TestScript required and validated capability
    TestscriptCapability,
    /// The human-friendly name of the test script
    Title,
    /// The uri that identifies the test script
    Url,
    /// The business version of the test script
    Version,
}
/// Search parameters for the ValueSet resource
#[derive(Clone, Debug)]
pub enum ValueSetSearchParameter {
    /// The value set publication date
    Date,
    /// The description of the value set
    Description,
    /// Uniquely identifies this expansion
    Expansion,
    /// External identifier for the value set
    Identifier,
    /// Intended jurisdiction for the value set
    Jurisdiction,
    /// Computationally friendly name of the value set
    Name,
    /// Name of the publisher of the value set
    Publisher,
    /// A code system included or excluded in the value set or an imported value set
    Reference,
    /// The current status of the value set
    Status,
    /// The human-friendly name of the value set
    Title,
    /// The uri that identifies the value set
    Url,
    /// The business version of the value set
    Version,
}
/// Search parameters for the VisionPrescription resource
#[derive(Clone, Debug)]
pub enum VisionPrescriptionSearchParameter {
    /// Return prescriptions with this external identifier
    Identifier,
    /// The identity of a patient to list dispenses for
    Patient,
    /// Return prescriptions with this encounter identifier
    Encounter,
    /// Return prescriptions written on this date
    Datewritten,
    /// Who authorizes the vision product
    Prescriber,
}
impl ResourceWithSearchParameters for resources::Account {
    /// Parameters that can be used when searching Account resources
    type Params = AccountSearchParameter;
}
impl ResourceWithSearchParameters for resources::ActivityDefinition {
    /// Parameters that can be used when searching ActivityDefinition resources
    type Params = ActivityDefinitionSearchParameter;
}
impl ResourceWithSearchParameters for resources::AdverseEvent {
    /// Parameters that can be used when searching AdverseEvent resources
    type Params = AdverseEventSearchParameter;
}
impl ResourceWithSearchParameters for resources::AllergyIntolerance {
    /// Parameters that can be used when searching AllergyIntolerance resources
    type Params = AllergyIntoleranceSearchParameter;
}
impl ResourceWithSearchParameters for resources::Appointment {
    /// Parameters that can be used when searching Appointment resources
    type Params = AppointmentSearchParameter;
}
impl ResourceWithSearchParameters for resources::AppointmentResponse {
    /// Parameters that can be used when searching AppointmentResponse resources
    type Params = AppointmentResponseSearchParameter;
}
impl ResourceWithSearchParameters for resources::AuditEvent {
    /// Parameters that can be used when searching AuditEvent resources
    type Params = AuditEventSearchParameter;
}
impl ResourceWithSearchParameters for resources::Basic {
    /// Parameters that can be used when searching Basic resources
    type Params = BasicSearchParameter;
}
impl ResourceWithSearchParameters for resources::Binary {
    /// Parameters that can be used when searching Binary resources
    type Params = BinarySearchParameter;
}
impl ResourceWithSearchParameters for resources::BodySite {
    /// Parameters that can be used when searching BodySite resources
    type Params = BodySiteSearchParameter;
}
impl ResourceWithSearchParameters for resources::Bundle {
    /// Parameters that can be used when searching Bundle resources
    type Params = BundleSearchParameter;
}
impl ResourceWithSearchParameters for resources::CapabilityStatement {
    /// Parameters that can be used when searching CapabilityStatement resources
    type Params = CapabilityStatementSearchParameter;
}
impl ResourceWithSearchParameters for resources::CarePlan {
    /// Parameters that can be used when searching CarePlan resources
    type Params = CarePlanSearchParameter;
}
impl ResourceWithSearchParameters for resources::CareTeam {
    /// Parameters that can be used when searching CareTeam resources
    type Params = CareTeamSearchParameter;
}
impl ResourceWithSearchParameters for resources::ChargeItem {
    /// Parameters that can be used when searching ChargeItem resources
    type Params = ChargeItemSearchParameter;
}
impl ResourceWithSearchParameters for resources::Claim {
    /// Parameters that can be used when searching Claim resources
    type Params = ClaimSearchParameter;
}
impl ResourceWithSearchParameters for resources::ClaimResponse {
    /// Parameters that can be used when searching ClaimResponse resources
    type Params = ClaimResponseSearchParameter;
}
impl ResourceWithSearchParameters for resources::ClinicalImpression {
    /// Parameters that can be used when searching ClinicalImpression resources
    type Params = ClinicalImpressionSearchParameter;
}
impl ResourceWithSearchParameters for resources::CodeSystem {
    /// Parameters that can be used when searching CodeSystem resources
    type Params = CodeSystemSearchParameter;
}
impl ResourceWithSearchParameters for resources::Communication {
    /// Parameters that can be used when searching Communication resources
    type Params = CommunicationSearchParameter;
}
impl ResourceWithSearchParameters for resources::CommunicationRequest {
    /// Parameters that can be used when searching CommunicationRequest resources
    type Params = CommunicationRequestSearchParameter;
}
impl ResourceWithSearchParameters for resources::CompartmentDefinition {
    /// Parameters that can be used when searching CompartmentDefinition resources
    type Params = CompartmentDefinitionSearchParameter;
}
impl ResourceWithSearchParameters for resources::Composition {
    /// Parameters that can be used when searching Composition resources
    type Params = CompositionSearchParameter;
}
impl ResourceWithSearchParameters for resources::ConceptMap {
    /// Parameters that can be used when searching ConceptMap resources
    type Params = ConceptMapSearchParameter;
}
impl ResourceWithSearchParameters for resources::Condition {
    /// Parameters that can be used when searching Condition resources
    type Params = ConditionSearchParameter;
}
impl ResourceWithSearchParameters for resources::Consent {
    /// Parameters that can be used when searching Consent resources
    type Params = ConsentSearchParameter;
}
impl ResourceWithSearchParameters for resources::Contract {
    /// Parameters that can be used when searching Contract resources
    type Params = ContractSearchParameter;
}
impl ResourceWithSearchParameters for resources::Coverage {
    /// Parameters that can be used when searching Coverage resources
    type Params = CoverageSearchParameter;
}
impl ResourceWithSearchParameters for resources::DataElement {
    /// Parameters that can be used when searching DataElement resources
    type Params = DataElementSearchParameter;
}
impl ResourceWithSearchParameters for resources::DetectedIssue {
    /// Parameters that can be used when searching DetectedIssue resources
    type Params = DetectedIssueSearchParameter;
}
impl ResourceWithSearchParameters for resources::Device {
    /// Parameters that can be used when searching Device resources
    type Params = DeviceSearchParameter;
}
impl ResourceWithSearchParameters for resources::DeviceComponent {
    /// Parameters that can be used when searching DeviceComponent resources
    type Params = DeviceComponentSearchParameter;
}
impl ResourceWithSearchParameters for resources::DeviceMetric {
    /// Parameters that can be used when searching DeviceMetric resources
    type Params = DeviceMetricSearchParameter;
}
impl ResourceWithSearchParameters for resources::DeviceRequest {
    /// Parameters that can be used when searching DeviceRequest resources
    type Params = DeviceRequestSearchParameter;
}
impl ResourceWithSearchParameters for resources::DeviceUseStatement {
    /// Parameters that can be used when searching DeviceUseStatement resources
    type Params = DeviceUseStatementSearchParameter;
}
impl ResourceWithSearchParameters for resources::DiagnosticReport {
    /// Parameters that can be used when searching DiagnosticReport resources
    type Params = DiagnosticReportSearchParameter;
}
impl ResourceWithSearchParameters for resources::DocumentManifest {
    /// Parameters that can be used when searching DocumentManifest resources
    type Params = DocumentManifestSearchParameter;
}
impl ResourceWithSearchParameters for resources::DocumentReference {
    /// Parameters that can be used when searching DocumentReference resources
    type Params = DocumentReferenceSearchParameter;
}
impl ResourceWithSearchParameters for resources::EligibilityRequest {
    /// Parameters that can be used when searching EligibilityRequest resources
    type Params = EligibilityRequestSearchParameter;
}
impl ResourceWithSearchParameters for resources::EligibilityResponse {
    /// Parameters that can be used when searching EligibilityResponse resources
    type Params = EligibilityResponseSearchParameter;
}
impl ResourceWithSearchParameters for resources::Encounter {
    /// Parameters that can be used when searching Encounter resources
    type Params = EncounterSearchParameter;
}
impl ResourceWithSearchParameters for resources::Endpoint {
    /// Parameters that can be used when searching Endpoint resources
    type Params = EndpointSearchParameter;
}
impl ResourceWithSearchParameters for resources::EnrollmentRequest {
    /// Parameters that can be used when searching EnrollmentRequest resources
    type Params = EnrollmentRequestSearchParameter;
}
impl ResourceWithSearchParameters for resources::EnrollmentResponse {
    /// Parameters that can be used when searching EnrollmentResponse resources
    type Params = EnrollmentResponseSearchParameter;
}
impl ResourceWithSearchParameters for resources::EpisodeOfCare {
    /// Parameters that can be used when searching EpisodeOfCare resources
    type Params = EpisodeOfCareSearchParameter;
}
impl ResourceWithSearchParameters for resources::ExpansionProfile {
    /// Parameters that can be used when searching ExpansionProfile resources
    type Params = ExpansionProfileSearchParameter;
}
impl ResourceWithSearchParameters for resources::ExplanationOfBenefit {
    /// Parameters that can be used when searching ExplanationOfBenefit resources
    type Params = ExplanationOfBenefitSearchParameter;
}
impl ResourceWithSearchParameters for resources::FamilyMemberHistory {
    /// Parameters that can be used when searching FamilyMemberHistory resources
    type Params = FamilyMemberHistorySearchParameter;
}
impl ResourceWithSearchParameters for resources::Flag {
    /// Parameters that can be used when searching Flag resources
    type Params = FlagSearchParameter;
}
impl ResourceWithSearchParameters for resources::Goal {
    /// Parameters that can be used when searching Goal resources
    type Params = GoalSearchParameter;
}
impl ResourceWithSearchParameters for resources::GraphDefinition {
    /// Parameters that can be used when searching GraphDefinition resources
    type Params = GraphDefinitionSearchParameter;
}
impl ResourceWithSearchParameters for resources::Group {
    /// Parameters that can be used when searching Group resources
    type Params = GroupSearchParameter;
}
impl ResourceWithSearchParameters for resources::GuidanceResponse {
    /// Parameters that can be used when searching GuidanceResponse resources
    type Params = GuidanceResponseSearchParameter;
}
impl ResourceWithSearchParameters for resources::HealthcareService {
    /// Parameters that can be used when searching HealthcareService resources
    type Params = HealthcareServiceSearchParameter;
}
impl ResourceWithSearchParameters for resources::ImagingManifest {
    /// Parameters that can be used when searching ImagingManifest resources
    type Params = ImagingManifestSearchParameter;
}
impl ResourceWithSearchParameters for resources::ImagingStudy {
    /// Parameters that can be used when searching ImagingStudy resources
    type Params = ImagingStudySearchParameter;
}
impl ResourceWithSearchParameters for resources::Immunization {
    /// Parameters that can be used when searching Immunization resources
    type Params = ImmunizationSearchParameter;
}
impl ResourceWithSearchParameters for resources::ImmunizationRecommendation {
    /// Parameters that can be used when searching ImmunizationRecommendation resources
    type Params = ImmunizationRecommendationSearchParameter;
}
impl ResourceWithSearchParameters for resources::ImplementationGuide {
    /// Parameters that can be used when searching ImplementationGuide resources
    type Params = ImplementationGuideSearchParameter;
}
impl ResourceWithSearchParameters for resources::Library {
    /// Parameters that can be used when searching Library resources
    type Params = LibrarySearchParameter;
}
impl ResourceWithSearchParameters for resources::Linkage {
    /// Parameters that can be used when searching Linkage resources
    type Params = LinkageSearchParameter;
}
impl ResourceWithSearchParameters for resources::List {
    /// Parameters that can be used when searching List resources
    type Params = ListSearchParameter;
}
impl ResourceWithSearchParameters for resources::Location {
    /// Parameters that can be used when searching Location resources
    type Params = LocationSearchParameter;
}
impl ResourceWithSearchParameters for resources::Measure {
    /// Parameters that can be used when searching Measure resources
    type Params = MeasureSearchParameter;
}
impl ResourceWithSearchParameters for resources::MeasureReport {
    /// Parameters that can be used when searching MeasureReport resources
    type Params = MeasureReportSearchParameter;
}
impl ResourceWithSearchParameters for resources::Media {
    /// Parameters that can be used when searching Media resources
    type Params = MediaSearchParameter;
}
impl ResourceWithSearchParameters for resources::Medication {
    /// Parameters that can be used when searching Medication resources
    type Params = MedicationSearchParameter;
}
impl ResourceWithSearchParameters for resources::MedicationAdministration {
    /// Parameters that can be used when searching MedicationAdministration resources
    type Params = MedicationAdministrationSearchParameter;
}
impl ResourceWithSearchParameters for resources::MedicationDispense {
    /// Parameters that can be used when searching MedicationDispense resources
    type Params = MedicationDispenseSearchParameter;
}
impl ResourceWithSearchParameters for resources::MedicationRequest {
    /// Parameters that can be used when searching MedicationRequest resources
    type Params = MedicationRequestSearchParameter;
}
impl ResourceWithSearchParameters for resources::MedicationStatement {
    /// Parameters that can be used when searching MedicationStatement resources
    type Params = MedicationStatementSearchParameter;
}
impl ResourceWithSearchParameters for resources::MessageDefinition {
    /// Parameters that can be used when searching MessageDefinition resources
    type Params = MessageDefinitionSearchParameter;
}
impl ResourceWithSearchParameters for resources::MessageHeader {
    /// Parameters that can be used when searching MessageHeader resources
    type Params = MessageHeaderSearchParameter;
}
impl ResourceWithSearchParameters for resources::NamingSystem {
    /// Parameters that can be used when searching NamingSystem resources
    type Params = NamingSystemSearchParameter;
}
impl ResourceWithSearchParameters for resources::NutritionOrder {
    /// Parameters that can be used when searching NutritionOrder resources
    type Params = NutritionOrderSearchParameter;
}
impl ResourceWithSearchParameters for resources::Observation {
    /// Parameters that can be used when searching Observation resources
    type Params = ObservationSearchParameter;
}
impl ResourceWithSearchParameters for resources::OperationDefinition {
    /// Parameters that can be used when searching OperationDefinition resources
    type Params = OperationDefinitionSearchParameter;
}
impl ResourceWithSearchParameters for resources::Organization {
    /// Parameters that can be used when searching Organization resources
    type Params = OrganizationSearchParameter;
}
impl ResourceWithSearchParameters for resources::Patient {
    /// Parameters that can be used when searching Patient resources
    type Params = PatientSearchParameter;
}
impl ResourceWithSearchParameters for resources::PaymentNotice {
    /// Parameters that can be used when searching PaymentNotice resources
    type Params = PaymentNoticeSearchParameter;
}
impl ResourceWithSearchParameters for resources::PaymentReconciliation {
    /// Parameters that can be used when searching PaymentReconciliation resources
    type Params = PaymentReconciliationSearchParameter;
}
impl ResourceWithSearchParameters for resources::Person {
    /// Parameters that can be used when searching Person resources
    type Params = PersonSearchParameter;
}
impl ResourceWithSearchParameters for resources::PlanDefinition {
    /// Parameters that can be used when searching PlanDefinition resources
    type Params = PlanDefinitionSearchParameter;
}
impl ResourceWithSearchParameters for resources::Practitioner {
    /// Parameters that can be used when searching Practitioner resources
    type Params = PractitionerSearchParameter;
}
impl ResourceWithSearchParameters for resources::PractitionerRole {
    /// Parameters that can be used when searching PractitionerRole resources
    type Params = PractitionerRoleSearchParameter;
}
impl ResourceWithSearchParameters for resources::Procedure {
    /// Parameters that can be used when searching Procedure resources
    type Params = ProcedureSearchParameter;
}
impl ResourceWithSearchParameters for resources::ProcedureRequest {
    /// Parameters that can be used when searching ProcedureRequest resources
    type Params = ProcedureRequestSearchParameter;
}
impl ResourceWithSearchParameters for resources::ProcessRequest {
    /// Parameters that can be used when searching ProcessRequest resources
    type Params = ProcessRequestSearchParameter;
}
impl ResourceWithSearchParameters for resources::ProcessResponse {
    /// Parameters that can be used when searching ProcessResponse resources
    type Params = ProcessResponseSearchParameter;
}
impl ResourceWithSearchParameters for resources::Provenance {
    /// Parameters that can be used when searching Provenance resources
    type Params = ProvenanceSearchParameter;
}
impl ResourceWithSearchParameters for resources::Questionnaire {
    /// Parameters that can be used when searching Questionnaire resources
    type Params = QuestionnaireSearchParameter;
}
impl ResourceWithSearchParameters for resources::QuestionnaireResponse {
    /// Parameters that can be used when searching QuestionnaireResponse resources
    type Params = QuestionnaireResponseSearchParameter;
}
impl ResourceWithSearchParameters for resources::ReferralRequest {
    /// Parameters that can be used when searching ReferralRequest resources
    type Params = ReferralRequestSearchParameter;
}
impl ResourceWithSearchParameters for resources::RelatedPerson {
    /// Parameters that can be used when searching RelatedPerson resources
    type Params = RelatedPersonSearchParameter;
}
impl ResourceWithSearchParameters for resources::RequestGroup {
    /// Parameters that can be used when searching RequestGroup resources
    type Params = RequestGroupSearchParameter;
}
impl ResourceWithSearchParameters for resources::ResearchStudy {
    /// Parameters that can be used when searching ResearchStudy resources
    type Params = ResearchStudySearchParameter;
}
impl ResourceWithSearchParameters for resources::ResearchSubject {
    /// Parameters that can be used when searching ResearchSubject resources
    type Params = ResearchSubjectSearchParameter;
}
impl ResourceWithSearchParameters for resources::RiskAssessment {
    /// Parameters that can be used when searching RiskAssessment resources
    type Params = RiskAssessmentSearchParameter;
}
impl ResourceWithSearchParameters for resources::Schedule {
    /// Parameters that can be used when searching Schedule resources
    type Params = ScheduleSearchParameter;
}
impl ResourceWithSearchParameters for resources::SearchParameter {
    /// Parameters that can be used when searching SearchParameter resources
    type Params = SearchParameterSearchParameter;
}
impl ResourceWithSearchParameters for resources::Sequence {
    /// Parameters that can be used when searching Sequence resources
    type Params = SequenceSearchParameter;
}
impl ResourceWithSearchParameters for resources::ServiceDefinition {
    /// Parameters that can be used when searching ServiceDefinition resources
    type Params = ServiceDefinitionSearchParameter;
}
impl ResourceWithSearchParameters for resources::Slot {
    /// Parameters that can be used when searching Slot resources
    type Params = SlotSearchParameter;
}
impl ResourceWithSearchParameters for resources::Specimen {
    /// Parameters that can be used when searching Specimen resources
    type Params = SpecimenSearchParameter;
}
impl ResourceWithSearchParameters for resources::StructureDefinition {
    /// Parameters that can be used when searching StructureDefinition resources
    type Params = StructureDefinitionSearchParameter;
}
impl ResourceWithSearchParameters for resources::StructureMap {
    /// Parameters that can be used when searching StructureMap resources
    type Params = StructureMapSearchParameter;
}
impl ResourceWithSearchParameters for resources::Subscription {
    /// Parameters that can be used when searching Subscription resources
    type Params = SubscriptionSearchParameter;
}
impl ResourceWithSearchParameters for resources::Substance {
    /// Parameters that can be used when searching Substance resources
    type Params = SubstanceSearchParameter;
}
impl ResourceWithSearchParameters for resources::SupplyDelivery {
    /// Parameters that can be used when searching SupplyDelivery resources
    type Params = SupplyDeliverySearchParameter;
}
impl ResourceWithSearchParameters for resources::SupplyRequest {
    /// Parameters that can be used when searching SupplyRequest resources
    type Params = SupplyRequestSearchParameter;
}
impl ResourceWithSearchParameters for resources::Task {
    /// Parameters that can be used when searching Task resources
    type Params = TaskSearchParameter;
}
impl ResourceWithSearchParameters for resources::TestReport {
    /// Parameters that can be used when searching TestReport resources
    type Params = TestReportSearchParameter;
}
impl ResourceWithSearchParameters for resources::TestScript {
    /// Parameters that can be used when searching TestScript resources
    type Params = TestScriptSearchParameter;
}
impl ResourceWithSearchParameters for resources::ValueSet {
    /// Parameters that can be used when searching ValueSet resources
    type Params = ValueSetSearchParameter;
}
impl ResourceWithSearchParameters for resources::VisionPrescription {
    /// Parameters that can be used when searching VisionPrescription resources
    type Params = VisionPrescriptionSearchParameter;
}
