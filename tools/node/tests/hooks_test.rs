// Importing proto_pdk crashes Windows because it contains WASM code
#[cfg(not(windows))]
mod node_hooks {
    use node_common::NodePluginConfig;
    use proto_pdk::InstallHook;
    use proto_pdk_test_utils::*;
    use serial_test::serial;
    use std::collections::HashSet;
    use std::env;
    use std::path::PathBuf;

    fn set_vars(path: PathBuf) {
        env::set_var("PROTO_HOME", path.to_string_lossy().to_string());
        env::set_var("PROTO_NODE_VERSION", "18.0.0");
    }

    fn reset_vars() {
        env::remove_var("PROTO_HOME");
        env::remove_var("PROTO_NODE_VERSION");
    }

    #[test]
    #[serial]
    fn installs_bundled_npm() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin_with_config("node-test", |config| {
            config.tool_config(NodePluginConfig {
                bundled_npm: true,
                ..Default::default()
            });
        });

        assert!(!sandbox.path().join(".proto/tools/npm/8.6.0").exists());

        set_vars(sandbox.path().join(".proto"));

        plugin.post_install(InstallHook::default());

        reset_vars();

        assert!(sandbox.path().join(".proto/tools/npm/8.6.0").exists());

        let manifest =
            ToolManifest::load(sandbox.path().join(".proto/tools/npm/manifest.json")).unwrap();

        assert_eq!(
            manifest.installed_versions,
            HashSet::from_iter([VersionSpec::parse("8.6.0").unwrap()])
        );

        let config = ProtoConfig::load_from(sandbox.path().join(".proto"), false).unwrap();

        assert_eq!(
            config.versions.unwrap().get("npm").unwrap(),
            &UnresolvedVersionSpec::parse("8.6.0").unwrap()
        );
    }

    #[test]
    #[serial]
    fn can_pin_bundled_npm() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin_with_config("node-test", |config| {
            config.tool_config(NodePluginConfig {
                bundled_npm: true,
                ..Default::default()
            });
        });

        set_vars(sandbox.path().join(".proto"));

        plugin.post_install(InstallHook {
            pinned: true,
            ..InstallHook::default()
        });

        reset_vars();

        let config = ProtoConfig::load_from(sandbox.path().join(".proto"), false).unwrap();

        assert_eq!(
            config.versions.unwrap().get("npm").unwrap(),
            &UnresolvedVersionSpec::parse("8.6.0").unwrap()
        );
    }

    #[test]
    #[serial]
    fn can_skip_bundled_npm() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin_with_config("node-test", |config| {
            config.tool_config(NodePluginConfig {
                bundled_npm: true,
                ..Default::default()
            });
        });

        assert!(!sandbox.path().join(".proto/tools/npm/8.6.0").exists());

        set_vars(sandbox.path().join(".proto"));

        plugin.post_install(InstallHook {
            passthrough_args: vec!["--no-bundled-npm".into()],
            ..InstallHook::default()
        });

        reset_vars();

        assert!(!sandbox.path().join(".proto/tools/npm/8.6.0").exists());
    }
}
