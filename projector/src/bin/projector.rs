use clap::Parser;

fn main () {
    let opts = src::opts::Opts::parse();
    println!("{:?}", opts)
}