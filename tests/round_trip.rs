use meta_signal_upgrade::{
    Block, BlockReason, CatalogueRejectionReason, EffectEmitted, EffectOutcome, ForceFlip,
    ForceReason, ForcedFlip, Frame, FrameBody, MigrationState, Operation, OperationKind,
    PolicyEntry, PolicyRange, PolicyRejected, PolicyReported, Quarantine, QuarantineReason,
    Quarantined, Query, Registration, Rejected, Reply, RequestUnimplemented, Rollback,
    RollbackReason, RolledBack, SelectorRejectionReason, SelectorVersion, UnimplementedReason,
    VersionLabel,
};
use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, SubReply,
};
use signal_upgrade::{ComponentName, MigrationIdentifier, Version as MigrationVersion};
use version_projection::{ComponentName as ProjectionComponentName, ContractVersion};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn catalogue_component() -> ComponentName {
    ComponentName::new("persona-spirit")
}

fn projection_component() -> ProjectionComponentName {
    ProjectionComponentName::new("persona-spirit")
}

fn source() -> MigrationVersion {
    MigrationVersion::new(0, 1, 0)
}

fn target() -> MigrationVersion {
    MigrationVersion::new(0, 1, 1)
}

fn migration_identifier() -> MigrationIdentifier {
    MigrationIdentifier::new("persona-spirit-0-1-0-to-0-1-1")
}

fn registration() -> Registration {
    Registration {
        component: catalogue_component(),
        source: source(),
        target: target(),
        migration: migration_identifier(),
        state: MigrationState::Enabled,
    }
}

fn range() -> PolicyRange {
    PolicyRange::new(catalogue_component(), source(), target())
}

fn contract_version(byte: u8) -> ContractVersion {
    ContractVersion::new([byte; 32])
}

fn selector_version(label: &str, byte: u8) -> SelectorVersion {
    SelectorVersion::new(VersionLabel::new(label), contract_version(byte))
}

fn force_flip() -> ForceFlip {
    ForceFlip {
        component: projection_component(),
        current_version: selector_version("v0.1.0", 1),
        target_version: selector_version("v0.1.1", 2),
        reason: ForceReason::OperatorOverride,
    }
}

fn rollback() -> Rollback {
    Rollback {
        component: projection_component(),
        active_version: selector_version("v0.1.1", 2),
        restore_version: selector_version("v0.1.0", 1),
        reason: RollbackReason::PostCutoverFailure,
    }
}

fn quarantine() -> Quarantine {
    Quarantine {
        component: projection_component(),
        version: selector_version("v0.1.1", 2),
        reason: QuarantineReason::FailedUpgrade,
    }
}

