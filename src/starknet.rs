/**
 * Interation avec le smart contract Cairo
 */

use anyhow::Result;
use starknet::{
    core::types::{BlockId, BlockTag, Felt, FunctionCall},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
};
use num_traits::cast::ToPrimitive;

pub async fn get_watched_count(contract_address: &str, rpc_url: &str) -> Result<u32> {
    let url = Url::parse(rpc_url)?;
    let provider = JsonRpcClient::new(HttpTransport::new(url));

    let contract = Felt::from_hex(contract_address)?;
    let selector = starknet::core::utils::get_selector_from_name("get_watched_count")?;

    let result = provider
        .call(
            FunctionCall {
                contract_address: contract,
                entry_point_selector: selector,
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await?;

    let count = result[0].to_u32().unwrap_or(0);
    Ok(count)
}