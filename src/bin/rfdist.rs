use gaps_rs::opt::Rfdist;
use gaps_rs::tools;
use structopt::StructOpt;

fn main() {
	let opt = Rfdist::from_args();
	println!("{}", tools::rfdist(&opt.infile));
}