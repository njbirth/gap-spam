use gaps_rs::opt::Gaps;
use structopt::StructOpt;

fn main() {
	let opt = Gaps::from_args();
	let stats = gaps_rs::run(opt).unwrap();

	println!("================== REPORT ==================");
	println!("input sequences: {}", stats.sequences);
	println!("input p-blocks: {}", stats.blocks);
	println!("pairs from input blocks: {}", stats.pairs);
	println!("pairs with new blocks: {}", stats.new_pairs);
	println!("categories:");
	println!("    2-2: \t{} \t({:.2}%)", stats.pairs_22, stats.pairs_22_perc);
	println!("    2-1-1: \t{} \t({:.2}%)", stats.pairs_211, stats.pairs_211_perc);
	println!("    1-1-1-1: \t{} \t({:.2}%)", stats.pairs_1111, stats.pairs_1111_perc);
	println!("    3-1: \t{} \t({:.2}%)", stats.pairs_31, stats.pairs_31_perc);
	println!("    4: \t\t{} \t({:.2}%)", stats.pairs_4, stats.pairs_4_perc);
	println!("quartet trees: {}", stats.qtrees);
	println!("coverage: {:.2}%", stats.coverage);
	println!("============================================");
}