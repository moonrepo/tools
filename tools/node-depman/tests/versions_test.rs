use proto_pdk_test_utils::*;

mod npm {
    use super::*;

    generate_resolve_versions_tests!("npm-test", {
        "7" => "7.24.2",
        "8.1" => "8.1.4",
        "9.7.2" => "9.7.2",
    });

    #[test]
    fn doesnt_parse_package_manager_if_diff_name() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("npm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "yarn@1.2.3" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput { version: None }
        );
    }

    #[test]
    fn parses_package_manager() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("npm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "npm@1.2.3" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }

    #[test]
    fn parses_package_manager_with_hash() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("npm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "npm@1.2.3+sha256.c362077587b1e782e5aef3dcf85826399ae552ad66b760e2585c4ac11102243f" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }

    #[test]
    fn parses_package_manager_latest() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("npm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "npm" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("latest").unwrap()),
            }
        );
    }

    #[test]
    fn parses_engines() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("npm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "engines": { "npm": "1.2.3" } }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }

    #[test]
    fn parses_volta() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("npm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "volta": { "npm": "1.2.3" } }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }
}

mod pnpm {
    use super::*;

    generate_resolve_versions_tests!("pnpm-test", {
        "7" => "7.33.7",
        "8.1" => "8.1.1",
        "dev" => "6.23.7-202112041634",
    });

    #[test]
    fn doesnt_parse_package_manager_if_diff_name() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("pnpm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "yarn@1.2.3" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput { version: None }
        );
    }

    #[test]
    fn parses_package_manager() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("pnpm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "pnpm@1.2.3" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }

    #[test]
    fn parses_package_manager_latest() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("pnpm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "pnpm" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("latest").unwrap()),
            }
        );
    }

    #[test]
    fn parses_engines() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("pnpm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "engines": { "pnpm": "1.2.3" } }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }

    #[test]
    fn parses_volta() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("pnpm-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "volta": { "pnpm": "1.2.3" } }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }
}

mod yarn {
    use super::*;

    generate_resolve_versions_tests!("yarn-test", {
        "1" => "1.22.22",
        "2" => "2.4.3",
        "3" => "3.8.3",
        "berry" => "4.3.1",
    });

    #[test]
    fn doesnt_parse_package_manager_if_diff_name() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("yarn-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "pnpm@1.2.3" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput { version: None }
        );
    }

    #[test]
    fn parses_package_manager() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("yarn-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "yarn@1.2.3" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }

    #[test]
    fn parses_package_manager_latest() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("yarn-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "yarn" }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("latest").unwrap()),
            }
        );
    }

    #[test]
    fn parses_engines() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("yarn-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "engines": { "yarn": "1.2.3" } }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }

    #[test]
    fn parses_volta() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("yarn-test");

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "volta": { "yarn": "1.2.3" } }"#.into(),
                file: "package.json".into(),
            }),
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("1.2.3").unwrap()),
            }
        );
    }
}
