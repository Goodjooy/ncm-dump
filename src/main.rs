use std::path::PathBuf;

use clap::Parser;
use ncm_dump::{ncm_dump, output::OriginSameOutput};
#[derive(Debug, Parser)]
#[command( version, about, long_about = None)]
pub struct Args {
    /// input .ncm files
    inputs: Vec<PathBuf>,
    #[arg(short, long, value_name = "DIR")]
    /// output mp3 or flac dir
    output: Option<PathBuf>,
    /// need create not exist dir
    #[arg(short, long)]
    mkdir: bool,
}

fn main() {
    let args = Args::parse();

    if let (true, Some(path)) = (args.mkdir, &args.output) {
        std::fs::create_dir_all(path).expect("create dir failure")
    }

    let inputs = args
        .inputs
        .into_iter()
        .map(|path| OriginSameOutput::new(path, args.output.as_deref()))
        .enumerate();

    for (id, output) in inputs {
        println!("{id} => now handling [{}]", output);
        let input = output.open().expect("cannot open Ncm file");

        ncm_dump(input, &output).expect("processing Ncm dump failure");

        println!("{id} => Done")
    }

    println!("all done")
}
