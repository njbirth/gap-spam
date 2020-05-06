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

use stopwatch::Stopwatch;
use std::io::Write;
use std::io::stdout;

pub fn run(opt: crate::opt::Gaps) -> Result<(), String> {
	let time_all = Stopwatch::start_new();

	if !opt.hide_progress { print!("- Read FASTA file"); }
	stdout().flush().unwrap();
	let mut sw = Stopwatch::start_new();
	let sequences = Sequence::read_fasta_file(&opt.fastafile);
	if !opt.hide_progress { println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }
	let rep_sequences = sequences.len();

	if !opt.hide_progress { print!("- Read PBlock file"); }
	stdout().flush().unwrap();
	sw.restart();
	let blocksize = if opt.pars { opt.blocksize } else { 4 };
	let input: Vec<PBlock> = PBlock::read_from_file(&opt.infile, blocksize);
	if !opt.hide_progress { println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }
	let rep_pblocks = input.len();

	if !opt.hide_progress { print!("- Sort PBlocks"); }
	stdout().flush().unwrap();
	sw.restart();
	let input_sorted: Vec<Vec<PBlock>> = PBlock::split_by_species(input);
	if !opt.hide_progress { println!("\t\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }

	if !opt.hide_progress { print!("- Collect PBlock pairs"); }
	stdout().flush().unwrap();
	sw.restart();
	let mut pairs: Vec<(PBlock, PBlock)> = Vec::new();
	let mut additional_blocks = Vec::new();
	for mut v in input_sorted {
		if opt.additional == 1 {
			additional_blocks.push(v[0].clone());
		}
		else if opt.additional == 2 {
			additional_blocks.append(&mut v.clone());
		}
		else {
			let mut new_pairs = PBlock::pairs_from_vector(&mut v);
			pairs.append(&mut new_pairs);
		}
	}
	if !opt.hide_progress { println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }
	let rep_input_pairs = pairs.len();

	let mut rep_add_pairs = 0;
	if opt.additional > 0 {
		if !opt.hide_progress { print!("- Collect additional pairs"); }
		stdout().flush().unwrap();
		sw.restart();
		let mut additional_pairs: Vec<(PBlock, PBlock)> = Vec::new();

		for i in 0..additional_blocks.len() {
			if !opt.hide_progress { print!("\r- Collect additional pairs\t({}/{})", i, additional_blocks.len()); }
			stdout().flush().unwrap();
			let block = additional_blocks[i].clone();
			let block2 = PBlock::find_matching_block(&block, &sequences, &opt.pattern, opt.range, opt.perfect);
			if block2.is_some() {
				additional_pairs.push((block, block2.unwrap()));
			}
		}

		rep_add_pairs = additional_pairs.len();
		pairs.append(&mut additional_pairs);
		if !opt.hide_progress { println!("\r- Collect additional pairs\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }
	}

	if !opt.hide_progress { print!("- Collect report data"); }
	stdout().flush().unwrap();
	sw.restart();
	let rep_count = PBlock::count(&pairs);
	if !opt.hide_progress { println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }

	if !opt.hide_progress { print!("- Filter pairs"); }
	stdout().flush().unwrap();
	sw.restart();
	pairs = if opt.perfect {
		pairs.into_iter().filter(|a| PBlock::perfect_pair(&a.0, &a.1)).collect()
	}
	else {
		pairs.into_iter().filter(|a| QTree::new(&a.0, &a.1).is_some() && (!PBlock::perfect_pair(&a.0, &a.1) || !opt.imperfect)).collect()
	};
	if !opt.hide_progress { println!("\t\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }

	if !opt.hide_progress { print!("- Build QTrees"); }
	stdout().flush().unwrap();
	sw.restart();
	let result = QTree::from_pairs(&pairs);
	let rep_trees = result.len();
	// result_unique is used for calculating the coverage in the report
	let mut result_unique = result.clone();
	result_unique.sort_unstable();
	result_unique.dedup();
	let rep_trees_unique = result_unique.len();
	if !opt.hide_progress { println!("\t\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }

	if !opt.hide_progress { print!("- Save result to file"); }
	stdout().flush().unwrap();
	sw.restart();
	if opt.pars {
		PBlock::save_pairs_to_pars_file(&pairs, &opt.outfile);
	}
	else {
		QTree::save_to_file(&result, &opt.outfile);
	}
	if !opt.hide_progress { println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }

	if !opt.hide_progress { println!("\t\t\t\t(Total time: {}s)\n", time_all.elapsed_ms() as f32/1000.0); }

	println!("================== REPORT ==================");
	println!("input sequences: {}", rep_sequences);
	println!("input p-blocks: {}", rep_pblocks);
	println!("pairs from input blocks: {}", rep_input_pairs);
	println!("pairs with new blocks: {}", rep_add_pairs);
	println!("categories:");
	println!("    2-2: \t{} \t({:.2}%)", rep_count[2], rep_count[2] as f64 / (rep_input_pairs as f64 + rep_add_pairs as f64) * 100.0);
	println!("    2-1-1: \t{} \t({:.2}%)", rep_count[3], rep_count[3] as f64 / (rep_input_pairs as f64 + rep_add_pairs as f64) * 100.0);
	println!("    1-1-1-1: \t{} \t({:.2}%)", rep_count[4], rep_count[4] as f64 / (rep_input_pairs as f64 + rep_add_pairs as f64) * 100.0);
	println!("    3-1: \t{} \t({:.2}%)", rep_count[0], rep_count[0] as f64 / (rep_input_pairs as f64 + rep_add_pairs as f64) * 100.0);
	println!("    4: \t\t{} \t({:.2}%)", rep_count[1], rep_count[1] as f64 / (rep_input_pairs as f64 + rep_add_pairs as f64) * 100.0);
	println!("quartet trees: {}", rep_trees);
	let max_coverage = (rep_sequences*(rep_sequences-1)*(rep_sequences-2)*(rep_sequences-3)) as f64 / 24.0;
	println!("coverage: {:.2}%", rep_trees_unique as f64 / max_coverage * 100.0);
	println!("============================================");

    Ok(())
}

// ========================================================

pub mod build_tree;

pub mod opt;

pub mod tools;


mod qtree;
pub use self::qtree::QTree;

mod sequence;
pub use self::sequence::Sequence;

mod spaced_word;
pub use self::spaced_word::SpacedWord;

mod pblock;
pub use self::pblock::PBlock;