extern crate web3_etz;

use web3_etz::futures::Future;

fn main() {
    let (_eloop, http) = web3_etz::transports::Http::new("http://localhost:8545").unwrap();
    let web3_etz = web3_etz::Web3::new(http);
    let accounts = web3_etz.eth().block_number().wait().unwrap();

    println!("Accounts: {:?}", accounts);
}
