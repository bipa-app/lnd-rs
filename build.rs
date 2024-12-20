fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);

    tonic_build::configure()
        .build_server(false)
        .out_dir(out_dir)
        .compile_protos(
            &[
                "protos/invoices.proto",
                "protos/lightning.proto",
                "protos/router.proto",
                "protos/chainkit.proto",
            ],
            &["protos"],
        )
        .map_err(Into::into)
}
