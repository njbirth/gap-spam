use std::env;
use std::fs;
use std::path::PathBuf;
use rand;

pub fn create_tmp_folder() -> PathBuf {
	let mut tmp_dir = env::temp_dir();
	let r: u64 = rand::random();
	tmp_dir.push(format!("gaps_rs_{}", r));
	fs::create_dir(&tmp_dir).unwrap();
	tmp_dir
}