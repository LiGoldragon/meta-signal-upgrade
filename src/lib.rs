//! Signal contract for the owner-only `upgrade` surface.
//!
//! This crate carries catalogue policy authority and selector authority
//! for the `upgrade` runtime. The peer-callable upgrade attempt itself
//! lives on `signal-upgrade`; this owner contract configures whether
//! those attempts may run and provides emergency selector controls.

pub mod schema {
    #[rustfmt::skip]
    pub mod lib;
}

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_sema::SemaObservation;
use signal_upgrade::{ComponentName, MigrationIdentifier, Version as MigrationVersion};
use version_projection::{ComponentName as ProjectionComponentName, ContractVersion};

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum MigrationState {
    Enabled,
    Disabled,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Registration {
    pub component: ComponentName,
    pub source: MigrationVersion,
    pub target: MigrationVersion,
    pub migration: MigrationIdentifier,
    pub state: MigrationState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum BlockReason {
    Unsafe,
    Superseded,
    NotReviewed,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub component: ComponentName,
    pub source: MigrationVersion,
    pub target: MigrationVersion,
    pub reason: BlockReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum Query {
    All,
    Component(ComponentName),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PolicyEntry {
    pub component: ComponentName,
    pub source: MigrationVersion,
    pub target: MigrationVersion,
    pub state: MigrationState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PolicyReported {
    pub entries: Vec<PolicyEntry>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum CatalogueRejectionReason {
    UnknownMigration,
    AlreadyRegistered,
    NotAllowed,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PolicyRejected {
    pub component: ComponentName,
    pub source: MigrationVersion,
    pub target: MigrationVersion,
    pub reason: CatalogueRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct VersionLabel(String);

impl VersionLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ForceReason {
    OperatorOverride,
    MarkerMismatchAccepted,
    EmergencyRecovery,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RollbackReason {
    PostCutoverFailure,
    OperatorOverride,
    RecoveryDrill,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum QuarantineReason {
    FailedUpgrade,
    SuspectState,
    OperatorHold,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ForceFlip {
    pub component: ProjectionComponentName,
    pub current_version: SelectorVersion,
    pub target_version: SelectorVersion,
    pub reason: ForceReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Rollback {
    pub component: ProjectionComponentName,
    pub active_version: SelectorVersion,
    pub restore_version: SelectorVersion,
    pub reason: RollbackReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Quarantine {
    pub component: ProjectionComponentName,
    pub version: SelectorVersion,
    pub reason: QuarantineReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ForcedFlip {
    pub component: ProjectionComponentName,
    pub active_version: SelectorVersion,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RolledBack {
    pub component: ProjectionComponentName,
    pub active_version: SelectorVersion,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Quarantined {
    pub component: ProjectionComponentName,
    pub version: SelectorVersion,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Rejected {
    pub component: ProjectionComponentName,
    pub reason: SelectorRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    IntegrationNotLanded,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel OwnerUpgrade {
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OperationReceived {
    pub operation: OperationKind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EffectEmitted {
    pub observation: SemaObservation,
}
