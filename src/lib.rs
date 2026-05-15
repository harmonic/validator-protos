#[cfg(not(any(feature = "client", feature = "server")))]
compile_error!(
    "validator-protos requires at least one of the `client` or `server` features to be enabled"
);

pub mod auth {
    tonic::include_proto!("auth");
}

pub mod shared {
    tonic::include_proto!("shared");
}

pub mod packet {
    tonic::include_proto!("packet");
}

pub mod bundle {
    tonic::include_proto!("bundle");
}

pub mod block_engine {
    tonic::include_proto!("block_engine");
}

pub mod relayer {
    tonic::include_proto!("relayer");
}
