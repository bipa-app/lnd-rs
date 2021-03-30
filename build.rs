fn main() -> Result<(), std::io::Error> {
    let _ = std::fs::create_dir("src/lnrpc");

    tonic_build::configure()
        .build_server(false)
        .out_dir("src/lnrpc")
        .format(false)
        .compile(
            &[
                "protos/autopilot.proto",
                "protos/chainnotifier.proto",
                "protos/invoices.proto",
                "protos/lncli.proto",
                "protos/router.proto",
                "protos/rpc.proto",
                "protos/signer.proto",
                "protos/verrpc.proto",
                "protos/walletkit.proto",
                "protos/walletunlocker.proto",
                "protos/watchtower.proto",
                "protos/wtclient.proto",
            ],
            &["protos"],
        )
}
