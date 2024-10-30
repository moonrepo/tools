use proto_pdk_test_utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn registers_metadata() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("python-test").await;

    assert_eq!(
        plugin.register_tool(ToolMetadataInput::default()).await,
        ToolMetadataOutput {
            name: "Python".into(),
            plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
            ..ToolMetadataOutput::default()
        }
    );
}
