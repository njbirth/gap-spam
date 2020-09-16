use crate::{Sequence, SpacedWord, QTree};
use std::fs::File;
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

	/// Count the number of pairs in the different categories (all different, weak split, ...)
	/// Order of return values: 3-1, 4, 2-2, 2-1-1, 1-1-1-1
	pub fn count(pairs: &Vec<(PBlock, PBlock)>) -> [u64; 5] {
		let mut result_sum = [0; 5];

		for pair in pairs {
			let dists = PBlock::get_distances(&pair.0, &pair.1);
			let mut different_dists = HashSet::new();
			for d in dists.values() {
				different_dists.insert(d);
			}

			result_sum[different_dists.len()] += 1;
			if different_dists.len() == 2 && QTree::new(&pair.0, &pair.1).is_none() {
				result_sum[0] += 1;
				result_sum[2] -= 1;
			}
		}

		result_sum
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

	/// Returns if block1 and block2 would form a QTree of kind A/A/B/B
	pub fn perfect_pair(block1: &PBlock, block2: &PBlock) -> bool {
		if block1.len() != 4 || block2.len() != 4 || block1.len() != block2.len() {
			return false;
		}

		let mut d = Vec::new();
		for i in 0..block1.0.len() {
			d.push(block1[i].position as i64 - block2[i].position as i64);
		}

		!(d[0] == d[1] && d[1] == d[2] && d[2] == d[3]) && (
		(d[0] == d[1] && d[2] == d[3]) ||
		(d[0] == d[2] && d[1] == d[3]) ||
		(d[0] == d[3] && d[2] == d[1]) )
	}

	pub fn get_distances(a: &PBlock, b: &PBlock) -> HashMap<String, i64> {
		let mut result = HashMap::new();
		for i in 0..a.0.len() {
			result.insert(a[i].seq_name.clone(), b[i].position as i64 - a[i].position as i64);
		}
		result
	}

	pub fn find_matching_block(block: &PBlock, sequences: &HashMap<String, Sequence>, pattern: &str, range: i64, perfect: bool) -> Option<PBlock> {
		let mut sequences_filtered = Vec::new();
		for species in block.get_sequence_names() {
			sequences_filtered.push(&sequences[species]);
		}

		let mut spaced_words = Vec::new();
		for i in 0..block.len() {
			if block[i].rev_comp {
				spaced_words.push(sequences_filtered[i].spaced_words(pattern, block[i].position as i64 * -1, block[i].position as i64 * -1 + range, true));
			}
			else {
				spaced_words.push(sequences_filtered[i].spaced_words(pattern, block[i].position as i64, block[i].position as i64 + range, false));
			}
			if i > 0  {
				spaced_words[i].sort();
			}
			else {
				if spaced_words[0].len() == 0 {
					return None;
				}
			}
		}

		let mut index_min = 0;
		let mut index_max = spaced_words[0].len() - 1;
		let mut index_mid = (index_max + index_min ) / 2;
		
		loop {
			if index_mid >= index_max || index_max - index_min <= 1 {
				return None;
			}
			let sw = spaced_words[0][index_mid].clone();

			let mut word_vec = vec![sw.clone()];
			for i in 1..spaced_words.len() {
				let search = spaced_words[i].binary_search(&sw);
				
				// If we don't find the spaced word in a sequence, we can't build a p-block out of this word
				if search.is_err() {
					index_mid += 1;
					break;
				}

				// If we find a spaced word more than one time, we throw it away, because we can't
				// decide, which of them is a match
				if search.unwrap() < spaced_words[i].len() - 1 && spaced_words[i][search.unwrap()] == spaced_words[i][search.unwrap() + 1]
				|| search.unwrap() > 0 && spaced_words[i][search.unwrap()] == spaced_words[i][search.unwrap() - 1] {
					index_mid += 1;
					break;
				}


				word_vec.push(spaced_words[i][search.unwrap()].clone());

				if i == spaced_words.len() - 1 {
					let new_block = PBlock::from_spaced_words(word_vec.clone());
					let tree = QTree::new(&block, &new_block);
					
					if tree.is_some() && (!perfect || PBlock::perfect_pair(&block, &new_block)) {
						return Some(new_block);
					}

					let dists = PBlock::get_distances(&block, &new_block);
					let mut tmp = HashSet::new();
					for d in dists.values() {
						tmp.insert(d);
					}

					if tmp.len() == 4 {
						index_max = index_mid;
						index_mid = (index_max + index_min ) / 2;
					}
					else {
						index_min = index_mid;
						index_mid = (index_max + index_min ) / 2;
					}
				}
			}
		}
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
			if true || dists_set.len() != 1 && dists_set.len() != blocks[i].len() {
				result.push((blocks[i].clone(), blocks[i+1].clone()));
			}
		}

		result
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