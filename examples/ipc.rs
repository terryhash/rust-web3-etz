extern crate tokio_core;
extern crate web3_etz;

use tokio_core::reactor;
use web3_etz::futures::Future;

fn main() {
    let mut event_loop = reactor::Core::new().unwrap();
    let handle = event_loop.handle();

    let ipc = web3_etz::transports::Ipc::with_event_loop("./jsonrpc.ipc", &handle).unwrap();
    let web3_etz = web3_etz::web3_etz::new(ipc);
    println!("Calling accounts.");

    let future = web3_etz.eth().accounts().map(|accounts| {
        println!("Accounts: {:?}", accounts);
    });

    event_loop.run(future).unwrap();
}
