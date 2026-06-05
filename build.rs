use std::{env, path::PathBuf};

use schema_rust_next::build::{CargoSchemaMetadata, GenerationDriver, GenerationPlan};

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/lib.schema");
        println!("cargo:rerun-if-changed=src/schema/lib.rs");
        CargoSchemaMetadata::new("meta-signal-upgrade").emit_schema_directory(&self.crate_root);

        GenerationDriver::new(GenerationPlan::wire_contract(
            &self.crate_root,
            "meta-signal-upgrade",
            "0.2.0",
        ))
        .generate()
        .expect("generate meta-signal-upgrade schema artifacts")
        .write_or_check("META_SIGNAL_UPGRADE_UPDATE_SCHEMA_ARTIFACTS")
        .expect("checked-in meta-signal-upgrade schema artifacts are fresh");
    }
}
