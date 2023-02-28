mod config;
mod routes;

use ceailo_site::run;
use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new();
    println!("{:?}", config);
    run().await
}
