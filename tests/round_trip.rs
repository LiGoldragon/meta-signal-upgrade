use meta_signal_upgrade::{
    Block, BlockReason, CatalogueRejectionReason, ComponentName, ContractVersion, ForceFlip,
    ForceReason, ForcedFlip, Frame, FrameBody, Input, InputRoute, MigrationIdentifier,
    MigrationState, MigrationVersion, Output, OutputRoute, PolicyEntry, PolicyRange,
    PolicyRejected, Quarantine, QuarantineReason, Quarantined, Query, Registration, Rejected,
    Rollback, RollbackReason, RolledBack, SelectorRejectionReason, SelectorVersion,
    UnimplementedReason, VersionLabel,
};
#[cfg(feature = "nota-text")]
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, Reply as FrameReply, SessionEpoch,
    SignalOperationHeads, SubReply,
};

#[cfg(feature = "nota-text")]
const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn component() -> ComponentName {
    ComponentName::new("persona-spirit")
}

fn source() -> MigrationVersion {
    MigrationVersion {
        major: 0,
        minor: 1,
        patch: 0,
    }
}

fn target() -> MigrationVersion {
    MigrationVersion {
        major: 0,
        minor: 1,
        patch: 1,
    }
}

fn migration_identifier() -> MigrationIdentifier {
    MigrationIdentifier::new("persona-spirit-0-1-0-to-0-1-1")
}

fn registration() -> Registration {
    Registration {
        component_name: component(),
        source: source(),
        target: target(),
        migration_identifier: migration_identifier(),
        migration_state: MigrationState::Enabled,
    }
}

fn range() -> PolicyRange {
    PolicyRange {
        component_name: component(),
        source: source(),
        target: target(),
    }
}

fn contract_version(byte: u64) -> ContractVersion {
    ContractVersion::new(vec![byte; 32])
}

fn version_label(value: &str) -> VersionLabel {
    VersionLabel::new(value)
}

fn selector_version(label: &str, byte: u64) -> SelectorVersion {
    SelectorVersion {
        version_label: version_label(label),
        contract_version: contract_version(byte),
    }
}

fn force_flip() -> ForceFlip {
    ForceFlip {
        component_name: component(),
        current: selector_version("v0.1.0", 1),
        target: selector_version("v0.1.1", 2),
        force_reason: ForceReason::OperatorOverride,
    }
}

fn rollback() -> Rollback {
    Rollback {
        component_name: component(),
        active: selector_version("v0.1.1", 2),
        restore: selector_version("v0.1.0", 1),
        rollback_reason: RollbackReason::PostCutoverFailure,
    }
}

fn quarantine() -> Quarantine {
    Quarantine {
        component_name: component(),
        selector_version: selector_version("v0.1.1", 2),
        quarantine_reason: QuarantineReason::FailedUpgrade,
    }
}

