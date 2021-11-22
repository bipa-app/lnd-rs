const BUILD_DIR: &'static str = "src/gen";

const PROTOS_NS: &'static str = "protos";
const PROTOS: [&'static str; 2] = ["protos/rpc.proto", "protos/router.proto"];

fn main() -> Result<(), std::io::Error> {
    std::fs::create_dir_all(BUILD_DIR)?;

    tonic_build::configure()
        .build_server(false)
        .out_dir(BUILD_DIR)
        .format(false)
        .compile(&PROTOS, &[PROTOS_NS])
}
