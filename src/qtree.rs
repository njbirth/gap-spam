/*	Copyright (C) 2020 - Niklas Birth

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use std::fmt;
use std::fs::File;
use std::io::Write;
use crate::{PBlock, SpacedWord};

#[derive(Debug)]
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
				pair1: pair1,
				pair2: pair2,
			}
		)
	}

	pub fn from_vector(blocks: &mut Vec<PBlock>) -> Vec<QTree> {
		blocks.sort_unstable_by(|a, b| a[0].position.cmp(&b[0].position));

		let mut result = Vec::new();

		if blocks.len() < 2 {
			return result;
		}

		for i in 0..blocks.len()-1 {
			let tree = QTree::new(&blocks[i], &blocks[i+1]);
			if tree.is_some() {
				result.push(tree.unwrap());
			}
		}

		result
	}

	pub fn from_nwk_str(nwk: &str) -> QTree {
		let mut nwk = nwk.replace(";", "");
		nwk = nwk.replace(" ", "");
		nwk = nwk.replace("),(", ",");
		nwk = nwk.replace("((", "");
		nwk = nwk.replace("))", "");
		let split: Vec<&str> = nwk.split(",").collect();

		QTree {
			blocks: (
				PBlock(vec![SpacedWord::new(split[0], 0, &None, &None, false), SpacedWord::new(split[1], 0, &None, &None, false), SpacedWord::new(split[2], 0, &None, &None, false), SpacedWord::new(split[3], 0, &None, &None, false)]),
				PBlock(vec![SpacedWord::new(split[0], 0, &None, &None, false), SpacedWord::new(split[1], 0, &None, &None, false), SpacedWord::new(split[2], 0, &None, &None, false), SpacedWord::new(split[3], 0, &None, &None, false)])
			),
			pair1: (0, 1),
			pair2: (2, 3)
		}
	}

	pub fn save_to_file(qtrees: &Vec<QTree>, filename: &str) {
		let mut f = File::create(filename).expect("Unable to create file");
		for i in 0..qtrees.len() {
			f.write_all(format!("{}\n", qtrees[i]).as_bytes()).expect("Unable to write data");
		}
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

	pub fn from_pairs(pairs: &Vec<(PBlock, PBlock)>) -> Vec<QTree> {
		let mut result = Vec::new();

		for pair in pairs {
			let tree = QTree::new(&pair.0, &pair.1);
			if tree.is_some() {
				result.push(tree.unwrap());
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