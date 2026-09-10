#![allow(dead_code, non_camel_case_types, non_snake_case)]
pub type ComponentName = String;
pub type MigrationIdentifier = String;
pub type ContractVersion = std::vec::Vec<i64>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct MigrationVersion {
    pub first_integer: i64,
    pub second_integer: i64,
    pub third_integer: i64,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum MigrationState {
    Enabled,
    Disabled,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Registration {
    pub component_name: ComponentName,
    pub first_migration_version: MigrationVersion,
    pub second_migration_version: MigrationVersion,
    pub migration_identifier: MigrationIdentifier,
    pub migration_state: MigrationState,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PolicyRange {
    pub component_name: ComponentName,
    pub first_migration_version: MigrationVersion,
    pub second_migration_version: MigrationVersion,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum BlockReason {
    Unsafe,
    Superseded,
    NotReviewed,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct BlockRequest {
    pub component_name: ComponentName,
    pub first_migration_version: MigrationVersion,
    pub second_migration_version: MigrationVersion,
    pub block_reason: BlockReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum QueryRequest {
    All,
    Component(ComponentName),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PolicyEntry {
    pub component_name: ComponentName,
    pub first_migration_version: MigrationVersion,
    pub second_migration_version: MigrationVersion,
    pub migration_state: MigrationState,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct BlockedReply {
    pub component_name: ComponentName,
    pub first_migration_version: MigrationVersion,
    pub second_migration_version: MigrationVersion,
    pub block_reason: BlockReason,
}
pub type PolicyReport = std::vec::Vec<PolicyEntry>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum CatalogueRejectionReason {
    UnknownMigration,
    AlreadyRegistered,
    NotAllowed,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PolicyRejection {
    pub component_name: ComponentName,
    pub first_migration_version: MigrationVersion,
    pub second_migration_version: MigrationVersion,
    pub catalogue_rejection_reason: CatalogueRejectionReason,
}
pub type VersionLabel = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SelectorVersion {
    pub version_label: VersionLabel,
    pub contract_version: ContractVersion,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ForceReason {
    OperatorOverride,
    MarkerMismatchAccepted,
    EmergencyRecovery,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RollbackReason {
    PostCutoverFailure,
    OperatorOverride,
    RecoveryDrill,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum QuarantineReason {
    FailedUpgrade,
    SuspectState,
    OperatorHold,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ForceFlipRequest {
    pub component_name: ComponentName,
    pub first_selector_version: SelectorVersion,
    pub second_selector_version: SelectorVersion,
    pub force_reason: ForceReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RollbackRequest {
    pub component_name: ComponentName,
    pub first_selector_version: SelectorVersion,
    pub second_selector_version: SelectorVersion,
    pub rollback_reason: RollbackReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct QuarantineRequest {
    pub component_name: ComponentName,
    pub selector_version: SelectorVersion,
    pub quarantine_reason: QuarantineReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ForcedFlip {
    pub component_name: ComponentName,
    pub selector_version: SelectorVersion,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RollbackComplete {
    pub component_name: ComponentName,
    pub selector_version: SelectorVersion,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct QuarantineComplete {
    pub component_name: ComponentName,
    pub selector_version: SelectorVersion,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Rejection {
    pub component_name: ComponentName,
    pub selector_rejection_reason: SelectorRejectionReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    IntegrationNotLanded,
}
pub type UnimplementedRequest = UnimplementedReason;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Register(Registration),
    Allow(PolicyRange),
    Block(BlockRequest),
    Query(QueryRequest),
    ForceFlip(ForceFlipRequest),
    Rollback(RollbackRequest),
    Quarantine(QuarantineRequest),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Registered(Registration),
    Allowed(PolicyRange),
    Blocked(BlockedReply),
    PolicyReported(PolicyReport),
    PolicyRejected(PolicyRejection),
    FlipForced(ForcedFlip),
    RolledBack(RollbackComplete),
    Quarantined(QuarantineComplete),
    Rejected(Rejection),
    RequestUnimplemented(UnimplementedRequest),
}
