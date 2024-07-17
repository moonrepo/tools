use proto_pdk_test_utils::*;

// We use a fake home directory but rustup requires a real one!
// generate_download_install_tests!("rust-test", "1.70.0");

#[test]
fn locates_linux_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("rust-test", |config| {
        config.host(HostOS::Linux, HostArch::Arm64);
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.69.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("bin/cargo".into())
    );
}

#[test]
fn locates_macos_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("rust-test", |config| {
        config.host(HostOS::MacOS, HostArch::X64);
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.69.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("bin/cargo".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("rust-test", |config| {
        config.host(HostOS::Windows, HostArch::X86);
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.69.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("bin/cargo.exe".into())
    );
}
