use clap::{Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Module to fetch from
    #[clap(value_parser, value_name = "MODULE")]
    module: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(module) = args.module.as_deref() {
        println!("Module passed: {}", module);
    }
}
