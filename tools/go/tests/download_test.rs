use proto_pdk_test_utils::*;

generate_download_install_tests!("go-test", "1.21.0");

#[test]
fn supports_linux_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("go-test", |config| {
        config.host(HostOS::Linux, HostArch::Arm64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("go".into()),
            checksum_url: Some("https://dl.google.com/go/go1.2.linux-arm64.tar.gz.sha256".into()),
            download_name: Some("go1.2.linux-arm64.tar.gz".into()),
            download_url: "https://dl.google.com/go/go1.2.linux-arm64.tar.gz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("go-test", |config| {
        config.host(HostOS::Linux, HostArch::X64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("go".into()),
            checksum_url: Some("https://dl.google.com/go/go1.2.linux-amd64.tar.gz.sha256".into()),
            download_name: Some("go1.2.linux-amd64.tar.gz".into()),
            download_url: "https://dl.google.com/go/go1.2.linux-amd64.tar.gz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("go-test", |config| {
        config.host(HostOS::MacOS, HostArch::Arm64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("go".into()),
            checksum_url: Some("https://dl.google.com/go/go1.2.darwin-arm64.tar.gz.sha256".into()),
            download_name: Some("go1.2.darwin-arm64.tar.gz".into()),
            download_url: "https://dl.google.com/go/go1.2.darwin-arm64.tar.gz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("go-test", |config| {
        config.host(HostOS::MacOS, HostArch::X64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("go".into()),
            checksum_url: Some("https://dl.google.com/go/go1.2.darwin-amd64.tar.gz.sha256".into()),
            download_name: Some("go1.2.darwin-amd64.tar.gz".into()),
            download_url: "https://dl.google.com/go/go1.2.darwin-amd64.tar.gz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("go-test", |config| {
        config.host(HostOS::Windows, HostArch::X64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("go".into()),
            checksum_url: Some("https://dl.google.com/go/go1.2.windows-amd64.zip.sha256".into()),
            download_name: Some("go1.2.windows-amd64.zip".into()),
            download_url: "https://dl.google.com/go/go1.2.windows-amd64.zip".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_freebsd_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("go-test", |config| {
        config.host(HostOS::FreeBSD, HostArch::X64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("go".into()),
            checksum_url: Some("https://dl.google.com/go/go1.2.freebsd-amd64.tar.gz.sha256".into()),
            download_name: Some("go1.2.freebsd-amd64.tar.gz".into()),
            download_url: "https://dl.google.com/go/go1.2.freebsd-amd64.tar.gz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("go-test", |config| {
        config.host(HostOS::Linux, HostArch::Arm64);
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("bin/go".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("go-test", |config| {
        config.host(HostOS::Windows, HostArch::X64);
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("bin/go.exe".into())
    );
}
