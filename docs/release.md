# Release workflow

Releases require [cargo-release](https://crates.io/crates/cargo-release).

```
cargo binstall cargo-release --force
```

## Adding changelogs

Before releaseing a plugin, changelog entries can be added to `CHANGELOG.md` under an unreleased header, like so.

```md
## Unreleased

- Changelog entry goes here.
```

## Releasing plugins

Plugin based Rust crates are _not_ published to crates.io but are still released using `cargo-release`. We use this library to help bump the version, tag the release, and push back to the repository. This can be achieved with the following command:

```
cargo release <bump> -p <crate> --no-publish
```

Once the tag is pushed, the `release.yml` GitHub workflow will build the WASM plugin in release mode, and create a GitHub release.

> Note: Plugins _must_ be published individually, because if more than 8 tags are pushed at once, the GitHub workflow will _not_ run!
