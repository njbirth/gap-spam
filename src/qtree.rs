use std::fmt;
use crate::PBlock;

#[derive(Debug, Clone)]
pub struct QTree {
	pub blocks: (PBlock, PBlock),
	pub pair1: (usize, usize),
	pub pair2: (usize, usize),
}

impl QTree {
	pub fn new(p1: &PBlock, p2: &PBlock) -> Option<QTree> {
		// panic, wenn die PBlocks nicht beide Größe 4 haben
		if p1.len() != 4 || p2.len() != 4 {
			panic!("PBlock has size != 4.");
		}
		
		// panic, wenn die PBlocks verschiedene Sequenzen enthalten
		if p1.get_sequence_names() != p2.get_sequence_names() {
			panic!("Species name not found in P Block");
		}

		// Abstände
		let g = [p1[0].gap_size(&p2[0]), p1[1].gap_size(&p2[1]), p1[2].gap_size(&p2[2]), p1[3].gap_size(&p2[3])];

		// Baum konstruieren, wenn möglich
		let pair1;
		let pair2;

		// Kein Baum möglich, wenn drei oder vier Distanzen gleich sind
		if 	g[0] == g[1] && g[1] == g[2] ||
			g[0] == g[1] && g[1] == g[3] ||
			g[0] == g[2] && g[2] == g[3] ||
			g[1] == g[2] && g[2] == g[3] {
				return None;	
			}

		// Finde passende Distanzen, oder None, wenn alle vier verschieden sind
		if 		g[0] == g[1] 	{ pair1 = (0, 1); pair2 = (2, 3); }
		else if g[0] == g[2] 	{ pair1 = (0, 2); pair2 = (1, 3); }
		else if g[0] == g[3] 	{ pair1 = (0, 3); pair2 = (1, 2); }
		else if g[1] == g[2] 	{ pair1 = (1, 2); pair2 = (0, 3); }
		else if g[1] == g[3] 	{ pair1 = (1, 3); pair2 = (0, 2); }
		else if g[2] == g[3] 	{ pair1 = (2, 3); pair2 = (0, 1); }
		else 					{ return None; }

		Some(
			QTree {
				blocks: (p1.clone(), p2.clone()),
				pair1,
				pair2,
			}
		)
	}

	pub fn seq_names(&self) -> Vec<&str> {
		vec![&self.blocks.0[0].seq_name, &self.blocks.0[1].seq_name, &self.blocks.0[2].seq_name, &self.blocks.0[3].seq_name]
	}

	pub fn gap_sizes(&self) -> Vec<i64> {
		vec![
			self.blocks.1[0].position as i64 - self.blocks.0[0].position as i64,
			self.blocks.1[1].position as i64 - self.blocks.0[1].position as i64,
			self.blocks.1[2].position as i64 - self.blocks.0[2].position as i64,
			self.blocks.1[3].position as i64 - self.blocks.0[3].position as i64
		]
	}

	pub fn from_pairs(pairs: &[(PBlock, PBlock)]) -> Vec<QTree> {
		let mut result = Vec::new();

		for pair in pairs {
			if let Some(tree) = QTree::new(&pair.0, &pair.1) {
				result.push(tree);
			}
		}

		result
	}
}

impl fmt::Display for QTree {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(({},{}),({},{}));", self.blocks.0[self.pair1.0].seq_name, self.blocks.0[self.pair1.1].seq_name, self.blocks.0[self.pair2.0].seq_name, self.blocks.0[self.pair2.1].seq_name)
    }
}