use crate::{Sequence, SpacedWord, QTree};
use std::collections::{HashMap, HashSet};
use std::ops::Index;
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

	pub fn read_from_file(filename: &str) -> Vec<PBlock> {
		let mut words: Vec<SpacedWord> = Vec::new();

		needletail::parse_sequence_path(
			filename,
	        |_| {},
	        |seq| {
	            let header = String::from_utf8(seq.id.into_owned()).unwrap();
	            let parts: SmallVec<[&str; 5]> = header.split(' ').collect();

				let mut rev_comp = false;
				if parts[4] == "1)" {
					rev_comp = true;
				}

				words.push(SpacedWord::new(parts[0], parts[2].parse().unwrap(), &None, &None, rev_comp));
	        },
	    )
	    .expect("parsing of block file failed");

		if words.len() % 4 != 0 {
			panic!("Number of spaced words in input file is not divisible by 4.");
		}

		words.chunks(4)
			.map(|chunk| PBlock::from_spaced_words(chunk.to_owned()))
			.collect()
	}

	pub fn get_sequence_names(&self) -> Vec<&String> {
		let mut result = Vec::new();

		for word in &self.0 {
			result.push(&word.seq_name);
		}
		result.sort_unstable();

		result
	}

	/// Returns true if block1 and block2 would form a pair that strongly supports a topology
	pub fn strong_pair(block1: &PBlock, block2: &PBlock) -> bool {
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
				spaced_words.push(sequences_filtered[i].spaced_words(pattern, -(block[i].position as i64), -(block[i].position as i64) + range, true));
			}
			else {
				spaced_words.push(sequences_filtered[i].spaced_words(pattern, block[i].position as i64, block[i].position as i64 + range, false));
			}
			if i > 0  {
				spaced_words[i].sort();
			}
			else if spaced_words[0].is_empty() {
				return None;
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
					
					if tree.is_some() && (!perfect || PBlock::strong_pair(&block, &new_block)) {
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

	pub fn blocks_to_string(blocks: &[PBlock]) -> String {
		let mut result = String::new();
		for block in blocks {
			result.push_str(&format!("{}:{}; {}:{}; {}:{}; {}:{}\n", block[0].seq_name, block[0].position, block[1].seq_name, block[1].position, block[2].seq_name, block[2].position, block[3].seq_name, block[3].position)[..]);
		}
		result
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

impl Index<usize> for PBlock {
	type Output = SpacedWord;

	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}