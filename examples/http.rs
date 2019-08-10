extern crate tokio_core;
extern crate web3_etz;

use web3_etz::futures::Future;

const MAX_PARALLEL_REQUESTS: usize = 64;

fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();

    let web3_etz = web3_etz::Web3::new(
        web3_etz::transports::Http::with_event_loop("http://localhost:8545", &event_loop.handle(), MAX_PARALLEL_REQUESTS)
            .unwrap(),
    );
    let accounts = web3_etz.eth().accounts().map(|accounts| {
        println!("Accounts: {:?}", accounts);
    });

    event_loop.run(accounts).unwrap();
}
