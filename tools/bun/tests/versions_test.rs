use proto_pdk_test_utils::*;

generate_resolve_versions_tests!("bun-test", {
    "0.4" => "0.4.0",
    "0.5.1" => "0.5.1",
    "1.1.0" => "1.1.0",
});

#[test]
fn loads_versions_from_git() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("bun-test");

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}

#[test]
fn sets_latest_alias() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("bun-test");

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}
