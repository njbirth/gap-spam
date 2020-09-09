use gaps_rs::opt::Nwk;
use gaps_rs::tools;
use gaps_rs::build_tree;
use structopt::StructOpt;
use std::fs::{self, OpenOptions};
use std::process::{Command, Stdio};
use std::io::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let opt = Nwk::from_args();

	if opt.pars {
		println!("{}", build_tree::pars(opt));
		return;
	}

	println!("{}", build_tree::max_cut_from_file(&opt.infile));
}