use anyhow::{Context, Result};
use clap::Parser;
use std::fs::create_dir;
// use std::io::stdout;
use std::time::Duration;
use tokio::sync::mpsc;

mod pinger;
mod writer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("ips/"))]
    output: String,
    #[arg(short, long, default_value_t = 1000)]
    limit: u32,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // match args.output.as_str() {
    //     "-" => wasping(stdout(), args.limit).await.unwrap(),
    //     _ => {
    create_dir(args.output.clone()).context("Failed to create output directory")?;
    wasping(args.output, args.limit).await.unwrap();
    // }
    // }
    Ok(())
}

async fn wasping(out: String, limit: u32) -> Result<(), anyhow::Error> {
    // 32 length because fuck it idk. id have to benchmark or use heuristics to get a real number
    // TODO: change to &str
    let (result_tx, mut result_rx) = mpsc::channel::<(u32, char)>(32);

    // TODO: add a second channel so that the writer can start by reading existing data
    // and telling the agent which addresses to ping, as ranges
    // let (ranges_tx, mut ranges_rx) = mpsc::channel::<String>(32);

    let timeout = Duration::from_secs(1);

    let agent = tokio::spawn(async move { pinger::sender(result_tx, timeout, limit).await });

    let recv = tokio::spawn(async move { writer::writer(&mut result_rx, out).await });

    // crawler_q_tx.send(root_url).await?;

    let _ = agent.await?;
    recv.await?;
    Ok(())
}
