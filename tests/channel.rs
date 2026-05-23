use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_domain_criome::{
    Address, AuthorityDelegation, AuthorityEndpoint, DomainName, DomainNameSystemRecord,
    NetworkAddress, Observation, Operation, OperationKind, Projection, ProjectionQuery,
    ProjectionScope, RecordKind, RecordValue, Reply, ReplyKind, ResolutionQuery, ResolutionResult,
    ResolutionScope,
};
use signal_frame::{RequestPayload, SignalOperationHeads};

fn encode_to_text<T: NotaEncode>(value: &T) -> String {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode");
    encoder.into_string()
}

#[test]
fn operations_are_domain_verbs_without_provider_names() {
    assert_eq!(
        <Operation as SignalOperationHeads>::HEADS,
        &["Observe", "Resolve", "Project"]
    );

    let operation = Operation::Project(ProjectionQuery {
        domain: DomainName::new("goldragon.criome"),
        scope: ProjectionScope::Everything,
    });
    assert_eq!(operation.operation_kind(), OperationKind::Project);
}

#[test]
fn resolve_operation_round_trips_through_nota() {
    let operation = Operation::Resolve(ResolutionQuery {
        name: DomainName::new("goldragon.criome"),
        scope: ResolutionScope::Public,
    });

    let text = encode_to_text(&operation);
    assert_eq!(text, "(Resolve ([goldragon.criome] Public))");

    let mut decoder = Decoder::new(&text);
    let decoded = Operation::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, operation);
}

#[test]
fn projection_operation_builds_single_signal_frame_request() {
    let operation = Operation::Project(ProjectionQuery {
        domain: DomainName::new("goldragon.criome"),
        scope: ProjectionScope::PublicRecords,
    });
    let request = operation.into_request();
    assert_eq!(request.payloads().len(), 1);
}

#[test]
fn projection_reply_round_trips_through_nota() {
    let reply = Reply::Projected(Projection {
        query: ProjectionQuery {
            domain: DomainName::new("goldragon.criome"),
            scope: ProjectionScope::PublicRecords,
        },
        records: vec![DomainNameSystemRecord {
            name: DomainName::new("goldragon.criome"),
            kind: RecordKind::AddressV4,
            value: RecordValue::new("203.0.113.10"),
        }],
        redirects: vec![],
    });

    assert_eq!(reply.kind(), ReplyKind::Projected);
    let text = encode_to_text(&reply);
    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn resolution_reply_round_trips_through_nota() {
    let query = ResolutionQuery {
        name: DomainName::new("goldragon.criome"),
        scope: ResolutionScope::Public,
    };
    let reply = Reply::Resolved(ResolutionResult {
        query: query.clone(),
        addresses: vec![Address {
            name: query.name,
            address: NetworkAddress::new("203.0.113.10"),
        }],
    });

    let text = encode_to_text(&reply);
    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn not_authoritative_reply_carries_authority_endpoint() {
    let reply = Reply::NotAuthoritative(AuthorityDelegation {
        domain: DomainName::new("goldragon.criome"),
        endpoint: AuthorityEndpoint::new("domain-criome://goldragon.criome"),
    });

    assert_eq!(reply.kind(), ReplyKind::NotAuthoritative);

    let text = encode_to_text(&reply);
    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn ordinary_contract_has_no_provider_vocabulary() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("manifest");
    let source = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("source");

    assert!(!manifest.contains("signal-core"));
    assert!(!source.contains("Cloudflare"));
    assert!(!source.contains("Google"));
    assert!(!source.contains("Hetzner"));
}

#[test]
fn observation_kind_is_typed() {
    let observation = Observation::Projection(ProjectionQuery {
        domain: DomainName::new("goldragon.criome"),
        scope: ProjectionScope::RedirectRules,
    });
    assert_eq!(
        observation.kind(),
        signal_domain_criome::ObservationKind::Projection
    );
}
