use schema_rust_next::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "meta-signal-upgrade",
        "0.2.3",
        "META_SIGNAL_UPGRADE_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
