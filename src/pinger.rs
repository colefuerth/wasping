// use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use ipnet::Ipv4AddrRange;
use std::net::Ipv4Addr;
use tokio::sync::mpsc;
use tokio::task::JoinSet;
// --------------------------------
// use std::net::IpAddr;
// use std::sync::Arc;
// use tokio_icmp_echo::Pinger;
// --------------------------------
use async_process::Command;
// --------------------------------

pub async fn sender(
    result_tx: mpsc::Sender<(u32, char)>,
    timeout: Duration,
    limit: u32,
) -> Result<()> {
    // --------------------------------
    // let pinger = Pinger::new().await.unwrap();
    // let pinger = Arc::new(pinger);
    // --------------------------------
    let from_addr = Ipv4Addr::new(192, 168, 0, 0);
    let to_addr = Ipv4Addr::new(192, 168, 255, 255);
    let mut waitset = JoinSet::new();
    let delay: f32 = 1.0 / limit as f32;

    // for each address in the range, spawn a task to ping it
    for addr in Ipv4AddrRange::new(from_addr, to_addr) {
        let result_tx = result_tx.clone();
        // --------------------------------
        // let pinger = Arc::clone(&pinger);
        // waitset.spawn(async move {
        //     let ident = (addr.octets()[3] as u16) + ((addr.octets()[2] as u16) << 8);
        //     let res = pinger
        //         .ping(IpAddr::from(addr), ident, 1, timeout)
        //         .await;
        //     // make sure to encode the results for the writer
        //     match res {
        //         Ok(Some(_)) => {
        //             result_tx.send((addr.into(), '1')).await.unwrap();
        //         }
        //         Ok(None) => {
        //             result_tx.send((addr.into(), '0')).await.unwrap();
        //         }
        //         Err(e) => {
        //             result_tx.send((addr.into(), 'e')).await.unwrap();
        //             eprintln!("error: {}", e);
        //         }
        //     }
        // });
        // --------------------------------
        waitset.spawn(async move {
            let mut cmd = Command::new("/bin/ping");
            cmd.arg("-c")
                .arg("2")
                .arg("-W")
                .arg(timeout.as_secs().to_string())
                .arg(addr.to_string());
            let output = cmd.output().await;
            let output = output.unwrap();
            let status = output.status;
            if status.success() {
                result_tx.send((addr.into(), '1')).await.unwrap();
            } else {
                result_tx.send((addr.into(), '0')).await.unwrap();
            }
            println!("{addr}: {status}");
        });
        // --------------------------------
        tokio::time::sleep(Duration::from_secs_f32(delay)).await;
    }

    while let Some(res) = waitset.join_next().await {
        res?;
    }

    drop(result_tx);
    Ok(())
}
