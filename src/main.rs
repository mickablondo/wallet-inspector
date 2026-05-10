mod cli;
mod rpc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    cli::run().await;
}