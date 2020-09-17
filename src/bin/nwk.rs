use gaps_rs::{opt::Nwk, build_tree::{pars, max_cut_from_file}};
use structopt::StructOpt;

fn main() {
	let opt = Nwk::from_args();

	let result = match &opt.method[..] {
		"max-cut" => max_cut_from_file(&opt.infile),
		"paup" => pars(opt),
		"phylip" => panic!("phylip pars not yet supported"),
		_ => panic!("This shouldn't happen, because structopt catches invalid inputs")
	};

	println!("{}", result);
}