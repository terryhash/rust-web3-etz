extern crate web3_etz;

use web3_etz::futures::Future;

fn main() {
    let (_el, transport) = web3::transports::Ipc::new("./jsonrpc.ipc").unwrap();
    let web3_etz = web3_etz::Web3::new(transport);

    println!("Calling accounts.");
    let accounts = web3_etz.eth().accounts().wait().unwrap();
    println!("Accounts: {:?}", accounts);

    println!("Calling balance.");
    let balance = web3_etz.eth().balance("0x0".parse().unwrap(), None).wait().unwrap();
    println!("Balance: {}", balance);
}
