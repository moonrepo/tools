use crate::config::DenoPluginConfig;
use extism_pdk::*;
use proto_pdk::*;
use schematic::SchemaBuilder;
use std::collections::HashMap;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

static NAME: &str = "Deno";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        config_schema: Some(SchemaBuilder::build_root::<DenoPluginConfig>()),
        minimum_proto_version: Some(Version::new(0, 42, 0)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        self_upgrade_commands: vec!["upgrade".into()],
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec![".dvmrc".into()],
        ignore: vec![],
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/denoland/deno")?
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
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64],
        ],
    )?;

    let version = &input.context.version;

    let arch = match env.arch {
        HostArch::Arm64 => "aarch64",
        HostArch::X64 => "x86_64",
        _ => unreachable!(),
    };

    let target = match env.os {
        HostOS::Linux => format!("{arch}-unknown-linux-gnu"),
        HostOS::MacOS => format!("{arch}-apple-darwin"),
        HostOS::Windows => format!("{arch}-pc-windows-msvc"),
        _ => unreachable!(),
    };

    let filename = format!("deno-{target}.zip");

    let download_url = if version.is_canary() {
        let hash = fetch_text(format!("https://dl.deno.land/canary-{target}-latest.txt"))?;

        format!("https://dl.deno.land/canary/{}/{filename}", hash.trim())
    } else if version.is_latest() {
        let tag = fetch_text("https://dl.deno.land/release-latest.txt")?;

        format!("https://dl.deno.land/release/{}/{filename}", tag.trim())
    } else {
        let config = get_tool_config::<DenoPluginConfig>()?;

        config
            .dist_url
            .replace("{version}", &version.to_string())
            .replace("{file}", &filename)
    };

    Ok(Json(DownloadPrebuiltOutput {
        download_url,
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([(
            "deno".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("deno")),
        )]),
        globals_lookup_dirs: vec![
            "$DENO_INSTALL_ROOT/bin".into(),
            "$DENO_HOME/bin".into(),
            "$HOME/.deno/bin".into(),
        ],
        ..LocateExecutablesOutput::default()
    }))
}
