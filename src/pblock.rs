use crate::{Sequence, SpacedWord, QTree};
use std::collections::HashMap;
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

				words.push(SpacedWord::new(parts[0], parts[2].parse().unwrap(), &None, &None, parts[4] == "1)").unwrap());
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
		let mut result = self.0.iter()
			.map(|word| &word.seq_name)
			.collect::<Vec<_>>();
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

	pub fn find_matching_block(block: &PBlock, sequences: &HashMap<String, Sequence>, pattern: &str, range: i64, strong_only: bool) -> Option<PBlock> {
		let sequences = block.get_sequence_names().iter()
			.map(|name| &sequences[*name])
			.collect::<Vec<_>>();

		let mut spaced_words = Vec::new();
		for i in 0..block.len() {
			if block[i].rev_comp {
				spaced_words.push(sequences[i].spaced_words(pattern, -(block[i].position as i64), -(block[i].position as i64) + range, true));
			}
			else {
				spaced_words.push(sequences[i].spaced_words(pattern, block[i].position as i64, block[i].position as i64 + range, false));
			}
			if i > 0  {
				spaced_words[i].sort();
			}
			else if spaced_words[0].is_empty() {
				return None;
			}
		}

		for sw in &spaced_words[0] {
			let mut word_vec = vec![sw.clone()];

			for words in &spaced_words[1..] {
				if let Ok(search) = words.binary_search(&sw) {
					// If we find a spaced word more than one time, we throw it away, because we can't
					// decide, which of them is a match
					if search < words.len() - 1 && words[search] == words[search + 1]
						|| search > 0 && words[search] == words[search - 1] {
						break;
					}

					word_vec.push(words[search].clone());
				}
				else {
					break;
				}
			}

			let new_block = PBlock::from_spaced_words(word_vec);
			if QTree::new(&block, &new_block).is_some() && (!strong_only || PBlock::strong_pair(&block, &new_block)) {
				return Some(new_block);
			}
		}

		None
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