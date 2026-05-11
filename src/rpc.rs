use anyhow::Result;
use serde::Deserialize;
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

// Objet représentant une transaction Ethereum
#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    #[serde(rename = "isError")]
    pub is_error: String,
}

/**
 * Récupère les 5 dernières transactions d'une adresse Ethereum en utilisant l'API d'Etherscan.
 */
pub async fn get_transactions(address: &str, api_key: &str) -> Result<Vec<Transaction>> {
    let url = format!(
        "https://api.etherscan.io/v2/api?chainid=1&module=account&action=txlist&address={}&startblock=0&endblock=99999999&page=1&offset=5&sort=desc&apikey={}",
        address, api_key
    );

    let response = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    let txs = serde_json::from_value(response["result"].clone())?;
    Ok(txs)
}