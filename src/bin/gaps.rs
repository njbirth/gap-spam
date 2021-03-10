use gaps_rs::opt::Gaps;
use structopt::StructOpt;

fn main() {
	let opt = Gaps::from_args();
	let stats = gaps_rs::run(opt).unwrap();
	println!("{}", stats);
}