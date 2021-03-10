use crate::SpacedWord;
use std::collections::HashMap;
use needletail::sequence::Sequence as NTSequence;

#[derive(Debug)]
pub struct Sequence {
	pub name: String,
	pub sequence: String,
	pub seq_rev: String,
	pub is_rev_comp: bool
}

impl Sequence {
	pub fn read_fasta_file(filename: &str) -> HashMap<String, Sequence> {
		let mut result = HashMap::new();

		needletail::parse_sequence_path(
			filename,
			|_| {},
			|seq| {
				// The to_uppercase makes this function quite slow. There might be a better way.
				let seq_rev = String::from_utf8(seq.reverse_complement()).unwrap().to_uppercase();
				let header = String::from_utf8(seq.id.into_owned()).unwrap();
				let sequence = String::from_utf8(seq.seq.into_owned()).unwrap().to_uppercase();

				result.insert(header.clone(), Sequence { name: header, sequence, seq_rev, is_rev_comp: false });
			},
		)
			.expect("parsing failed");

		result
	}

	pub fn len(&self) -> usize {
		self.sequence.len()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn get_rev_comp(&self) -> Sequence {
		Sequence {
			name: self.name.clone(),
			sequence: self.seq_rev.clone(),
			seq_rev: self.sequence.clone(),
			is_rev_comp: !self.is_rev_comp
		}
	}

	pub fn spaced_words(&self, pattern: &str, mut min_pos: i64, mut max_pos: i64, reverse: bool) -> Vec<SpacedWord> {
		if min_pos > max_pos {
			panic!("min_pos > max_pos...sollte nicht passieren");
		}

		let mut result = Vec::new();
		let self_len = self.len() as i64;
		let pat_len = pattern.len() as i64;

		if min_pos < 0 {
			min_pos = 0;
		}
		if max_pos > self_len {
			max_pos = self_len;
		}

		if pat_len > max_pos - min_pos {
			return result;
		}

		let seq = if reverse { &self.seq_rev } else { &self.sequence };

		for i in min_pos..max_pos - pat_len {
			result.push(
				SpacedWord::new(
					&self.name,
					i as i64,
					&Some(&seq[(i as usize)..(i+pat_len) as usize]),
					&Some(pattern),
					self.is_rev_comp
				)
			);
		}

		result
	}
}