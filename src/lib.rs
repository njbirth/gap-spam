use stopwatch::Stopwatch;
use rayon::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use std::io::Write;
use std::io::stdout;

pub fn run(opt: crate::opt::Gaps) -> Result<Stats, String> {
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
	let blocksize = if opt.format != "nwk" { opt.blocksize } else { 4 };
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
		stdout().flush().unwrap();
		sw.restart();

		let progress_bar = if opt.hide_progress {
			ProgressBar::hidden()
		}
		else {
			ProgressBar::new(additional_blocks.len() as u64)
				.with_style(ProgressStyle::default_bar()
					.template("- Collect additional pairs\t{bar:20}"))
		};

		let mut additional_pairs = additional_blocks.into_par_iter()
			.progress_with(progress_bar)
			.fold(Vec::new, |mut acc, block| {
				if let Some(block2) = PBlock::find_matching_block(&block, &sequences, &opt.pattern, opt.range, opt.perfect) {
					acc.push((block, block2));
				}
				acc
			})
			.reduce(Vec::new, |mut acc, mut v| {
				acc.append(&mut v);
				acc
			});

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
	match &opt.format[..] {
		"nwk" => output::to_nwk(&result, &opt.outfile),
		"phylip" => output::to_phylip_pars(&pairs, &opt.outfile),
		"paup" => output::to_paup(&pairs, &opt.outfile),
		_ => panic!("Invalid format (should have been caught by structopt)")
	}

	if !opt.hide_progress { println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }

	if !opt.hide_progress { println!("\t\t\t\t(Total time: {}s)\n", time_all.elapsed_ms() as f32/1000.0); }

	// Return stats

	let max_coverage = (rep_sequences*(rep_sequences-1)*(rep_sequences-2)*(rep_sequences-3)) as f64 / 24.0;
	let blocks_sum = rep_input_pairs as f64 + rep_add_pairs as f64;

    Ok(Stats {
		sequences: rep_sequences as u64,
		blocks: rep_pblocks as u64,
		pairs: rep_input_pairs as u64,
		new_pairs: rep_add_pairs as u64,
		pairs_22: rep_count[2],
		pairs_22_perc: rep_count[2] as f64 / blocks_sum * 100.0,
		pairs_211: rep_count[3],
		pairs_211_perc: rep_count[3] as f64 / blocks_sum * 100.0,
		pairs_1111: rep_count[4],
		pairs_1111_perc: rep_count[4] as f64 / blocks_sum * 100.0,
		pairs_31: rep_count[0],
		pairs_31_perc: rep_count[0] as f64 / blocks_sum * 100.0,
		pairs_4: rep_count[1],
		pairs_4_perc: rep_count[1] as f64 / blocks_sum * 100.0,
		qtrees: rep_trees as u64,
		coverage: rep_trees_unique as f64 / max_coverage * 100.0,
		rfdist: -1
	})
}

pub struct Stats {
	// number of input sequences
	pub sequences: u64,
	// number of input blocks
	pub blocks: u64,
	// number of pairs from to input blocks
	pub pairs: u64,
	// number of pairs from an input block with a new block
	pub new_pairs: u64,
	// number of pairs of a distance category
	pub pairs_22: u64,
	pub pairs_22_perc: f64,
	pub pairs_211: u64,
	pub pairs_211_perc: f64,
	pub pairs_1111: u64,
	pub pairs_1111_perc: f64,
	pub pairs_31: u64,
	pub pairs_31_perc: f64,
	pub pairs_4: u64,
	pub pairs_4_perc: f64,
	// number of quartet trees
	pub qtrees: u64,
	// coverage
	pub coverage: f64,
	// RF-distance (use -1 if unknown)
	pub rfdist: i64
}

// ========================================================

pub mod build_tree;

pub mod opt;

pub mod tools;

pub mod output;


mod qtree;
pub use self::qtree::QTree;

mod sequence;
pub use self::sequence::Sequence;

mod spaced_word;
pub use self::spaced_word::SpacedWord;

mod pblock;
pub use self::pblock::PBlock;