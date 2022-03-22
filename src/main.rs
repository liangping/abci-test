
use std::net::{TcpStream, ToSocketAddrs};

fn main() {
    println!("Hello, world!");
    abci();
}

fn abci() -> bool{

    let cb = tendermint_abci::ClientBuilder::default();
    let mut conn = cb.connect("tcp://127.0.0.1:26658").expect("Connected to server");
    // let info = conn.?;
    // println!("info: {:?}", info.last_block_height);
    match conn.list_snapshots() {
        Ok(snaps) => {
            println!("connected.");
            snaps.snapshots.iter().for_each(|x| {
                println!("{:?}-{:?}", x.height, x.hash);
            });
        },
        Err(e) => println!("Not connected.{}", e)
    }
    return true
}