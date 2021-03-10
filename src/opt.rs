use structopt::StructOpt;

// === Options for gaps-binary =================================================

fn check_format(input: &str) -> Result<String, String> {
	match input {
		"max-cut" | "paup" => Ok(input.to_string()),
		_ => Err(input.to_string())
	}
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "gaps", about = "Mind the gap!")]
pub struct Gaps {
	/// input file with P-blocks
	#[structopt(short = "i")]
	pub infile: String,
	/// sequence file (FASTA)
	#[structopt(short = "f")]
	pub fastafile: String,
	/// output file
	#[structopt(short = "o", default_value = "outfile")]
	pub outfile: String,

	/// Output format (nwk|paup)
	#[structopt(long = "format", default_value = "paup", parse(try_from_str = check_format))]
	pub format: String,
	/// P block size (0 for variable size; always 4 if --pars is not set)
	#[structopt(short = "s", default_value = "0")]
	pub blocksize: u32,

	/// pattern for new blocks
	#[structopt(short = "p", long = "pattern", default_value = "1111111")]
	pub pattern: String,
	/// range for new blocks
	#[structopt(long = "range", default_value = "500")]
	pub range: i64,

	/// use only pairs that strongly support a topology
	#[structopt(long = "strong")]
	pub strong: bool,
	/// use only pairs that weakly support a topology
	#[structopt(long = "weak")]
	pub weak: bool,
	/// Hide progress output
	#[structopt(long = "hide-progress")]
	pub hide_progress: bool,
}

// === Options for nwk-binary ==================================================

#[derive(StructOpt, Debug)]
#[structopt(name = "nwk", about = "Constructs a tree from a gaps outfile")]
pub struct Nwk {
	/// Method for building trees (max-cut|paup); requires "max-cut-tree"/"paup" to be in path
	#[structopt(long = "method", default_value = "paup", parse(try_from_str = check_format))]
	pub method: String,
	/// input file
	#[structopt()]
	pub infile: String,
	/// show paup output
	#[structopt(short = "v")]
	pub verbose: bool,
	/// show all found trees (paup only; max-cut always returns one tree)
	#[structopt(long = "all")]
	pub all: bool
}

// === Options for rfdist-binary ===============================================

#[derive(StructOpt, Debug)]
#[structopt(name = "rfdist", about = "Returns the Robinson-Foulds-distancs between two trees (requires phylip treedist to be in path)")]
pub struct Rfdist {
	/// input file (two trees in FASTA format)
	#[structopt()]
	pub infile: String
}

// === Options for benchmark-binary =============================================

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "benchmark", about = "Testing")]
pub struct Benchmark {
	/// input folder with P-block files
	#[structopt(short = "i")]
	pub infolder: String,
	/// sequence file (FASTA)
	#[structopt(short = "f")]
	pub fastafile: String,
	/// nwk reference tree file
	#[structopt(short = "n")]
	pub nwkfile: String,
	/// output file (csv)
	#[structopt(short = "o", default_value = "results.csv")]
	pub outfile: String,

	/// Output format (nwk|paup)
	#[structopt(long = "format", default_value = "paup", parse(try_from_str = check_format))]
	pub format: String,
	/// P block size (0 for variable size; always 4 if --pars is not set)
	#[structopt(short = "s", default_value = "0")]
	pub blocksize: u32,

	/// pattern for new blocks
	#[structopt(short = "p", long = "pattern", default_value = "1111111")]
	pub pattern: String,
	/// range for new blocks
	#[structopt(long = "range", default_value = "500")]
	pub range: i64,

	/// use only pairs that strongly support a topology
	#[structopt(long = "strong")]
	pub strong: bool,
	/// use only pairs that weakly support a topology
	#[structopt(long = "weak")]
	pub weak: bool,
	/// Hide progress output
	#[structopt(long = "hide-progress")]
	pub hide_progress: bool,
}