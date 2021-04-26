use stopwatch::Stopwatch;
use rayon::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use std::io::Write;
use std::io::stdout;

pub fn run(opt: crate::opt::Gaps) -> Result<Stats, String> {
	let time_all = Stopwatch::start_new();

	// =============================================================================================

	if !opt.hide_progress { print!("- Reading FASTA file"); }

	stdout().flush().unwrap();
	let mut sw = Stopwatch::start_new();
	let sequences = Sequence::read_fasta_file(&opt.fastafile);

	if !opt.hide_progress { println!("\t\t(Finished in {}s)\n  => {} input sequences", sw.elapsed_ms() as f32/1000.0, sequences.len()); }

	// =============================================================================================

	if !opt.hide_progress { print!("- Reading PBlock file"); }

	stdout().flush().unwrap();
	sw.restart();
	let blocks: Vec<PBlock> = PBlock::read_from_file(&opt.infile);

	if !opt.hide_progress { println!("\t\t(Finished in {}s)\n  => {} input blocks", sw.elapsed_ms() as f32/1000.0, blocks.len()); }

	// =============================================================================================

	stdout().flush().unwrap();
	sw.restart();

	let progress_bar = if opt.hide_progress {
		ProgressBar::hidden()
	}
	else {
		ProgressBar::new(blocks.len() as u64)
			.with_style(ProgressStyle::default_bar()
				.template("- Searching for pairs\t\t{bar:20}"))
	};

	let mut pairs = blocks.into_par_iter()
		.progress_with(progress_bar)
		.fold(Vec::new, |mut acc, block| {
			if let Some(block2) = PBlock::find_matching_block(&block, &sequences, &opt.pattern, opt.range) {
				acc.push((block, block2));
			}
			acc
		})
		.reduce(Vec::new, |mut acc, mut v| {
			acc.append(&mut v);
			acc
		});

	// Filter pairs
	pairs = if opt.strong {
		pairs.into_iter().filter(|a| PBlock::strong_pair(&a.0, &a.1)).collect()
	}
	else {
		pairs.into_iter().filter(|a| QTree::new(&a.0, &a.1).is_some() && (!PBlock::strong_pair(&a.0, &a.1) || !opt.weak)).collect()
	};

	if !opt.hide_progress { println!("\r- Searching for pairs\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }

	// =============================================================================================

	if !opt.hide_progress { print!("- Saving result to file"); }

	stdout().flush().unwrap();
	sw.restart();
	match &opt.format[..] {
		"max-cut" => output::to_nwk(&QTree::from_pairs(&pairs), &opt.outfile),
		"phylip" => output::to_phylip_pars(&pairs, &opt.outfile),
		"paup" => output::to_paup(&pairs, &opt.outfile),
		_ => panic!("Invalid format (should have been caught by structopt)")
	}

	if opt.print_pairs {
		output::pairs_to_file(&pairs, "pairs.txt");
	}

	if !opt.hide_progress { println!("\t\t(Finished in {}s)", sw.elapsed_ms() as f32/1000.0); }

	// =============================================================================================

	if !opt.hide_progress { println!("\t\t\t\t(Total time: {}s)\n", time_all.elapsed_ms() as f32/1000.0); }

    Ok(Stats::new(&pairs, sequences.len()))
}

// =================================================================================================

pub mod build_tree;

pub mod opt;

pub mod tools;

pub mod output;


mod stats;
pub use self::stats::Stats;

mod qtree;
pub use self::qtree::QTree;

mod sequence;
pub use self::sequence::Sequence;

mod spaced_word;
pub use self::spaced_word::SpacedWord;

mod pblock;
pub use self::pblock::PBlock;