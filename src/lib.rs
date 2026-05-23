//! Ordinary Signal contract for the domain-criome component.
//!
//! This crate carries provider-neutral Criome-domain observation, resolution,
//! and projection records.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

macro_rules! string_newtype {
    ($name:ident) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            NotaTransparent,
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
        )]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

string_newtype!(DomainName);
string_newtype!(NetworkAddress);
string_newtype!(RecordValue);
string_newtype!(UniformResourceLocator);
string_newtype!(DelegationName);
string_newtype!(AuthorityEndpoint);

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ResolutionScope {
    Public,
    Internal,
    Intelligent,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RecordKind {
    AddressV4,
    AddressV6,
    CanonicalName,
    Text,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RedirectStatus {
    Permanent,
    Temporary,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum PathTreatment {
    Preserve,
    Replace,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ProjectionScope {
    PublicRecords,
    RedirectRules,
    Everything,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ResolutionQuery {
    pub name: DomainName,
    pub scope: ResolutionScope,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub name: DomainName,
    pub address: NetworkAddress,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ResolutionResult {
    pub query: ResolutionQuery,
    pub addresses: Vec<Address>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct NoRecords {
    pub query: ResolutionQuery,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AuthorityDelegation {
    pub domain: DomainName,
    pub endpoint: AuthorityEndpoint,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DomainQuery {
    pub root: Option<DomainName>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DomainListing {
    pub domains: Vec<DomainName>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Delegation {
    pub name: DelegationName,
    pub domain: DomainName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DelegationQuery {
    pub domain: Option<DomainName>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DelegationListing {
    pub delegations: Vec<Delegation>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DomainNameSystemRecord {
    pub name: DomainName,
    pub kind: RecordKind,
    pub value: RecordValue,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RedirectRule {
    pub source: DomainName,
    pub target: UniformResourceLocator,
    pub status: RedirectStatus,
    pub path_treatment: PathTreatment,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ProjectionQuery {
    pub domain: DomainName,
    pub scope: ProjectionScope,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Projection {
    pub query: ProjectionQuery,
    pub records: Vec<DomainNameSystemRecord>,
    pub redirects: Vec<RedirectRule>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum Observation {
    Domains(DomainQuery),
    Delegations(DelegationQuery),
    Projection(ProjectionQuery),
}

impl Observation {
    pub fn kind(&self) -> ObservationKind {
        match self {
            Self::Domains(_) => ObservationKind::Domains,
            Self::Delegations(_) => ObservationKind::Delegations,
            Self::Projection(_) => ObservationKind::Projection,
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ObservationKind {
    Domains,
    Delegations,
    Projection,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum ObservationResult {
    Domains(DomainListing),
    Delegations(DelegationListing),
    Projection(Projection),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RejectionReason {
    DomainUnknown,
    DelegationUnknown,
    ProjectionUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestRejected {
    pub reason: RejectionReason,
}

signal_channel! {
    channel DomainCriome {
        operation Observe(Observation),
        operation Resolve(ResolutionQuery),
        operation Project(ProjectionQuery),
    }
    reply Reply {
        Observed(ObservationResult),
        Resolved(ResolutionResult),
        NoRecords(NoRecords),
        NotAuthoritative(AuthorityDelegation),
        Projected(Projection),
        RequestRejected(RequestRejected),
    }
}

pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<Reply>;

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
