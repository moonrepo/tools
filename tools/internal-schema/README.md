# Schema-based plugin

A WASM plugin that powers [proto](https://github.com/moonrepo/proto)'s [TOML plugin](https://moonrepo.dev/docs/proto/toml-plugin) pattern. This plugin is responsible for parsing the TOML schema file and providing the necessary information to proto by implementing the applicable WASM functions.

## Installation

This plugin is built-in to proto, but if you want to override it with an explicit version, add the following to `.prototools`.

```toml
[plugins]
internal-schema = "source:https://github.com/moonrepo/schema-plugin/releases/download/vX.Y.Z/schema_plugin.wasm"
```

## Configuration

This plugin does not support configuration.

## Hooks

This plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install moon-test
proto list-remote moon-test
```

> Since this plugin requires an external schema file, its testing uses moon: https://moonrepo.dev/docs/install#proto
