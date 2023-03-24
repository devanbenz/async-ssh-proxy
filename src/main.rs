use std::{process::Stdio, sync::{Arc, atomic::{AtomicU64, Ordering}}, time::Duration};

use tokio::{process::Command, net::{TcpListener, TcpStream}, time::Instant, io::AsyncWriteExt};

#[tokio::main]
async fn main() {
    env_logger::init();

    let num_conns: Arc<AtomicU64> = Default::default();

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    while let Ok((mut ingress, _)) = listener.accept().await {
        let num_conns = num_conns.clone();
        tokio::spawn(async move {
            let mut egress = TcpStream::connect("127.0.0.2:22").await.unwrap();
            num_conns.fetch_add(1, Ordering::SeqCst);

            log::debug!("{num_conns:?} connection(s) added.");

            log::debug!("{ingress:?}");

            match tokio::io::copy_bidirectional(&mut ingress, &mut egress).await {
                Ok((val, val2)) => println!("{val:?} {val2:?}"),
                Err(err) => println!("{err:?}")
            }

            num_conns.fetch_sub(1, Ordering::SeqCst);
            log::debug!("Connection(s) dropped. Current connections: {num_conns:?}.");
        });
    }   
}
