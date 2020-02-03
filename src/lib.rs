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
use std::fs::File;

pub fn run(opt: crate::opt::Gaps) -> Result<(), String> {
	let time_all = Stopwatch::start_new();

	print!("- Read FASTA file");
	stdout().flush().unwrap();
	let mut sw = Stopwatch::start_new();
	let sequences = Sequence::read_fasta_file(&opt.fastafile);
	println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0);
	println!("  => {} sequences read", sequences.len());

	print!("- Read PBlock file");
	stdout().flush().unwrap();
	sw.restart();
	let blocksize = if opt.pars { opt.blocksize } else { 4 };
	let input: Vec<PBlock> = PBlock::read_from_file(&opt.infile, blocksize);
	println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0);
	println!("  => {} PBlocks read", input.len());

	print!("- Sort PBlocks");
	stdout().flush().unwrap();
	sw.restart();
	let input_sorted: Vec<Vec<PBlock>> = PBlock::split_by_species(input);
	println!("\t\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0);
	println!("  => {} different sequence sets", input_sorted.len());

	print!("- Collect PBlock pairs");
	stdout().flush().unwrap();
	sw.restart();
	let mut pairs: Vec<(PBlock, PBlock)> = Vec::new();
	let mut additional_blocks = Vec::new();
	for mut v in input_sorted {
		if v.len() < 2 {
			additional_blocks.push(v[0].clone());
			continue;
		}
		let mut new_pairs = PBlock::pairs_from_vector(&mut v);
		pairs.append(&mut new_pairs);
	}
	println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0);
	println!("  => {} PBlock pairs collected", pairs.len());

	if opt.additional {
		print!("- Collect additional pairs");
		stdout().flush().unwrap();
		sw.restart();
		//let mut blockfile = File::create("outfile_blocks_debug").expect("Unable to create file");
		let mut additional_pairs: Vec<(PBlock, PBlock)> = Vec::new();
		for i in 0..additional_blocks.len() {
			print!("\r- Collect additional pairs\t({}/{})", i, additional_blocks.len());
			stdout().flush().unwrap();
			let mut new_blocks = PBlock::find_matching_pblocks(&additional_blocks[i], &sequences, &opt.pattern, opt.range);
			new_blocks.sort_unstable_by(|a, b| a[0].position.cmp(&b[0].position));
			//blockfile.write_all(format!("#{}", PBlock::blocks_to_string(&vec![additional_blocks[i].clone()])).as_bytes()).expect("Unable to write data");
			//blockfile.write_all(format!("{}\n", PBlock::blocks_to_string(&new_blocks)).as_bytes()).expect("Unable to write data");
			let mut new_pairs = PBlock::pairs_from_vector(&mut new_blocks);
			additional_pairs.append(&mut new_pairs);
		}
		let add_pairs = additional_pairs.len();
		pairs.append(&mut additional_pairs);
		println!("\r- Collect additional pairs\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0);
		println!("  => {} additional pairs collected", add_pairs);
	}

	let mut result = Vec::new();
	if !opt.pars {
		print!("- Build QTrees");
		stdout().flush().unwrap();
		sw.restart();
		result = QTree::from_pairs(&pairs);
		println!("\t\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0);
		println!("  => {} QTrees built", result.len());
	}

	print!("- Save result to file");
	stdout().flush().unwrap();
	sw.restart();
	if opt.pars {
		PBlock::save_pairs_to_pars_file(&pairs, &opt.outfile);
	}
	else {
		QTree::save_to_file(&result, &opt.outfile);
	}
	println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0);
	println!("  => ...");

	println!("\t\t\t\t(Total time: {}s)", time_all.elapsed_ms() as f32/1000.0);
    Ok(())
}

// ========================================================

pub mod build_tree;

pub mod opt;


mod qtree;
pub use self::qtree::QTree;

mod sequence;
pub use self::sequence::Sequence;

mod spaced_word;
pub use self::spaced_word::SpacedWord;

mod pblock;
pub use self::pblock::PBlock;