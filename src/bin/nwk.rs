use gaps_rs::{opt::Nwk, tools, build_tree::{pars, max_cut_from_file}};
use structopt::StructOpt;

fn main() {
	let opt = Nwk::from_args();
	let result = if opt.pars { pars(opt) } else { max_cut_from_file(&opt.infile) };
	println!("{}", result);
}