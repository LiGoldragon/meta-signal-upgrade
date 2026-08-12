use std::process::Command;

#[test]
fn default_dependency_tree_is_binary_only() {
    let output = Command::new(env!("CARGO"))
        .args(["tree", "--edges", "normal", "--no-default-features"])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("cargo tree output");
    for forbidden in ["dotos v", "dotos-codec", "signal-core"] {
        assert!(
            !tree.contains(forbidden),
            "default dependency tree unexpectedly contains {forbidden}:\n{tree}"
        );
    }
}

#[test]
fn dotos_text_feature_is_the_text_codec_boundary() {
    let output = Command::new(env!("CARGO"))
        .args([
            "tree",
            "--edges",
            "normal",
            "--no-default-features",
            "--features",
            "dotos-text",
        ])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("cargo tree output");
    assert!(
        tree.contains("dotos v"),
        "dotos-text feature should pull dotos:\n{tree}"
    );
}
