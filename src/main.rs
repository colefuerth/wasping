use anyhow::{Context, Result};
use clap::Parser;
use std::io::stdout;
use std::{fs::File, io::Write};
use tokio::sync::mpsc;

mod crawler;
mod identity;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("https://ryanprairie.com"))]
    // TODO: get better root
    root: String,

    #[arg(short, long, default_value_t = String::from("web-graph.txt"))]
    output: String,

    #[arg(short, long, default_value_t = 100)]
    limit: u32,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    println!("Hello, {:?}", args);

    match args.output.as_str() {
        "-" => flytrap(args.root, stdout(), args.limit).await,
        _ => {
            let mut f = File::create(&args.output)
                .with_context(|| format!("file {} cannot be created", args.output))?;
            flytrap(args.root, f, args.limit).await;
        }
    }
    Ok(())
}

async fn flytrap<W: Write + Send + 'static>(root_url: String, mut out: W, limit: u32) {
    //out.write("hi\n".as_bytes()).expect("whoops");

    // 32 length because fuck it idk. id have to benchmark or use heuristics to get a real number
    // TODO: change to &str
    let (iden_q_tx, mut iden_q_rx) = mpsc::channel::<(String, String)>(32);

    // TODO: change to &str
    let (crawler_q_tx, mut crawler_q_rx) = mpsc::channel::<String>(32);

    let disp =
        tokio::spawn(async move { crawler::dispatcher(&mut crawler_q_rx, iden_q_tx, limit).await });

    let crawler_tx_clone = crawler_q_tx.clone();
    let iden =
        tokio::spawn(async move { identity::writer(crawler_tx_clone, &mut iden_q_rx, out).await });

    crawler_q_tx.send(root_url).await;

    disp.await;
    iden.await;
}
