use anyhow::Result;
use serde_json::{json, Value};

/**
 * Récupère le solde d'une adresse Ethereum en utilisant l'API d'Alchemy.
 */
pub async fn get_balance(address: &str, api_key: &str) -> Result<f64> {
    let url = format!(
        "https://eth-mainnet.g.alchemy.com/v2/{}",
        api_key
    );

    let client = reqwest::Client::new();
    let body = json!({
        "jsonrpc": "2.0",
        "method": "eth_getBalance",
        "params": [address, "latest"],
        "id": 1
    });

    let response = client
        .post(&url)
        .json(&body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let hex_balance = response["result"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No result in response"))?;

    let wei = u128::from_str_radix(&hex_balance[2..], 16)?;
    let eth = wei as f64 / 1e18;

    Ok(eth)
}