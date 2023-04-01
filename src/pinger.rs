use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use ipnet::Ipv4AddrRange;
use std::net::{IpAddr, Ipv4Addr};
use tokio::sync::mpsc;
use tokio::task::JoinSet;

pub async fn sender(
    // queue_rx: &mut mpsc::Receiver<(u32, u32)>,
    result_tx: mpsc::Sender<(u32, char)>,
    timeout: Duration,
    // limit: u32,
) -> Result<()> {
    let pinger = tokio_icmp_echo::Pinger::new().await.unwrap();
    let from_addr = Ipv4Addr::new(192, 168, 0, 0);
    let to_addr = Ipv4Addr::new(192, 168, 255, 255);
    // use an arc to share the client between tasks
    // each client will ping an address in the address range
    let pinger = Arc::new(pinger);
    let mut waitset = JoinSet::new();

    // for each address in the range, spawn a task to ping it
    for addr in Ipv4AddrRange::new(from_addr, to_addr) {
        let pinger = Arc::clone(&pinger);
        let result_tx = result_tx.clone();
        waitset.spawn(async move {
            let ident = (addr.octets()[3] as u16) + ((addr.octets()[2] as u16) << 8);
            let res = pinger
                .ping(IpAddr::from(addr), ident, 1, timeout)
                .await;
            // if result is ok and a time, then return true
            // if result is ok and None, then return false
            // if result is err, then error
            match res {
                Ok(Some(_)) => {
                    result_tx.send((addr.into(), '1')).await.unwrap();
                }
                Ok(None) => {
                    result_tx.send((addr.into(), '0')).await.unwrap();
                }
                Err(e) => {
                    result_tx.send((addr.into(), 'e')).await.unwrap();
                    eprintln!("error: {}", e);
                }
            }
        });
    }

    // wait for all tasks to finish
    while let Some(res) = waitset.join_next().await {
        res?;
    }

    // result_tx.closed().await;
    // close the result_tx
    drop(result_tx);
    Ok(())
}
