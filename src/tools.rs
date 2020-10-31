use std::env;
use std::fs;
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

pub fn rfdist(infile: &str) -> u64 {
	// Create temporary folder
	let tmp_folder = create_tmp_folder();

	// Copy input file to intree
	let mut intree_file = tmp_folder.clone();
	intree_file.push("intree");
	fs::copy(infile, intree_file).unwrap();

	// Execute phylip treedist
	let mut child = Command::new("phylip")
		.arg("treedist")
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