fn round_trip_request(operation: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: operation.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: FrameReply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted frame reply, got {other:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

fn encode<T: NotaEncode>(value: &T) -> String {
    value.to_nota()
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = encode(&value);
    assert_eq!(encoded, expected);

    let recovered = NotaSource::new(&encoded).parse::<T>().expect("decode nota");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn catalogue_meta_requests_round_trip_through_signal_frames() {
    let operations = [
        Operation::Register(registration()),
        Operation::Allow(range()),
        Operation::Block(Block {
            component: catalogue_component(),
            source: source(),
            target: MigrationVersion::new(0, 1, 2),
            reason: BlockReason::Unsafe,
        }),
        Operation::Query(Query::All),
    ];

    for operation in operations {
        assert_eq!(round_trip_request(operation.clone()), operation);
    }
}

#[test]
fn selector_meta_requests_round_trip_through_signal_frames() {
    let operations = [
        Operation::ForceFlip(force_flip()),
        Operation::Rollback(rollback()),
        Operation::Quarantine(quarantine()),
    ];

    for operation in operations {
        assert_eq!(round_trip_request(operation.clone()), operation);
    }
}

#[test]
fn meta_replies_round_trip_through_signal_frames() {
    let replies = [
        Reply::Registered(registration()),
        Reply::Allowed(range()),
        Reply::Blocked(Block {
            component: catalogue_component(),
            source: source(),
            target: MigrationVersion::new(0, 1, 2),
            reason: BlockReason::Unsafe,
        }),
        Reply::PolicyReported(PolicyReported {
            entries: vec![PolicyEntry {
                component: catalogue_component(),
                source: source(),
                target: target(),
                state: MigrationState::Enabled,
            }],
        }),
        Reply::PolicyRejected(PolicyRejected {
            component: catalogue_component(),
            source: source(),
            target: target(),
            reason: CatalogueRejectionReason::UnknownMigration,
        }),
        Reply::FlipForced(ForcedFlip {
            component: projection_component(),
            active_version: selector_version("v0.1.1", 2),
        }),
        Reply::RolledBack(RolledBack {
            component: projection_component(),
            active_version: selector_version("v0.1.0", 1),
        }),
        Reply::Quarantined(Quarantined {
            component: projection_component(),
            version: selector_version("v0.1.1", 2),
        }),
        Reply::Rejected(Rejected {
            component: projection_component(),
            reason: SelectorRejectionReason::AlreadyQuarantined,
        }),
        Reply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn operation_kinds_are_generated_without_attempt_handover() {
    assert_eq!(
        Operation::Register(registration()).kind(),
        OperationKind::Register
    );
    assert_eq!(
        Operation::ForceFlip(force_flip()).kind(),
        OperationKind::ForceFlip
    );
    assert_eq!(
        Operation::Rollback(rollback()).kind(),
        OperationKind::Rollback
    );
    assert_eq!(
        Operation::Quarantine(quarantine()).kind(),
        OperationKind::Quarantine
    );
}

#[test]
fn effect_event_uses_contract_owned_outcome_not_sema_observation() {
    let event = EffectEmitted {
        operation: OperationKind::ForceFlip,
        outcome: EffectOutcome::FlipForced,
    };

    let encoded = encode(&event);
    assert_eq!(encoded, "(ForceFlip FlipForced)");
    assert!(!encoded.contains("Sema"));

    let recovered = NotaSource::new(&encoded)
        .parse::<EffectEmitted>()
        .expect("decode event");
    assert_eq!(recovered, event);
}

#[test]
fn catalogue_canonical_nota_examples_round_trip() {
    round_trip_nota(
        Operation::Register(registration()),
        "(Register ([persona-spirit] (0 1 0) (0 1 1) [persona-spirit-0-1-0-to-0-1-1] Enabled))",
    );
    round_trip_nota(
        Operation::Allow(range()),
        "(Allow ([persona-spirit] (0 1 0) (0 1 1)))",
    );
    round_trip_nota(
        Operation::Block(Block {
            component: catalogue_component(),
            source: source(),
            target: MigrationVersion::new(0, 1, 2),
            reason: BlockReason::Unsafe,
        }),
        "(Block ([persona-spirit] (0 1 0) (0 1 2) Unsafe))",
    );
    round_trip_nota(Operation::Query(Query::All), "(Query All)");
    round_trip_nota(
        Reply::Registered(registration()),
        "(Registered ([persona-spirit] (0 1 0) (0 1 1) [persona-spirit-0-1-0-to-0-1-1] Enabled))",
    );
    round_trip_nota(
        Reply::PolicyReported(PolicyReported {
            entries: vec![PolicyEntry {
                component: catalogue_component(),
                source: source(),
                target: target(),
                state: MigrationState::Enabled,
            }],
        }),
        "(PolicyReported ([([persona-spirit] (0 1 0) (0 1 1) Enabled)]))",
    );
}

#[test]
fn selector_canonical_nota_examples_round_trip() {
    round_trip_nota(
        Operation::ForceFlip(force_flip()),
        "(ForceFlip (persona-spirit ([v0.1.0] #0101010101010101010101010101010101010101010101010101010101010101) ([v0.1.1] #0202020202020202020202020202020202020202020202020202020202020202) OperatorOverride))",
    );
    round_trip_nota(
        Operation::Rollback(rollback()),
        "(Rollback (persona-spirit ([v0.1.1] #0202020202020202020202020202020202020202020202020202020202020202) ([v0.1.0] #0101010101010101010101010101010101010101010101010101010101010101) PostCutoverFailure))",
    );
    round_trip_nota(
        Operation::Quarantine(quarantine()),
        "(Quarantine (persona-spirit ([v0.1.1] #0202020202020202020202020202020202020202020202020202020202020202) FailedUpgrade))",
    );
    round_trip_nota(
        Reply::FlipForced(ForcedFlip {
            component: projection_component(),
            active_version: selector_version("v0.1.1", 2),
        }),
        "(FlipForced (persona-spirit ([v0.1.1] #0202020202020202020202020202020202020202020202020202020202020202)))",
    );
    round_trip_nota(
        Reply::Rejected(Rejected {
            component: projection_component(),
            reason: SelectorRejectionReason::AlreadyQuarantined,
        }),
        "(Rejected (persona-spirit AlreadyQuarantined))",
    );
    round_trip_nota(
        Reply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
        "(RequestUnimplemented (NotBuiltYet))",
    );
}
