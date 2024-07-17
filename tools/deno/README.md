# Deno plugin

[Deno](https://deno.land/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

This plugin is built-in to proto, but if you want to override it with an explicit version, add the following to `.prototools`.

```toml
[plugins]
deno = "https://github.com/moonrepo/deno-plugin/releases/download/vX.Y.Z/deno_tool.wasm"
```

## Configuration

Deno plugin can be configured with a `.prototools` file.

- `dist-url` (string) - The distribution URL to download Deno archives from. Supports `{version}` and `{file}` tokens.

```toml
[tools.deno]
dist-url = "https://..."
```

## Hooks

Deno plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install deno-test
proto list-remote deno-test
```
