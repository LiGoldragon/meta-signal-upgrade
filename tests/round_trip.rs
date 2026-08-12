use meta_signal_upgrade::{
    BlockReason, BlockRequest, BlockedReply, CatalogueRejectionReason, ComponentName, ContractVersion, ForceFlipRequest,
    ForceReason, ForcedFlip, Frame, FrameBody, Input, InputRoute, MigrationIdentifier,
    MigrationState, MigrationVersion, Output, OutputRoute, PolicyEntry, PolicyRange,
    PolicyRejection, QuarantineComplete, QuarantineReason, QuarantineRequest, QueryRequest, Registration, Rejection,
    RollbackComplete, RollbackReason, RollbackRequest, SelectorRejectionReason, SelectorVersion,
    UnimplementedReason, VersionLabel,
};
#[cfg(feature = "dotos-text")]
use dotos::{DotosDecode, DotosEncode, DotosSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, Reply as FrameReply, SessionEpoch,
    SignalOperationHeads, SubReply,
};

#[cfg(feature = "dotos-text")]
const CANONICAL: &str = include_str!("../examples/canonical.dotos");

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

fn force_flip() -> ForceFlipRequest {
    ForceFlipRequest {
        component_name: component(),
        current: selector_version("v0.1.0", 1),
        target: selector_version("v0.1.1", 2),
        force_reason: ForceReason::OperatorOverride,
    }
}

fn rollback() -> RollbackRequest {
    RollbackRequest {
        component_name: component(),
        active: selector_version("v0.1.1", 2),
        restore: selector_version("v0.1.0", 1),
        rollback_reason: RollbackReason::PostCutoverFailure,
    }
}

fn quarantine() -> QuarantineRequest {
    QuarantineRequest {
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

#[cfg(feature = "dotos-text")]
fn encode<T: DotosEncode>(value: &T) -> String {
    value.to_dotos()
}

#[cfg(feature = "dotos-text")]
fn round_trip_dotos<T>(value: T, _expected: &str)
where
    T: DotosEncode + DotosDecode + PartialEq + std::fmt::Debug,
{
    let encoded = encode(&value);
    let recovered = DotosSource::new(&encoded).parse::<T>().expect("decode dotos");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.lines().any(|line| line == encoded),
        "examples/canonical.dotos missing line: {encoded}"
    );
}

#[test]
fn catalogue_meta_requests_round_trip_through_signal_frames() {
    let inputs = [
        Input::register(registration()),
        Input::allow(range()),
        Input::block(BlockRequest {
            component_name: component(),
            source: source(),
            target: MigrationVersion {
                major: 0,
                minor: 1,
                patch: 2,
            },
            block_reason: BlockReason::Unsafe,
        }),
        Input::query(QueryRequest::All),
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
        Output::blocked(BlockedReply {
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
        Output::policy_rejected(PolicyRejection {
            component_name: component(),
            source: source(),
            target: target(),
            catalogue_rejection_reason: CatalogueRejectionReason::UnknownMigration,
        }),
        Output::flip_forced(ForcedFlip {
            component_name: component(),
            selector_version: selector_version("v0.1.1", 2),
        }),
        Output::rolled_back(RollbackComplete {
            component_name: component(),
            selector_version: selector_version("v0.1.0", 1),
        }),
        Output::quarantined(QuarantineComplete {
            component_name: component(),
            selector_version: selector_version("v0.1.1", 2),
        }),
        Output::rejected(Rejection {
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
#[cfg(feature = "dotos-text")]
fn catalogue_canonical_dotos_examples_round_trip() {
    round_trip_dotos(
        Input::register(registration()),
        "(Register (persona-spirit (0 1 0) (0 1 1) persona-spirit-0-1-0-to-0-1-1 Enabled))",
    );
    round_trip_dotos(
        Input::allow(range()),
        "(Allow (persona-spirit (0 1 0) (0 1 1)))",
    );
    round_trip_dotos(
        Input::block(BlockRequest {
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
    round_trip_dotos(Input::query(QueryRequest::All), "(Query All)");
    round_trip_dotos(
        Output::registered(registration()),
        "(Registered (persona-spirit (0 1 0) (0 1 1) persona-spirit-0-1-0-to-0-1-1 Enabled))",
    );
    round_trip_dotos(
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
#[cfg(feature = "dotos-text")]
fn selector_canonical_dotos_examples_round_trip() {
    round_trip_dotos(
        Input::force_flip(force_flip()),
        "(ForceFlip (persona-spirit (v0.1.0 [1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1]) (v0.1.1 [2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2]) OperatorOverride))",
    );
    round_trip_dotos(
        Input::rollback(rollback()),
        "(Rollback (persona-spirit (v0.1.1 [2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2]) (v0.1.0 [1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1]) PostCutoverFailure))",
    );
    round_trip_dotos(
        Input::quarantine(quarantine()),
        "(Quarantine (persona-spirit (v0.1.1 [2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2]) FailedUpgrade))",
    );
    round_trip_dotos(
        Output::flip_forced(ForcedFlip {
            component_name: component(),
            selector_version: selector_version("v0.1.1", 2),
        }),
        "(FlipForced (persona-spirit (v0.1.1 [2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2])))",
    );
    round_trip_dotos(
        Output::rejected(Rejection {
            component_name: component(),
            selector_rejection_reason: SelectorRejectionReason::AlreadyQuarantined,
        }),
        "(Rejected (persona-spirit AlreadyQuarantined))",
    );
    round_trip_dotos(
        Output::request_unimplemented(UnimplementedReason::NotBuiltYet),
        "(RequestUnimplemented NotBuiltYet)",
    );
}
