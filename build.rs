fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);

    tonic_prost_build::configure()
        .build_server(false)
        // invoices.proto uses proto3 `optional`, which protoc only accepts unflagged
        // from 3.15. Ubuntu 22.04 — what bipa's CI runners are — ships 3.12.4, so
        // without this a consumer fails to build us. Still accepted by current protoc.
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir(out_dir)
        .compile_protos(
            &[
                "proto/invoices.proto",
                "proto/lightning.proto",
                "proto/router.proto",
                "proto/chainkit.proto",
            ],
            &["proto"],
        )
        .map_err(Into::into)
}
