use proto_pdk_test_utils::*;
use starbase_sandbox::locate_fixture;

#[cfg(not(windows))]
generate_shims_test!(
    "schema-test",
    [],
    Some(locate_fixture("schemas").join("base.toml"))
);
