use std::time;

use web3::contract::{Contract, Options};

const GET: &str = "get";
const SET: &str = "set";

pub async fn deploy_contract(cid: String) -> web3::contract::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;

    // Get current balance
    let balance = web3.eth().balance(accounts[0], None).await?;

    println!("Balance: {}", balance);

    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("../res/FileCIDStorage.bin");
    // Deploying a contract
    let contract = Contract::deploy(web3.eth(), include_bytes!("../res/FileCIDStorage.abi"))?
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), accounts[0])
        .await?;

    println!("Deployed at: {}", contract.address());

    let before: String = contract.query(GET, (), None, Options::default(), None).await?;

    println!("before = {:?}", before);

    let result = contract.call(
        SET, (cid,), accounts[0], Options::default()
    ).await?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    let after: String = contract.query(GET, (), None, Options::default(), None).await?;

    println!("after = {:?}", after);
    Ok(())
}
