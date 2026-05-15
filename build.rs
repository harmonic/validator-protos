fn main() -> Result<(), std::io::Error> {
    // Always use the vendored protoc from `protobuf-src` for reproducible builds.
    #[cfg(not(windows))]
    // SAFETY: build scripts run single-threaded before any user code.
    unsafe {
        std::env::set_var("PROTOC", protobuf_src::protoc());
    }

    // Collect every *.proto under protos/ and tell cargo to rebuild when any changes.
    let proto_base_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("protos");
    let protos: Vec<_> = std::fs::read_dir(&proto_base_path)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "proto"))
        .inspect(|p| println!("cargo:rerun-if-changed={}", p.display()))
        .collect();

    // Cargo features gate client/server code generation
    let build_client = std::env::var_os("CARGO_FEATURE_CLIENT").is_some();
    let build_server = std::env::var_os("CARGO_FEATURE_SERVER").is_some();

    tonic_prost_build::configure()
        // Use `bytes::Bytes` for all `bytes` fields (zero-copy on the receive path).
        .bytes(".")
        .build_client(build_client)
        .build_server(build_server)
        // `optional` is no longer experimental in protoc but tonic-prost still requires the flag.
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&protos, &[proto_base_path])?;

    Ok(())
}
