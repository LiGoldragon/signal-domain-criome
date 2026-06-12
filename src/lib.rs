//! Ordinary Signal contract for the domain-criome component.
//!
//! This crate carries provider-neutral Criome-domain observation, resolution,
//! and projection records.

#[cfg(not(feature = "nota-text"))]
use nota_next::{Block, NotaDecodeError};
use nota_next::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

pub mod schema;

macro_rules! string_newtype {
    ($name:ident) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            NotaEncode,
            NotaDecode,
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
string_newtype!(DelegationTarget);

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ResolutionScope {
    Public,
    Internal,
    Intelligent,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RecordKind {
    AddressV4,
    AddressV6,
    CanonicalName,
    Text,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RedirectStatus {
    Permanent,
    Temporary,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum PathTreatment {
    Preserve,
    Replace,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ProjectionScope {
    PublicRecords,
    RedirectRules,
    Everything,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ResolutionQuery {
    pub name: DomainName,
    pub scope: ResolutionScope,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Address {
    pub name: DomainName,
    pub address: NetworkAddress,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ResolutionResult {
    pub query: ResolutionQuery,
    pub addresses: Vec<Address>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DomainQuery {
    pub root: Option<DomainName>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DomainListing {
    pub domains: Vec<DomainName>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Delegation {
    pub name: DelegationName,
    pub domain: DomainName,
    pub target: DelegationTarget,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DelegationQuery {
    pub domain: Option<DomainName>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DelegationListing {
    pub delegations: Vec<Delegation>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DomainNameSystemRecord {
    pub name: DomainName,
    pub kind: RecordKind,
    pub value: RecordValue,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RedirectRule {
    pub source: DomainName,
    pub target: UniformResourceLocator,
    pub status: RedirectStatus,
    pub path_treatment: PathTreatment,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ProjectionQuery {
    pub domain: DomainName,
    pub scope: ProjectionScope,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Projection {
    pub query: ProjectionQuery,
    pub records: Vec<DomainNameSystemRecord>,
    pub redirects: Vec<RedirectRule>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ObservationKind {
    Domains,
    Delegations,
    Projection,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ObservationResult {
    Domains(DomainListing),
    Delegations(DelegationListing),
    Projection(Projection),
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RejectionReason {
    DomainUnknown,
    DelegationUnknown,
    ProjectionUnavailable,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestRejected {
    pub operation: OperationKind,
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
        Projected(Projection),
        RequestRejected(RequestRejected),
    }
}

#[cfg(not(feature = "nota-text"))]
impl OperationKind {
    const fn as_nota_atom(self) -> &'static str {
        match self {
            Self::Observe => "Observe",
            Self::Resolve => "Resolve",
            Self::Project => "Project",
        }
    }

    fn from_nota_atom(atom: &str) -> Result<Self, NotaDecodeError> {
        match atom {
            "Observe" => Ok(Self::Observe),
            "Resolve" => Ok(Self::Resolve),
            "Project" => Ok(Self::Project),
            variant => Err(NotaDecodeError::UnknownVariant {
                enum_name: "OperationKind",
                variant: variant.to_owned(),
            }),
        }
    }
}

#[cfg(not(feature = "nota-text"))]
impl NotaEncode for OperationKind {
    fn to_nota(&self) -> String {
        self.as_nota_atom().to_owned()
    }
}

#[cfg(not(feature = "nota-text"))]
impl NotaDecode for OperationKind {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let atom = block
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom {
                type_name: "OperationKind",
            })?;
        Self::from_nota_atom(atom)
    }
}

pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<Reply>;

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
