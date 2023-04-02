use std::collections::HashMap;
use std::io::Write;
use tokio::sync::mpsc;
use std::net::Ipv4Addr;
use std::fs::File;

pub async fn writer(
    result_rx: &mut mpsc::Receiver<(u32, char)>,
    out: String,
) {
    // if ./ips does not exist yet, create it
    let mut out = File::create(out + "/ips").expect("whoops");
    let mut result_map = HashMap::new();
    // let mut id_count: u32 = 0;

    // TODO: add buffered writing
    while let Some((from, res)) = result_rx.recv().await {
        result_map.insert(from, res);
        let from_id = Ipv4Addr::from(from).to_string();
        println!("{from_id}: {res:?}");

        out.write_all(format!("{from_id},{res}\n").as_bytes())
            .expect("whoops");
    }
    drop(out);
}
