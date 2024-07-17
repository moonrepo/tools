# Python plugin (experimental)

[Python](https://www.python.org/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

```shell
proto install python
```

This plugin is built-in to proto, but if you want to override it with an explicit version, add the following to `.prototools`.

```toml
[plugins]
python = "source:https://github.com/moonrepo/python-plugin/releases/download/vX.Y.Z/python_plugin.wasm"
```

## Configuration

Python plugin does not support configuration.

## Hooks

Python plugin does not support hooks.

## Caveats

This will install a pre-built version from [indygreg/python-build-standalone](https://github.com/indygreg/python-build-standalone), which doesn't support all versions, only Python 3.

Building from source directly (with `python-build`), and supporting Python 2, will be fully supported in the future.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install python-test
proto list-remote python-test
```
