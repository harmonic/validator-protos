# Validator Protos

Protobuf definitions for the Harmonic validator client.

## Proto files

Sources live in [`protos/`](protos/):

- `auth.proto`: Authentication service for obtaining access tokens for the block engine and relayer
- `block.proto`: Block stream types with explicit bundle boundaries
- `block_engine.proto`: Block engine interface for streaming packets, bundles and blocks, reporting leader slots, and endpoint discovery
- `bundle.proto`: Bundle types
- `packet.proto`: Transaction packet and metadata types
- `relayer.proto`: Relayer TPU proxy interface — socket config and packet streaming
- `shared.proto`: Common types shared across protos

## Using the Rust crate

### As a git dependency

```toml
[dependencies]
validator-protos = { git = "https://github.com/harmonic/validator-protos.git", features = ["client"] }
```

Pin to a tag for reproducible releases:

```toml
validator-protos = { git = "https://github.com/harmonic/validator-protos.git", tag = "v0.1.0", version = "0.1", features = ["client"] }
```

### Features

**At least one of `client` or `server` must be enabled**.

- `client` — generate gRPC client stubs.
- `server` — generate gRPC server traits.

```toml
[dependencies]
validator-protos = { git = "...", features = ["server"] }

[dev-dependencies]
validator-protos = { git = "...", features = ["client"] }
```

### Building locally

```sh
cargo build --all-features
```

## Updating

1. Edit the `.proto` files in `protos/`.
2. Bump the version in `Cargo.toml`:
   - **patch** (`0.x.Y`): non-breaking field additions
   - **minor** (`0.X.0`): backwards-compatible service additions
   - **major** (`X.0.0`): removed/renamed fields, changed field numbers, changed wire format
3. Tag the release: `git tag v0.1.0 && git push --tags`.
4. Consumers bump their `tag = "..."` (and `version = "..."` constraint) in lockstep.
