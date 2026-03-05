# Validator Protos

Protobuf definitions for the Harmonic validator client.

## Proto files

- `auth.proto`: Authentication service for obtaining access tokens for the block engine and relayer
- `block_engine.proto`: Block engine interface for streaming packets and bundles, reporting leader slots, and endpoint discovery
- `bundle.proto`: Bundle types
- `packet.proto`: Transaction packet and metadata types
- `relayer.proto`: Relayer TPU proxy interface — socket config and packet streaming
- `shared.proto`: Common types shared across protos
