use rocket::error::Error;
use rocketapi::server::init_server;
use std::process::exit;

#[rocket::main]
async fn main() -> Result<(), Error> {
    // start the server
    match init_server().await {
        Ok(server) => {
            let _ = server.launch().await;
        },
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }
    Ok(())
}
