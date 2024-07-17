use proto_pdk_test_utils::*;
use starbase_sandbox::locate_fixture;

#[test]
fn registers_metadata() {
    let sandbox = create_empty_proto_sandbox();
    let plugin =
        sandbox.create_schema_plugin("schema-test", locate_fixture("schemas").join("base.toml"));

    assert_eq!(
        plugin.register_tool(ToolMetadataInput::default()),
        ToolMetadataOutput {
            name: "moon-test".into(),
            type_of: PluginType::CLI,
            plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
            ..ToolMetadataOutput::default()
        }
    );
}
