use proto_pdk_test_utils::*;

generate_download_install_tests!("node-test", "18.0.0");

// Doesn't provide macos x64 builds
#[cfg(not(target_os = "macos"))]
mod canary {
    use super::*;

    generate_download_install_tests!("node-test", "canary");
}

#[test]
fn supports_linux_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::Linux, HostArch::Arm64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-arm64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-arm64.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-arm64.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_arm() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::Linux, HostArch::Arm);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-armv7l".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-armv7l.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-armv7l.tar.xz"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::Linux, HostArch::X64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-x64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-x64.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-x64.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_s390x() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::Linux, HostArch::S390x);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-s390x".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-s390x.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-s390x.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_ppc64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::Linux, HostArch::Powerpc64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-ppc64le".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-ppc64le.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-ppc64le.tar.xz"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::MacOS, HostArch::Arm64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-darwin-arm64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-darwin-arm64.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-darwin-arm64.tar.xz"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::MacOS, HostArch::X64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-darwin-x64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-darwin-x64.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-darwin-x64.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::Windows, HostArch::Arm64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-win-arm64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-win-arm64.zip".into()),
            download_url: "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-win-arm64.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::Windows, HostArch::X64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-win-x64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-win-x64.zip".into()),
            download_url: "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-win-x64.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x86() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("node-test", |config| {
        config.host(HostOS::Windows, HostArch::X86);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("20.0.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-win-x86".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-win-x86.zip".into()),
            download_url: "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-win-x86.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("bun-test", |config| {
        config.host(HostOS::Linux, HostArch::Arm64);
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("20.0.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("bin/node".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("bun-test", |config| {
        config.host(HostOS::Windows, HostArch::X64);
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("20.0.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("node.exe".into())
    );
}
