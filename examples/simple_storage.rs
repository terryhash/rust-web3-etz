//based on examples/contract.rs
extern crate rustc_hex;
extern crate web3_etz;

use std::time;
use web3_etz::contract::{Contract, Options};
use web3_etz::futures::Future;
use web3_etz::types::U256;

fn main() {
    let (_eloop, transport) = web3_etz::transports::Http::new("http://localhost:8545").unwrap();
    let web3_etz = web3_etz::Web3::new(transport);
    let accounts = web3_etz.eth().accounts().wait().unwrap();

    //Get current balance
    let balance = web3_etz.eth().balance(accounts[0], None).wait().unwrap();

    println!("Balance: {}", balance);

    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("./build/SimpleStorage.bin");
    // Deploying a contract
    let contract = Contract::deploy(web3_etz.eth(), include_bytes!("./build/SimpleStorage.abi"))
        .unwrap()
        .confirmations(0)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), accounts[0])
        .unwrap()
        .wait()
        .unwrap();

    println!("{}", contract.address());

    //interact with the contract
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.wait().unwrap();
    println!("{}", storage);

    //Change state of the contract
    contract.call("set", (42,), accounts[0], Options::default());

    //View changes made
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.wait().unwrap();
    println!("{}", storage);
}
