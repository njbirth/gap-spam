use std::cmp::Ordering;

#[derive(Debug, Eq, Clone)]
pub struct SpacedWord {
	pub seq_name: String,
	pub position: i64,
	pub match_positions: Option<u64>,
	pub rev_comp: bool
}

impl SpacedWord {
	// Returns None, if the word contains a non-A/C/G/T symbol on a match position
	pub fn new(seq_name: &str, mut position: i64, word: &Option<&str>, pattern: &Option<&str>, rev_comp: bool) -> Option<SpacedWord> {
		if rev_comp {
			position = -position;
		}

		let mut match_positions = None;
		if let (Some(word), Some(pattern)) = (word, pattern) {
			match_positions = Some(SpacedWord::match_positions(word, pattern)?)
		}

		Some(SpacedWord {
			seq_name: String::from(seq_name),
			position,
			match_positions,
			rev_comp
		})
	}

	// Returns None, if the word contains a non-A/C/G/T symbol on a match position
	fn match_positions(word: &str, pattern: &str) -> Option<u64> {
		word.chars().zip(pattern.chars())
			.filter(|(_, p)| p == &'1')
			.fold(Some(0), |acc, (base, _)| {
				match base {
					'A' | 'a' => Some(acc? + 0 << 2),
					'C' | 'c' => Some(acc? + 1 << 2),
					'G' | 'g' => Some(acc? + 2 << 2),
					'T' | 't' => Some(acc? + 3 << 2),
					_ => None
				}
			})
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