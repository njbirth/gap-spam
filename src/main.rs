use gaps_rust::opt::Opt;
use structopt::StructOpt;

fn main() {
	let opt = Opt::from_args();
	match opt {
		Opt::QTrees(opt_qtrees) => {
			gaps_rust::run_qtrees(opt_qtrees).unwrap();
		},
		Opt::Pars(opt_pars) => {
			gaps_rust::run_pars(opt_pars).unwrap();
		},
		Opt::Nwk(opt_nwk) => {
			gaps_rust::run_nwk(opt_nwk).unwrap();
		},
	}
}