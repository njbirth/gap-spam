use gaps_rs::opt::Nwk;
use gaps_rs::tools;
use structopt::StructOpt;
use std::fs::{self, OpenOptions};
use std::process::{Command, Stdio};
use std::io::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let opt = Nwk::from_args();

	if opt.pars {
		pars(opt);
		return;
	}

	println!("{}", gaps_rs::build_tree::max_cut_from_file(&opt.infile));
}

fn pars(opt: Nwk) {
	// Create temporary folder
	let tmp_folder = tools::create_tmp_folder();

	// Some files
	let mut phy_f = tmp_folder.clone();
	phy_f.push("pars.phy");
	let mut nex_f = tmp_folder.clone();
	nex_f.push("pars.nex");
	let mut nwk_f = tmp_folder.clone();
	nwk_f.push("pars.nwk");

	// infile -> tmp/pars.phy
	fs::copy(opt.infile, phy_f).unwrap();

	// tmp/pars.phy -> tmp/pars.nex (with seqmagick)
	let mut seqmagick = Command::new("seqmagick")
		.arg("convert")
		.arg("pars.phy")
		.arg("pars.nex")
		.arg("--alphabet")
		.arg("protein")
		.current_dir(&tmp_folder)
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn().unwrap();

	seqmagick.wait().unwrap();

	// Append to pars.nex
	let mut nexfile = OpenOptions::new().append(true).open(nex_f).expect("Unable to open file");
	nexfile.write_all(b"\nbegin paup;\nset maxtrees=1000;\nset increase=auto;\nHSearch addseq=random nreps=20;\nSaveTrees format=newick file=pars.nwk replace=yes;\nquit;\nend;").expect("Unable to write nexfile");

	// tmp/pars.nex -> tmp/pars.nwk (with paup)
	let stdout = if opt.verbose { Stdio::inherit() } else { Stdio::null() };
	let mut paup = Command::new("paup")
		.arg("pars.nex")
		.current_dir(&tmp_folder)
		.stdout(stdout)
		.spawn().unwrap();

	paup.wait().unwrap();

	// Output tree(s)
	let mut lines = BufReader::new(File::open(nwk_f).expect("Unable to open file")).lines();
	if opt.all {
		for line in lines {
			println!("{}", line.unwrap());
		}
	}
	else {
		println!("{}", lines.next().unwrap().unwrap());
	}

	// Delete temporary folder
	fs::remove_dir_all(tmp_folder).unwrap();
}