use std::collections::HashMap;
use std::io::Write;
use tokio::sync::mpsc;
use std::net::Ipv4Addr;

pub async fn writer<W: Write + Send + 'static>(
    result_rx: &mut mpsc::Receiver<(u32, bool)>,
    mut out: W,
) {
    let mut result_map = HashMap::<u32, bool>::new();
    // let mut id_count: u32 = 0;

    // TODO: add buffered writing
    while let Some((from, res)) = result_rx.recv().await {
        result_map.insert(from, res);
        let from_id = Ipv4Addr::from(from).to_string();
        // println!("writer got {}: {:?}", from_id, res);

        out.write(format!("{} {}\n", from_id, res).as_bytes())
            .expect("whoops");
    }
}
