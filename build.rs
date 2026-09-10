use ethos_zero::{Actualizing, File, Generating, Potential};
use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    println!("cargo:rerun-if-changed=ethos/signal.ethos");
    println!("cargo:rerun-if-changed=src/generated/signal.rs");
    let source = fs::read_to_string(root.join("ethos/signal.ethos")).expect("read Ethos source");
    let file = Potential::<File>::from(source)
        .actualize()
        .unwrap_or_else(|_| panic!("read Signal"));
    let generated = file
        .generate()
        .unwrap_or_else(|_| panic!("generate Signal contract"));
    let committed =
        fs::read_to_string(root.join("src/generated/signal.rs")).expect("read committed contract");
    assert_eq!(
        generated, committed,
        "committed signal.rs is stale against Ethos source"
    );
}
