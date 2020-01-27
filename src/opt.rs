use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gaps_rust", about = "Mind the gap!")]
pub enum Opt {
	/// Constructs quartet trees from P-blocks
	#[structopt(name = "qtrees")]
	QTrees(QTrees),
	/// Convert PBlocks to an outfile for phylip pars
	#[structopt(name = "pars")]
	Pars(Pars),
	/// Constructs a tree from quartet trees
	#[structopt(name = "nwk")]
	Nwk(Nwk),
}

#[derive(StructOpt, Debug)]
pub struct QTrees {
	/// output file for quartet trees
	#[structopt(short = "o", default_value = "outfile")]
	pub outfile: String,
	/// pattern for additional blocks
	#[structopt(short = "p", long = "pattern", default_value = "1111111")]
	pub pattern: String,
	/// search for additional QTrees
	#[structopt(short = "a")]
	pub additional_trees: bool,
	/// range for -a
	#[structopt(long = "range", default_value = "500")]
	pub range: i64,
	/// input file with P-blocks
	#[structopt()]
	pub infile: String,
	/// sequence file (FASTA)
	#[structopt()]
	pub fastafile: String,
}

#[derive(StructOpt, Debug)]
pub struct Pars {
	/// output file for quartet trees
	#[structopt(short = "o", default_value = "outfile")]
	pub outfile: String,
	/// pattern for additional blocks
	#[structopt(short = "p", long = "pattern", default_value = "1111111")]
	pub pattern: String,
	/// search for additional pairs
	#[structopt(short = "a")]
	pub additional_pairs: bool,
	/// range for -a
	#[structopt(long = "range", default_value = "500")]
	pub range: i64,
	/// input file with P-blocks
	#[structopt()]
	pub infile: String,
	/// sequence file (FASTA)
	#[structopt()]
	pub fastafile: String,
	/// P block size (0 for variable size)
	#[structopt(short = "s", default_value = "0")]
	pub blocksize: u32,
}

#[derive(StructOpt, Debug)]
pub struct Nwk {
	/// use phylip pars instead of max-cut (requires applicable input file)
	#[structopt(long = "pars")]
	pub pars: bool,
	/// input file with quartet trees
	#[structopt()]
	pub infile: String
}