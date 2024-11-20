use clap::Parser;
use geo_git::{geo::color::ColorType, gui, public::parse};
#[derive(Debug, Parser)]
#[clap(version)]
struct Opt {
    /// Path to script-file of geom.
    #[clap(short, long)]
    file: String,
}

#[tokio::main]
pub async fn main() {
    println!("Hi");
    let opt = Opt::parse();
    let str = String::from_utf8(tokio::fs::read(opt.file).await.unwrap()).unwrap();
    let cfg = crate::gui::public::Config {
        color: ColorType::Gradient {
            generator: Box::new(colorgrad::preset::magma()),
            min: 0.0,
            max: 3000.0,
        },
    };
    let db = parse(str, cfg);
    gui::public::run_gui(db);
}
