use gaps_rs::opt::Nwk;
use structopt::StructOpt;

fn main() {
	let opt = Nwk::from_args();

	if opt.pars {
		println!("pars not implemented yet!");
		return;
	}

	println!("{}", gaps_rs::build_tree::max_cut_from_file(&opt.infile));
}