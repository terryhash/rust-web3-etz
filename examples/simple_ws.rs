extern crate web3_etz;

use web3_etz::futures::Future;

fn main() {
    let (_eloop, ws) = web3_etz::transports::WebSocket::new("ws://localhost:8546").unwrap();
    let web3_etz = web3_etz::Web3::new(ws);
    let accounts = web3_etz.eth().accounts().wait().unwrap();

    println!("Accountsw: {:?}", accounts);
}
