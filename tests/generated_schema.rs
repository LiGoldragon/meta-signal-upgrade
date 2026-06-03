use meta_signal_upgrade::schema::lib::{
    Block, BlockReason, ComponentName, ForceFlip, ForceReason, Input, InputRoute, NexusAction,
    NexusActionRoute, NexusWork, ObjectName, OriginRoute, Output, OutputRoute, PolicyRange,
    SemaWriteInput, SemaWriteInputRoute, SemaWriteOutput, SignalObjectName, TraceEvent,
    VersionLabel,
};
use meta_signal_upgrade::schema::lib::{ContractVersion, SelectorVersion};

fn version_label(value: &str) -> VersionLabel {
    String::from(value)
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
        component: ComponentName::from("persona-spirit"),
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
        component: ComponentName::from("persona-spirit"),
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
fn generated_meta_signal_nexus_sema_projection_routes_force_flip() {
    let work = NexusWork::signal_arrived(Input::force_flip(force_flip()))
        .with_origin_route(OriginRoute(7));
    let action = work.into_nexus_action();

    assert_eq!(action.origin_route(), OriginRoute(7));
    assert_eq!(action.root().route(), NexusActionRoute::CommandSemaWrite);
    match action.root() {
        NexusAction::CommandSemaWrite(SemaWriteInput::ForceFlip(payload)) => {
            assert_eq!(payload.component, "persona-spirit");
        }
        other => panic!("expected ForceFlip SEMA write, got {other:?}"),
    }

    let sema_input = action.into_sema_write_input();
    assert_eq!(sema_input.origin_route(), OriginRoute(7));
    assert_eq!(sema_input.root().route(), SemaWriteInputRoute::ForceFlip);
}

#[test]
fn generated_meta_sema_reply_projects_back_to_signal_output() {
    let output = SemaWriteOutput::blocked(Block {
        component: ComponentName::from("persona-spirit"),
        source: range().source,
        target: range().target,
        reason: BlockReason::Unsafe,
    })
    .with_origin_route(OriginRoute(11))
    .into_nexus_work()
    .into_nexus_action()
    .into_signal_output();

    assert_eq!(output.origin_route(), OriginRoute(11));
    assert_eq!(output.root().route(), OutputRoute::Blocked);
    match output.into_root() {
        Output::Blocked(block) => assert_eq!(block.reason, BlockReason::Unsafe),
        other => panic!("expected Blocked output, got {other:?}"),
    }
}

#[test]
fn generated_meta_trace_vocabulary_names_meta_operation() {
    let trace = TraceEvent::new(ObjectName::Signal(SignalObjectName::Input(
        InputRoute::ForceFlip,
    )));

    assert_eq!(trace.name(), "SignalInputForceFlip");
}
