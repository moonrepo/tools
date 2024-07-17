use proto_pdk_test_utils::*;

#[test]
fn registers_metadata() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("rust-test");

    let metadata = plugin.register_tool(ToolMetadataInput {
        id: "rust-test".into(),
    });

    assert_eq!(metadata.name, "Rust");
    assert_eq!(
        metadata.default_version,
        Some(UnresolvedVersionSpec::Alias("stable".to_owned()))
    );
    assert!(metadata.inventory.disable_progress_bars);
    assert!(metadata.inventory.override_dir.is_some());
    assert!(metadata.inventory.version_suffix.is_some());
}
