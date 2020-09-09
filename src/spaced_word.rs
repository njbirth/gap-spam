use std::cmp::Ordering;

#[derive(Debug, Eq, Clone)]
pub struct SpacedWord {
	pub seq_name: String,
	pub position: i64,
	pub match_positions: Option<u64>,
	pub rev_comp: bool
}

impl SpacedWord {
	pub fn new(seq_name: &str, mut position: i64, word: &Option<&str>, pattern: &Option<&str>, rev_comp: bool) -> SpacedWord {
		if rev_comp {
			position = position * (-1);
		}

		SpacedWord {
			seq_name: String::from(seq_name),
			position,
			match_positions: SpacedWord::match_positions(word, pattern),
			rev_comp
		}
	}

	fn match_positions(word: &Option<&str>, pattern: &Option<&str>) -> Option<u64> {
		if word.is_none() || pattern.is_none() {
			return None;
		}

		let word_chars: Vec<char> = word.unwrap().chars().collect();
		let pattern_chars: Vec<char> = pattern.unwrap().chars().collect();

		let mut match_positions: u64 = 0;

		for i in 0..word_chars.len() {
			if pattern_chars[i] == '0' {
				continue;
			}

			if word_chars[i] == 'A' {
				match_positions += 0;
				match_positions <<= 2;
			}
			if word_chars[i] == 'C' {
				match_positions += 1;
				match_positions <<= 2;
			}
			if word_chars[i] == 'G' {
				match_positions += 2;
				match_positions <<= 2;
			}
			if word_chars[i] == 'T' {
				match_positions += 3;
				match_positions <<= 2;
			}
		}

		Some(match_positions)
	}

	pub fn gap_size(&self, other: &SpacedWord) -> i64 {
		(self.position as i64 - other.position as i64).abs()
	}
}

impl PartialEq for SpacedWord {
	fn eq(&self, other: &Self) -> bool {
		self.match_positions == other.match_positions
	}
}

impl Ord for SpacedWord {
	fn cmp(&self, other: &Self) -> Ordering {
		self.match_positions.cmp(&other.match_positions)
	}
}

impl PartialOrd for SpacedWord {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}