use proto_pdk_test_utils::*;
use starbase_sandbox::locate_fixture;

generate_resolve_versions_tests!(
    "schema-test",
    {
        "1.0.3" => "1.0.3",
        "1.4" => "1.4.0",
        "1.5" => "1.5.1",
    },
    Some(locate_fixture("schemas").join("base.toml"))
);

#[test]
fn loads_versions_from_git_tags() {
    let sandbox = create_empty_proto_sandbox();
    let plugin =
        sandbox.create_schema_plugin("schema-test", locate_fixture("schemas").join("base.toml"));

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}

#[test]
fn sets_latest_alias() {
    let sandbox = create_empty_proto_sandbox();
    let plugin =
        sandbox.create_schema_plugin("schema-test", locate_fixture("schemas").join("base.toml"));

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}

#[test]
fn version_pattern_supports_common_classes() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_schema_plugin(
        "schema-test",
        locate_fixture("schemas").join("version-pattern.toml"),
    );

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}
