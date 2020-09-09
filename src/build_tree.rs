use crate::QTree;
use std::collections::HashMap;
use std::process::Command;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::fs;

// === max-cut =====================================================================

pub fn max_cut_from_file(filename: &str) -> String {
	let f = File::open(filename).expect("Unable to open file");
	let r = BufReader::new(f);

	let mut trees = Vec::new();

	for line in r.lines() {
		trees.push(line.unwrap());
	}

	max_cut(&trees)
}

pub fn max_cut(qtrees: &Vec<String>) -> String {
	// Replace species names by ids
	let id_dict = build_id_dict(&qtrees);
	let qtrees = replace_names_to_ids(qtrees, &id_dict);

	// Write reformatted QTrees to temp file
	let mut f = File::create("max_cut_input.tmp").expect("Unable to create file");
	for tree in qtrees {
		f.write_all(format!("{}\n", to_max_cut_string(&tree)).as_bytes()).expect("Unable to write data");
	}

	// Run max-cut
	Command::new("max-cut-tree")
				.arg("qrtt=max_cut_input.tmp")
				.arg("weights=off")
				.arg("otre=max_cut_output.tmp")
				.output()
				.expect("failed to execute max-cut");

	// Read and reformat result
	let nwk = replace_ids_to_names(&fs::read_to_string("max_cut_output.tmp").unwrap().replace("\n", ""), &id_dict);

	// Remove tmp files
	fs::remove_file("max_cut_input.tmp").unwrap();
	fs::remove_file("max_cut_output.tmp").unwrap();

	nwk
}

fn replace_ids_to_names(nwk: &str, dict: &HashMap<String, u32>) -> String {
	// Invert HashMap
	let mut dict_inv = Vec::new();
	dict_inv.resize(dict.len(), "nothing");

	for (key, value) in dict {
		dict_inv[*value as usize] = key;
	}

	// Replace ids
	let mut nwk_new = String::from(nwk);
	for i in (0..dict_inv.len()).rev() {
		nwk_new = nwk_new.replace(&format!("({}", i), &format!("(\"{}\"", dict_inv[i]));
		nwk_new = nwk_new.replace(&format!("{})", i), &format!("\"{}\")", dict_inv[i]));
		nwk_new = nwk_new.replace(&format!(",{},", i), &format!(",\"{}\",", dict_inv[i]));
	}
	nwk_new = nwk_new.replace("\"", "");

	nwk_new
}

fn replace_names_to_ids(qtrees: &Vec<String>, dict: &HashMap<String, u32>) -> Vec<String> {
	let mut qtrees_new = Vec::new();
	
	for nwk in qtrees {
		let split = split_nwk(nwk);
		qtrees_new.push(format!("(({},{}),({},{});", dict[&split[0]], dict[&split[1]], dict[&split[2]], dict[&split[3]]));
	}

	qtrees_new
}

fn build_id_dict(qtrees: &Vec<String>) -> HashMap<String, u32> {
	let mut dict = HashMap::new();
	let mut cur_id = 0;

	for tree in qtrees {
		for name in split_nwk(tree) {
			if !dict.contains_key(&name) {
				dict.insert(String::from(name), cur_id);
				cur_id += 1;
			}
		}
	}

	dict
}

fn split_nwk(nwk: &str) -> Vec<String> {
	let mut nwk = String::from(nwk).replace(";", "");
	nwk = nwk.replace(" ", "");
	nwk = nwk.replace("),(", ",");
	nwk = nwk.replace("((", "");
	nwk = nwk.replace("))", "");

	let mut result = Vec::new();
	for name in nwk.split(",") {
		result.push(String::from(name));
	}
	result
}

pub fn to_max_cut_string(nwk: &str) -> String {
	let split = split_nwk(nwk);
	format!("{},{}|{},{}", split[0], split[1], split[2], split[3])
}

// === parsimony =============================================================

/*pub fn parsimony_from_file(filename: &str) -> String {
	let mut pars = Command::new("phylip").arg("pars").stdin(Stdio::piped()).spawn().unwrap();
	pars.stdin.as_mut().unwrap().write_all(b"outfile\nF\noutfile.tmp\nY\n");

	String::from("unimplemented")
}*/

pub fn parsimony(_qtrees: &Vec<QTree>) -> String {
	unimplemented!();
}