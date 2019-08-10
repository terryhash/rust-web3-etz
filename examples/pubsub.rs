extern crate web3_etz;

use web3_etz::futures::{Future, Stream};

fn main() {
    let (_eloop, ws) = web3_etz::transports::WebSocket::new("ws://localhost:8546").unwrap();
    let web3_etz = web3_etz::web3_etz::new(ws.clone());
    let mut sub = web3_etz.eth_subscribe().subscribe_new_heads().wait().unwrap();

    println!("Got subscription id: {:?}", sub.id());

    (&mut sub)
        .take(5)
        .for_each(|x| {
            println!("Got: {:?}", x);
            Ok(())
        })
        .wait()
        .unwrap();

    sub.unsubscribe();

    drop(web3_etz);
}
