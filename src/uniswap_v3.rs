use ethers::contract::abigen;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use std::sync::Arc;

abigen!(
    UniswapV3Factory,
    r#"[
        function getPool(address,address,uint24) external view returns (address)
    ]"#,
);
abigen!(
    IQuoterV2,
    "./abi/uniswap_v3/IQuoterV2.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

const RPC_URL: &str = "https://eth.llamarpc.com";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let client = Arc::new(provider);
    let weth = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
        .parse::<Address>()
        .unwrap();
    let usdc = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
        .parse::<Address>()
        .unwrap();

    // get factory pool
    let factory_address = "0x1F98431c8aD98523631AE4a59f267346ea31F984"
        .parse::<Address>()
        .unwrap();
    let factory = UniswapV3Factory::new(factory_address, client.clone());
    let pool = factory.get_pool(weth, usdc, 500u32).await?;
    println!("WETH-USDC-500 pool address: {:?}", pool);

    // simulate swap with quoter V2
    let quoter_address = "0x61fFE014bA17989E743c5F6cB21bF9697530B21e"
        .parse::<Address>()
        .unwrap();
    let quoter = IQuoterV2::new(quoter_address, client);
    let res = quoter
        .quote_exact_input_single(QuoteExactInputSingleParams {
            token_in: weth,
            token_out: usdc,
            amount_in: U256::from(1000000000000000000usize),
            fee: 500u32,
            sqrt_price_limit_x96: U256::zero(),
        })
        .await?;
    println!("{:?}", res);

    Ok(())
}
