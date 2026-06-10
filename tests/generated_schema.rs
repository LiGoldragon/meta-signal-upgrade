use meta_signal_upgrade::schema::lib::{
    Block, BlockReason, ComponentName, ForceFlip, ForceReason, Input, InputRoute, Output,
    OutputRoute, PolicyRange, VersionLabel,
};
use meta_signal_upgrade::schema::lib::{ContractVersion, SelectorVersion};

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
        label: version_label(label),
        contract_version: contract_version(byte),
    }
}

fn range() -> PolicyRange {
    PolicyRange {
        component: ComponentName::new("persona-spirit"),
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

fn force_flip() -> ForceFlip {
    ForceFlip {
        component: ComponentName::new("persona-spirit"),
        current_version: selector_version("v0.1.0", 1),
        target_version: selector_version("v0.1.1", 2),
        reason: ForceReason::OperatorOverride,
    }
}

#[test]
fn generated_meta_input_owns_short_header_and_frame() {
    let input = Input::force_flip(force_flip());

    assert_eq!(input.route(), InputRoute::ForceFlip);

    let frame = input.encode_signal_frame().expect("encode generated input");
    let (route, decoded) = Input::decode_signal_frame(&frame).expect("decode generated input");

    assert_eq!(route, InputRoute::ForceFlip);
    assert_eq!(decoded, input);
}

#[test]
fn generated_meta_output_owns_short_header_and_frame() {
    let output = Output::blocked(Block {
        component: ComponentName::new("persona-spirit"),
        source: range().source,
        target: range().target,
        reason: BlockReason::Unsafe,
    });

    assert_eq!(output.route(), OutputRoute::Blocked);

    let frame = output
        .encode_signal_frame()
        .expect("encode generated output");
    let (route, decoded) = Output::decode_signal_frame(&frame).expect("decode generated output");

    assert_eq!(route, OutputRoute::Blocked);
    match decoded {
        Output::Blocked(block) => assert_eq!(block.reason, BlockReason::Unsafe),
        other => panic!("expected Blocked output, got {other:?}"),
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
