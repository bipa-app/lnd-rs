use lnd;
use std::fs;
use std::env;
use std::thread;

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
    let mut lightning;
    match lnd::Lnd::connect_with_macaroon(node_address.clone(), &cert, &macaroon).await {
        Ok(lndconn) => {
            lightning = lndconn;
        }
        Err(e) => {
            println!("Could not connect to: {} using tls and macaroon", node_address);
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    //Enter a loop of pulling invoices
    let mut current_index: u64 = 0;
    loop {

        //Get a list of invoices
        match lnd::Lnd::list_invoices(&mut lightning, false, current_index.clone(), 100, false).await {
            Ok(response) => {
                for invoice in response.invoices {
                    println!("{} - {} - {} sats", invoice.add_index, invoice.settle_date, invoice.amt_paid_sat);
                    if invoice.add_index > current_index {
                        current_index = invoice.add_index;
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }

        //Current index
        println!("Current index: {}", current_index);

        thread::sleep(std::time::Duration::from_millis(15000));
    }
}
