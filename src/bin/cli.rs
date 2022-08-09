
use chimtool::chim_core::{
    loaders::{loader::StringLoader, txt_loader::TxtLoader},
};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of file to parse
    filename: String,
}

fn main() {
    let args = Args::parse();

    let _res = TxtLoader::load(args.filename.as_str());   
}
