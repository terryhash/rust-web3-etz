extern crate tokio_core;
extern crate web3_etz;

use web3_etz::futures::Future;

const MAX_PARALLEL_REQUESTS: usize = 64;

fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let remote = event_loop.remote();

    let http =
        web3_etz::transports::Http::with_event_loop("http://localhost:8545", &event_loop.handle(), MAX_PARALLEL_REQUESTS)
            .unwrap();

    let web3_etz = web3_etz::web3_etz::new(web3_etz::transports::Batch::new(http));
    let _ = web3_etz.eth().accounts();

    let block = web3_etz.eth().block_number().then(|block| {
        println!("Best Block: {:?}", block);
        Ok(())
    });

    let result = web3_etz.transport().submit_batch().then(|accounts| {
        println!("Result: {:?}", accounts);
        Ok(())
    });

    remote.spawn(move |_| block);
    remote.spawn(move |_| result);

    loop {
        event_loop.turn(None);
    }
}
