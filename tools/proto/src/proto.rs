use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: "proto".into(),
        type_of: PluginType::CLI,
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        self_upgrade_commands: vec!["up".into(), "upgrade".into()],
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/moonrepo/proto")?
        .into_iter()
        .filter_map(|tag| tag.strip_prefix('v').map(|tag| tag.to_owned()))
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        "proto",
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64],
        ],
    )?;

    let version = input.context.version;
    let arch = env.arch.to_rust_arch();

    if version.is_canary() {
        return Err(plugin_err!(PluginError::UnsupportedCanary {
            tool: "proto".into()
        }));
    }

    let target = match env.os {
        HostOS::Linux => format!("{arch}-unknown-linux-{}", env.libc),
        HostOS::MacOS => format!("{arch}-apple-darwin"),
        HostOS::Windows => format!("{arch}-pc-windows-msvc"),
        _ => unreachable!(),
    };
    let target_name = format!("proto_cli-{target}");
    let target_ext = if env.os.is_windows() { "zip" } else { "tar.xz" };

    let download_file = format!("{target_name}.{target_ext}");
    let checksum_file = format!("{download_file}.sha256");
    let base_url = format!("https://github.com/moonrepo/proto/releases/download/v{version}");

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(target_name),
        checksum_url: Some(format!("{base_url}/{checksum_file}")),
        checksum_name: Some(checksum_file),
        download_url: format!("{base_url}/{download_file}"),
        download_name: Some(download_file),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        primary: Some(ExecutableConfig::new(env.os.get_exe_name("proto"))),
        secondary: HashMap::from_iter([(
            "proto-shim".into(),
            ExecutableConfig::new(env.os.get_exe_name("proto-shim")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}