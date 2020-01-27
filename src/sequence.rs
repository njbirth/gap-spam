use crate::SpacedWord;
use std::collections::HashMap;
use needletail;
use needletail::sequence::Sequence as NTSequence;

#[derive(Debug)]
pub struct Sequence {
	pub name: String,
	pub sequence: String,
	pub seq_rev: String,
	pub is_rev_comp: bool
}

impl Sequence {
	/// TODO: rev_comp should be calculated by needletail; much more efficient
	/// not too important, because this method is currently not in use
	pub fn new(name: String, sequence: String, is_rev_comp: bool) -> Sequence {
		let mut seq_rev = sequence.chars().rev().collect::<String>();
		
		seq_rev = seq_rev.replace("A", "c");
		seq_rev = seq_rev.replace("C", "a");
		seq_rev = seq_rev.replace("G", "t");
		seq_rev = seq_rev.replace("T", "g");
		
		seq_rev = seq_rev.replace("A", "A");
		seq_rev = seq_rev.replace("C", "C");
		seq_rev = seq_rev.replace("G", "G");
		seq_rev = seq_rev.replace("T", "T");

		Sequence {
			name: name,
			sequence: sequence,
			seq_rev: seq_rev,
			is_rev_comp: is_rev_comp
		}
	}

	pub fn len(&self) -> usize {
		self.sequence.len()
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

	pub fn read_fasta_file(filename: &str) -> HashMap<String, Sequence> {
		let mut result = HashMap::new();

		needletail::parse_sequence_path(
			filename,
	        |_| {},
	        |seq| {
	        	let seq_rev = String::from_utf8(seq.reverse_complement()).unwrap();
	            let header = String::from_utf8(seq.id.into_owned()).unwrap();
	            let sequence = String::from_utf8(seq.seq.into_owned()).unwrap();

	            result.insert(header.clone(), Sequence { name: header, sequence: sequence, seq_rev: seq_rev, is_rev_comp: false });
	        },
	    )
	    .expect("parsing failed");

	    result
	}
}