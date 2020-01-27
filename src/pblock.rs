use crate::{Sequence, SpacedWord};
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::ops::Index;
use needletail;
use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct PBlock(pub Vec<SpacedWord>);

impl PBlock {
	pub fn from_spaced_words(mut input: Vec<SpacedWord>) -> PBlock {
		input.sort_unstable_by(|a, b| {
			a.seq_name.cmp(&b.seq_name)
		});

		PBlock(input)
	}

	pub fn read_from_file(filename: &str, blocksize: u32) -> Vec<PBlock> {
		if blocksize == 0 {
			return PBlock::read_var_from_file(filename);
		}

		let mut words: Vec<SpacedWord> = Vec::new();

		needletail::parse_sequence_path(
			filename,
	        |_| {},
	        |seq| {
	            let header = String::from_utf8(seq.id.into_owned()).unwrap();
	            let parts: SmallVec<[&str; 5]> = header.split(" ").collect();

				let mut rev_comp = false;
				if parts[4] == "1)" {
					rev_comp = true;
				}

				words.push(SpacedWord::new(parts[0], parts[2].parse().unwrap(), &None, &None, rev_comp));
	        },
	    )
	    .expect("parsing failed");

		let mut result: Vec<PBlock> = Vec::new();

		let mut it = words.into_iter();
		loop {
			let next = it.next();
			if next.is_none() {
				break;
			}

			let mut tmp = vec![next.unwrap()];
			for _i in 0..blocksize-1 {
				tmp.push(it.next().unwrap());
			}

			result.push(PBlock::from_spaced_words(tmp));
		}

		result
	}

	fn read_var_from_file(filename: &str) -> Vec<PBlock> {
		let f = File::open(filename).expect("Unable to open file");
		let r = BufReader::new(f);
		let mut lines = r.lines();

		let mut result: Vec<PBlock> = Vec::new();

		loop {
			let mut tmp_vec = Vec::new();

			loop {
				let next = lines.next();
				if next.is_none() {
					return result;
				}

				let next = next.unwrap().unwrap();
				if &next == "" {
					lines.next();
					break;
				}
				if &next[0..1] != ">" {
					continue;
				}

				let parts: SmallVec<[&str; 5]> = next[1..].split(" ").collect();

				let mut rev_comp = false;
				if parts[4] == "1)" {
					rev_comp = true;
				}

				tmp_vec.push(SpacedWord::new(parts[0], parts[2].parse().unwrap(), &None, &None, rev_comp));
			}

			result.push(PBlock::from_spaced_words(tmp_vec));
		}
	}

	pub fn get_sequence_names(&self) -> Vec<&String> {
		let mut result = Vec::new();

		for word in &self.0 {
			result.push(&word.seq_name);
		}
		result.sort_unstable();

		result
	}

	pub fn get_distances(a: &PBlock, b: &PBlock) -> HashMap<String, i64> {
		let mut result = HashMap::new();
		for i in 0..a.0.len() {
			result.insert(a[i].seq_name.clone(), b[i].position as i64 - a[i].position as i64);
		}
		result
	}

	pub fn find_matching_pblocks(block: &PBlock, sequences: &HashMap<String, Sequence>, pattern: &str, range: i64) -> Vec<PBlock> {
		let mut sequences_filtered = Vec::new();
		for species in block.get_sequence_names() {
			sequences_filtered.push(&sequences[species]);
		}

		let mut result = Vec::new();

		let mut spaced_words = Vec::new();
		for i in 0..block.len() {
			if block[i].rev_comp {
				spaced_words.push(sequences_filtered[i].spaced_words(pattern, block[i].position as i64 - range, block[i].position as i64 + range, true));
			}
			else {
				spaced_words.push(sequences_filtered[i].spaced_words(pattern, block[i].position as i64 - range, block[i].position as i64 + range, false));
			}
			spaced_words[i].sort();
		}

		for sw in &spaced_words[0] {
			let mut word_vec = Vec::new();
			for i in 0..spaced_words.len() {
				let search = spaced_words[i].binary_search(sw);
				
				// If we don't find the spaced word in a sequence, we can't build a p-block out of this word
				if search.is_err() {
					break;
				}

				// If we find a spaced word more than one time, we throw it away, because we can't
				// decide, which of them is a match
				if search.unwrap() < spaced_words[i].len() - 1 && spaced_words[i][search.unwrap()] == spaced_words[i][search.unwrap() + 1]
				|| search.unwrap() > 0 && spaced_words[i][search.unwrap()] == spaced_words[i][search.unwrap() - 1] {
					break;
				}


				word_vec.push(spaced_words[i][search.unwrap()].clone());

				if i == spaced_words.len() - 1 {
					result.push(PBlock::from_spaced_words(word_vec));
					break;
				}
			}
		}

		result
	}

