use meta_signal_upgrade::schema::lib::{
    BlockedReply, BlockReason, ComponentName, ForceFlipRequest, ForceReason, Frame, FrameBody, Input, InputRoute, Output,
    OutputRoute, PolicyRange, VersionLabel,
};
use meta_signal_upgrade::schema::lib::{ContractVersion, SelectorVersion};
use signal_frame::{ExchangeIdentifier, ExchangeLane, LaneSequence, Reply, SessionEpoch, SubReply};

const SCHEMA_SOURCE: &str = include_str!("../schema/lib.schema");
const GENERATED_SCHEMA_RUST: &str = include_str!("../src/schema/lib.rs");

fn version_label(value: &str) -> VersionLabel {
    VersionLabel::new(value)
}

fn contract_version(byte: u64) -> ContractVersion {
    ContractVersion::new(vec![byte; 32])
}

fn selector_version(label: &str, byte: u64) -> SelectorVersion {
    SelectorVersion {
        version_label: version_label(label),
        contract_version: contract_version(byte),
    }
}

fn range() -> PolicyRange {
    PolicyRange {
        component_name: ComponentName::new("persona-spirit"),
        source: meta_signal_upgrade::schema::lib::MigrationVersion {
            major: 0,
            minor: 1,
            patch: 0,
        },
        target: meta_signal_upgrade::schema::lib::MigrationVersion {
            major: 0,
            minor: 1,
            patch: 1,
        },
    }
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn force_flip() -> ForceFlipRequest {
    ForceFlipRequest {
        component_name: ComponentName::new("persona-spirit"),
        current: selector_version("v0.1.0", 1),
        target: selector_version("v0.1.1", 2),
        force_reason: ForceReason::OperatorOverride,
    }
}

#[test]
fn generated_meta_input_owns_short_header_and_frame() {
    let input = Input::force_flip(force_flip());

    assert_eq!(input.route(), InputRoute::ForceFlip);

    let frame = input.clone().into_frame(exchange());
    let bytes = frame.encode_length_prefixed().expect("encode generated input");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode generated input");

    match decoded.into_body() {
        FrameBody::Request { request, .. } => assert_eq!(request.payloads().head(), &input),
        other => panic!("expected request, got {other:?}"),
    }
}

#[test]
fn generated_meta_output_owns_short_header_and_frame() {
    let output = Output::blocked(BlockedReply {
        component_name: ComponentName::new("persona-spirit"),
        source: range().source,
        target: range().target,
        block_reason: BlockReason::Unsafe,
    });

    assert_eq!(output.route(), OutputRoute::Blocked);

    let frame = output.clone().into_reply_frame(exchange());
    let bytes = frame.encode_length_prefixed().expect("encode generated output");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode generated output");

    match decoded.into_body() {
        FrameBody::Reply { reply: Reply::Accepted { per_operation, .. }, .. } => match per_operation.into_head() {
            SubReply::Ok(Output::Blocked(block)) => assert_eq!(block.block_reason, BlockReason::Unsafe),
            other => panic!("expected Blocked output, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn generated_meta_contract_surface_excludes_runtime_plane_terms() {
    for term in [
        "NexusWork",
        "NexusAction",
        "CommandSemaWrite",
        "CommandSemaRead",
        "SemaWriteInput",
        "SemaReadInput",
        "SemaWriteOutput",
        "SemaReadOutput",
        "SignalEngine",
        "NexusEngine",
        "SemaEngine",
        "TraceEvent",
        "ObjectName",
        "SignalObjectName",
        "OriginRoute",
        "MessageIdentifier",
        "MessageSent",
        "MessageProcessed",
        "pub struct Signal<Root>",
        "pub struct Nexus<Root>",
        "pub struct Sema<Root>",
        "pub enum Plane",
        "UpgradeFrom",
        "AcceptPrevious",
    ] {
        assert!(
            !SCHEMA_SOURCE.contains(term),
            "contract schema must not declare runtime term {term}"
        );
        assert!(
            !GENERATED_SCHEMA_RUST.contains(term),
            "generated contract module must not export runtime term {term}"
        );
    }
}
