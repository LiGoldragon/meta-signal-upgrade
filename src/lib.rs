//! Meta-signal contract for the meta-policy `upgrade` surface.
//!
//! This crate carries catalogue policy authority and selector authority
//! for the `upgrade` runtime. The peer-callable upgrade attempt itself
//! lives on `signal-upgrade`; this meta-signal contract configures whether
//! those attempts may run and provides emergency selector controls.

pub mod schema {
    #[rustfmt::skip]
    pub mod lib;
}

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_upgrade::{ComponentName, MigrationIdentifier, Version as MigrationVersion};
use version_projection::{ComponentName as ProjectionComponentName, ContractVersion};

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MigrationState {
    Enabled,
    Disabled,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Registration {
    pub component: ComponentName,
    pub source: MigrationVersion,
    pub target: MigrationVersion,
    pub migration: MigrationIdentifier,
    pub state: MigrationState,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct PolicyRange {
    pub component: ComponentName,
    pub source: MigrationVersion,
    pub target: MigrationVersion,
}

impl PolicyRange {
    pub fn new(
        component: ComponentName,
        source: MigrationVersion,
        target: MigrationVersion,
    ) -> Self {
        Self {
            component,
            source,
            target,
        }
    }
}

impl From<signal_upgrade::Attempt> for PolicyRange {
    fn from(attempt: signal_upgrade::Attempt) -> Self {
        Self {
            component: attempt.component,
            source: attempt.source,
            target: attempt.target,
        }
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockReason {
    Unsafe,
    Superseded,
    NotReviewed,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub component: ComponentName,
    pub source: MigrationVersion,
    pub target: MigrationVersion,
    pub reason: BlockReason,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Query {
    All,
    Component(ComponentName),
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct PolicyEntry {
    pub component: ComponentName,
    pub source: MigrationVersion,
    pub target: MigrationVersion,
    pub state: MigrationState,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct PolicyReported {
    pub entries: Vec<PolicyEntry>,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatalogueRejectionReason {
    UnknownMigration,
    AlreadyRegistered,
    NotAllowed,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct PolicyRejected {
    pub component: ComponentName,
    pub source: MigrationVersion,
    pub target: MigrationVersion,
    pub reason: CatalogueRejectionReason,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SelectorVersion {
    pub label: VersionLabel,
    pub contract_version: ContractVersion,
}

impl SelectorVersion {
    pub fn new(label: VersionLabel, contract_version: ContractVersion) -> Self {
        Self {
            label,
            contract_version,
        }
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionLabel(String);

impl VersionLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForceReason {
    OperatorOverride,
    MarkerMismatchAccepted,
    EmergencyRecovery,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RollbackReason {
    PostCutoverFailure,
    OperatorOverride,
    RecoveryDrill,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuarantineReason {
    FailedUpgrade,
    SuspectState,
    OperatorHold,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForceFlip {
    pub component: ProjectionComponentName,
    pub current_version: SelectorVersion,
    pub target_version: SelectorVersion,
    pub reason: ForceReason,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Rollback {
    pub component: ProjectionComponentName,
    pub active_version: SelectorVersion,
    pub restore_version: SelectorVersion,
    pub reason: RollbackReason,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Quarantine {
    pub component: ProjectionComponentName,
    pub version: SelectorVersion,
    pub reason: QuarantineReason,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForcedFlip {
    pub component: ProjectionComponentName,
    pub active_version: SelectorVersion,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RolledBack {
    pub component: ProjectionComponentName,
    pub active_version: SelectorVersion,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Quarantined {
    pub component: ProjectionComponentName,
    pub version: SelectorVersion,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectorRejectionReason {
    UnknownComponent,
    UnknownVersion,
    NotAllowed,
    AlreadyQuarantined,
    NotQuarantined,
    VersionQuarantined,
    HandoverRejected,
    UpgradeSocketUnavailable,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Rejected {
    pub component: ProjectionComponentName,
    pub reason: SelectorRejectionReason,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnimplementedReason {
    NotBuiltYet,
    IntegrationNotLanded,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel MetaUpgrade {
        operation Register(Registration),
        operation Allow(PolicyRange),
        operation Block(Block),
        operation Query(Query),
        operation ForceFlip(ForceFlip),
        operation Rollback(Rollback),
        operation Quarantine(Quarantine),
    }
    reply Reply {
        Registered(Registration),
        Allowed(PolicyRange),
        Blocked(Block),
        PolicyReported(PolicyReported),
        PolicyRejected(PolicyRejected),
        FlipForced(ForcedFlip),
        RolledBack(RolledBack),
        Quarantined(Quarantined),
        Rejected(Rejected),
        RequestUnimplemented(RequestUnimplemented),
    }
    observable {
        filter default;
        operation_event OperationReceived;
        effect_event EffectEmitted;
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct OperationReceived {
    pub operation: OperationKind,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct EffectEmitted {
    pub operation: OperationKind,
    pub outcome: EffectOutcome,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectOutcome {
    Registered,
    Allowed,
    Blocked,
    PolicyReported,
    PolicyRejected,
    FlipForced,
    RolledBack,
    Quarantined,
    Rejected,
    RequestUnimplemented,
    NoChange,
}
