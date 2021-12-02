use lnd;
use std::fs;
use std::env;

#[tokio::main]
async fn main() {

    //Get the macaroon and cert files
    let macaroon = fs::read("admin.macaroon").unwrap();
    let cert = fs::read("tls.cert").unwrap();

    //Get the url connection string of the lnd node
    let env_lnd_url = env::var("LND_URL");
    if env_lnd_url.is_err() {
        println!("The $LND_URL environment variable could not be found.\n");
        std::process::exit(1);
    }
    let node_address = "https://".to_owned() + env_lnd_url.unwrap().as_str();

    //Make the connection to LND
    let mut lightning = match lnd::Lnd::connect_with_macaroon(node_address.clone(), &cert, &macaroon).await {
        Ok(lndconn) => lndconn,
        Err(e) => {
            println!("Could not connect to: {} using tls and macaroon", node_address);
            eprintln!("{}", e);
            std::process::exit(1);
        },
    };

    //Get basic information about this node
    match lnd::Lnd::get_info(&mut lightning).await {
        Ok(node_info) => {
            println!("{:#?}", node_info);
        }
        Err(e) => {
            eprintln!("Error connecting to LND: {:#?}", e);
        }
    }
}
