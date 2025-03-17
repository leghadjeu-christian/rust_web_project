use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(parse(from_os_str))]
    infile: PathBuf,

    #[structopt(parse(from_os_str))]
    outfile: PathBuf,
}

fn main() {
    let opts = Opts::from_args();

    println!("{:?}", opts);
}