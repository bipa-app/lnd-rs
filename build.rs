const BUILD_DIR: &str = "src/gen";

const PROTOS_NS: &str = "protos";
const PROTOS: &[&str] = &[
    "protos/invoices.proto",
    "protos/lightning.proto",
    "protos/router.proto",
    "protos/chainkit.proto",
];

fn main() -> Result<(), std::io::Error> {
    std::fs::create_dir_all(BUILD_DIR)?;

    tonic_build::configure()
        .build_server(false)
        .out_dir(BUILD_DIR)
        .compile_protos(PROTOS, &[PROTOS_NS])
}
