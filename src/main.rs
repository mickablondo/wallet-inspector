mod cli;
mod rpc;
mod starknet;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    cli::run().await;
}