fn round_trip_input(input: Input) -> Input {
    let frame = input.clone().into_frame(exchange());
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_output(output: Output) -> Output {
    let frame = output.clone().into_reply_frame(exchange());
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

#[cfg(feature = "nota-text")]
fn encode<T: NotaEncode>(value: &T) -> String {
    value.to_nota()
}

#[cfg(feature = "nota-text")]
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
    let inputs = [
        Input::register(registration()),
        Input::allow(range()),
        Input::block(Block {
            component_name: component(),
            source: source(),
            target: MigrationVersion {
                major: 0,
                minor: 1,
                patch: 2,
            },
            block_reason: BlockReason::Unsafe,
        }),
        Input::query(Query::All),
    ];

    for input in inputs {
        assert_eq!(round_trip_input(input.clone()), input);
    }
}

#[test]
fn selector_meta_requests_round_trip_through_signal_frames() {
    let inputs = [
        Input::force_flip(force_flip()),
        Input::rollback(rollback()),
        Input::quarantine(quarantine()),
    ];

    for input in inputs {
        assert_eq!(round_trip_input(input.clone()), input);
    }
}

#[test]
fn meta_replies_round_trip_through_signal_frames() {
    let outputs = [
        Output::registered(registration()),
        Output::allowed(range()),
        Output::blocked(Block {
            component_name: component(),
            source: source(),
            target: MigrationVersion {
                major: 0,
                minor: 1,
                patch: 2,
            },
            block_reason: BlockReason::Unsafe,
        }),
        Output::policy_reported(vec![PolicyEntry {
            component_name: component(),
            source: source(),
            target: target(),
            migration_state: MigrationState::Enabled,
        }]),
        Output::policy_rejected(PolicyRejected {
            component_name: component(),
            source: source(),
            target: target(),
            catalogue_rejection_reason: CatalogueRejectionReason::UnknownMigration,
        }),
        Output::flip_forced(ForcedFlip {
            component_name: component(),
            selector_version: selector_version("v0.1.1", 2),
        }),
        Output::rolled_back(RolledBack {
            component_name: component(),
            selector_version: selector_version("v0.1.0", 1),
        }),
        Output::quarantined(Quarantined {
            component_name: component(),
            selector_version: selector_version("v0.1.1", 2),
        }),
        Output::rejected(Rejected {
            component_name: component(),
            selector_rejection_reason: SelectorRejectionReason::AlreadyQuarantined,
        }),
        Output::request_unimplemented(UnimplementedReason::NotBuiltYet),
    ];

    for output in outputs {
        assert_eq!(round_trip_output(output.clone()), output);
    }
}

#[test]
fn generated_routes_are_closed_and_attempt_handover_is_absent() {
    assert_eq!(
        Input::register(registration()).route(),
        InputRoute::Register
    );
    assert_eq!(
        Input::force_flip(force_flip()).route(),
        InputRoute::ForceFlip
    );
    assert_eq!(Input::rollback(rollback()).route(), InputRoute::Rollback);
    assert_eq!(
        Input::quarantine(quarantine()).route(),
        InputRoute::Quarantine
    );
    assert_eq!(
        Output::request_unimplemented(UnimplementedReason::NotBuiltYet).route(),
        OutputRoute::RequestUnimplemented
    );
}

#[test]
fn generated_wire_contract_exposes_signal_frame_request_heads() {
    assert!(<Input as SignalOperationHeads>::contains_head("Register"));
    assert!(<Input as SignalOperationHeads>::contains_head("ForceFlip"));
    assert!(<Input as SignalOperationHeads>::contains_head("Quarantine"));
    assert!(!<Input as SignalOperationHeads>::contains_head(
        "AttemptHandover"
    ));
}

#[test]
#[cfg(feature = "nota-text")]
fn catalogue_canonical_nota_examples_round_trip() {
    round_trip_nota(
        Input::register(registration()),
        "(Register (persona-spirit (0 1 0) (0 1 1) persona-spirit-0-1-0-to-0-1-1 Enabled))",
    );
    round_trip_nota(
        Input::allow(range()),
        "(Allow (persona-spirit (0 1 0) (0 1 1)))",
    );
    round_trip_nota(
        Input::block(Block {
            component_name: component(),
            source: source(),
            target: MigrationVersion {
                major: 0,
                minor: 1,
                patch: 2,
            },
            block_reason: BlockReason::Unsafe,
        }),
        "(Block (persona-spirit (0 1 0) (0 1 2) Unsafe))",
    );
    round_trip_nota(Input::query(Query::All), "(Query All)");
    round_trip_nota(
        Output::registered(registration()),
        "(Registered (persona-spirit (0 1 0) (0 1 1) persona-spirit-0-1-0-to-0-1-1 Enabled))",
    );
    round_trip_nota(
        Output::policy_reported(vec![PolicyEntry {
            component_name: component(),
            source: source(),
            target: target(),
            migration_state: MigrationState::Enabled,
        }]),
        "(PolicyReported [(persona-spirit (0 1 0) (0 1 1) Enabled)])",
    );
}

#[test]
#[cfg(feature = "nota-text")]
fn selector_canonical_nota_examples_round_trip() {
    round_trip_nota(
        Input::force_flip(force_flip()),
        "(ForceFlip (persona-spirit (v0.1.0 [1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1]) (v0.1.1 [2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2]) OperatorOverride))",
    );
    round_trip_nota(
        Input::rollback(rollback()),
        "(Rollback (persona-spirit (v0.1.1 [2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2]) (v0.1.0 [1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1]) PostCutoverFailure))",
    );
    round_trip_nota(
        Input::quarantine(quarantine()),
        "(Quarantine (persona-spirit (v0.1.1 [2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2]) FailedUpgrade))",
    );
    round_trip_nota(
        Output::flip_forced(ForcedFlip {
            component_name: component(),
            selector_version: selector_version("v0.1.1", 2),
        }),
        "(FlipForced (persona-spirit (v0.1.1 [2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2])))",
    );
    round_trip_nota(
        Output::rejected(Rejected {
            component_name: component(),
            selector_rejection_reason: SelectorRejectionReason::AlreadyQuarantined,
        }),
        "(Rejected (persona-spirit AlreadyQuarantined))",
    );
    round_trip_nota(
        Output::request_unimplemented(UnimplementedReason::NotBuiltYet),
        "(RequestUnimplemented NotBuiltYet)",
    );
}
