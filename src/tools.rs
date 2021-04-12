use std::env;
use std::{fs, str};
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};

pub fn create_tmp_folder() -> PathBuf {
	let mut tmp_dir = env::temp_dir();
	let r: u64 = rand::random();
	tmp_dir.push(format!("gaps_rs_{}", r));
	fs::create_dir(&tmp_dir).unwrap();
	tmp_dir
}

// Requires single_quartet_check to be in path
pub fn qcheck(qtreefile: &str, fastafile: &str, nwkfile: &str) -> (u64, u64) {
	let output = Command::new("single_quartet_check")
		.arg(fastafile)
		.arg(qtreefile)
		.arg(nwkfile)
		.output()
		.expect("Failed to execute qcheck");

	let lines = str::from_utf8(&output.stdout).unwrap()
		.split('\n').collect::<Vec<_>>();

	lines[0..lines.len()-1].iter()
		.fold((0, 0), |acc, line| {
			if line == &"0" {
				return (acc.0 + 1, acc.1);
			}
			if line == &"1" {
				return (acc.0, acc.1 + 1);
			}
			panic!("Invalid qcheck output")
		})
}

pub fn rfdist(infile: &str) -> u64 {
	// Create temporary folder
	let tmp_folder = create_tmp_folder();

	// Copy input file to intree
	let mut intree_file = tmp_folder.clone();
	intree_file.push("intree");
	fs::copy(infile, intree_file).unwrap();

	// Execute phylip treedist
	let mut child = Command::new("treedist")
		// .arg("treedist")
		.current_dir(&tmp_folder)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn().unwrap();

	// Write commands to stdin
	let child_stdin = child.stdin.as_mut().unwrap();
	child_stdin.write_all(b"D\nY\n").unwrap();
	child.wait_with_output().unwrap();

	// Extract result
	let mut outfile = tmp_folder.clone();
	outfile.push("outfile");

	let lines = BufReader::new(File::open(outfile).expect("Unable to open file")).lines();
	let last_line = lines.last().unwrap().unwrap();
	let result = last_line.split(' ')
		.collect::<Vec<&str>>()
		.last()
		.unwrap()
		.parse::<u64>()
		.unwrap();

	// Delete temporary folder
	fs::remove_dir_all(tmp_folder).unwrap();

	result
}