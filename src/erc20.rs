use ethers::contract::abigen;
use ethers::prelude::{Http, Provider};
use ethers::types::Address;
use ethers::utils::format_units;
use std::sync::Arc;

abigen!(
    IERC20,
    r#"[
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function transfer(address recipient, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool)
        function symbol() extenral view returns (string)
        function decimals() external view returns (uint8)
    ]"#,
);

const RPC_URL: &str = "https://eth.llamarpc.com";
const WETH: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let client = Arc::new(provider);
    let weth = IERC20::new(WETH.parse::<Address>().unwrap(), client);

    // symbol
    let weth_symbol = weth.symbol().await?;

    // decimals
    let decimals = weth.decimals().await?;
    println!("Decimals: {decimals}");

    // total supply
    let total_supply = weth.total_supply().await?;
    let total_supply = format_units(total_supply, "ether").unwrap();
    println!("Total supply: {total_supply}");

    // balance
    let owner = "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640"
        .parse::<Address>()
        .unwrap();
    let balance = weth.balance_of(owner).await?;
    let balance = format_units(balance, "ether").unwrap();
    println!("Account {owner:?} has {balance} {weth_symbol}");

    Ok(())
}