	pub fn blocks_to_string(blocks: &Vec<PBlock>) -> String {
		let mut result = String::new();
		for block in blocks {
			result.push_str(&format!("{}:{}; {}:{}; {}:{}; {}:{}\n", block[0].seq_name, block[0].position, block[1].seq_name, block[1].position, block[2].seq_name, block[2].position, block[3].seq_name, block[3].position)[..]);
		}
		result
	}

	pub fn split_by_species(mut blocks: Vec<PBlock>) -> Vec<Vec<PBlock>> {
		blocks.sort_unstable_by(|a, b| {
			let n = if a.len() < b.len() { a.len() } else { b.len() };

			for i in 0..n {
				if a[i].seq_name != b[i].seq_name {
					return a[i].seq_name.cmp(&b[i].seq_name);
				}
			}

			a.len().cmp(&b.len())
		});

		let mut r: Vec<Vec<PBlock>> = vec![vec![blocks[0].clone()]];

		for i in 1..blocks.len() {
			if blocks[i].get_sequence_names() == blocks[i-1].get_sequence_names() {
				let l = r.len()-1;
				r[l].push(blocks[i].clone());
			}
			else {
				r.push(vec![blocks[i].clone()]);
			}
		}

		r
	}

	pub fn pairs_from_vector(blocks: &mut Vec<PBlock>) -> Vec<(PBlock, PBlock)> {
		blocks.sort_unstable_by(|a, b| {
			a[0].position.cmp(&b[0].position)
		});

		let mut result = Vec::new();

		if blocks.len() < 2 {
			return result;
		}

		for i in 0..blocks.len()-1 {
			let mut dists_set = HashSet::new();
			for (_, dist) in PBlock::get_distances(&blocks[i], &blocks[i+1]) {
				dists_set.insert(dist);
			}
			if dists_set.len() != 1 && dists_set.len() != blocks[i].len() {
				result.push((blocks[i].clone(), blocks[i+1].clone()));
			}
		}

		result
	}

	pub fn save_pairs_to_pars_file(pairs: &Vec<(PBlock, PBlock)>, filename: &str) {
		// Collect species
		let mut species = HashSet::new();
		for pair in pairs {
			for name in pair.0.get_sequence_names() {
				species.insert(String::from(name));
			}
		}
		let species: Vec<String> = species.iter().cloned().collect();

		// Build lines
		let mut output: HashMap<String, Vec<&str>> = HashMap::new();
		let padding = ["", "", " ", "  ", "   ", "    ", "     ", "      ", "       "];
		for i in 0..species.len() {
			if species[i].len() >= 9 {
				output.insert(species[i].clone(), vec![&species[i][0..9]]);
			}
			else {
				output.insert(species[i].clone(), vec![&species[i]]);
				output.get_mut(&species[i]).unwrap().push(padding[9-species[i].len()]);
			}
		}

		for pair in pairs {
			//let subst = ["A", "B", "C", "D", "E", "F", "G", "H", "?"]; // alphabetisch
			//let subst = ["A", "C", "G", "T", "R", "Y", "M", "K", "?"]; // DNA (mit Makros)
			let subst = ["A", "R", "N", "D", "C", "Q", "E", "G", "?"]; // protein (ohne Makros)
			let mut subst_index = 0;
			let distances: HashMap<String, i64> = PBlock::get_distances(&pair.0, &pair.1);
			let mut dists_replaced: HashMap<String, &str> = HashMap::new();
			for key in pair.0.get_sequence_names() {
				dists_replaced.insert(String::from(key), "0");
			}

			for (seq, dist) in distances.iter() {
				if dists_replaced[seq] != "0" {
					continue;
				}
				for seq2 in pair.0.get_sequence_names() {
					if distances[seq2] == *dist {
						dists_replaced.insert(seq2.to_string(), subst[subst_index]);
					}
				}
				if subst_index < subst.len()-1 {
					subst_index += 1;
				}
			}

			// Ausgabezeilen ergänzen
			for s in &species {
				let c = dists_replaced.get(s);
				if c.is_none() {
					output.get_mut(s).unwrap().push("?");
				}
				else {
					output.get_mut(s).unwrap().push(c.unwrap());
				}
			}
		}

		// Write to file
		let mut f = File::create(filename).expect("Unable to create file");
		f.write_all(format!("{} {}\n", species.len(), pairs.len()).as_bytes()).expect("Unable to write data");
		for (_, value) in &output {
			f.write_all(value.join(" ").as_bytes()).expect("Unable to write data");
			f.write_all(b"\n").expect("Unable to write data");
		}
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}
}

impl Index<usize> for PBlock {
	type Output = SpacedWord;

	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}