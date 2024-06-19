use anyhow::Context;
use clap::Parser;
use geo_git::{gui, public::parse};
#[derive(Debug, Parser)]
#[clap(version)]
struct Opt {
    /// Path to script-file of geom.
    #[clap(short, long)]
    file: String
}

#[tokio::main]
pub async fn main() {
    let opt = Opt::parse();
    let str = String::from_utf8(tokio::fs::read(opt.file).await.unwrap()).unwrap();
    let db = parse(str);
    gui::public::run_gui(db);
}