use structopt::StructOpt;

// === Options for gaps-binary =================================================

fn check_format(input: &str) -> Result<String, String> {
	match input {
		"nwk" | "phylip" | "paup" => Ok(input.to_string()),
		_ => Err(input.to_string())
	}
}

#[derive(Debug, StructOpt)]
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

	/// Output format (nwk|phylip|paup)
	#[structopt(long = "format", default_value = "paup", parse(try_from_str = check_format))]
	pub format: String,
	/// P block size (0 for variable size; always 4 if --pars is not set)
	#[structopt(short = "s", default_value = "0")]
	pub blocksize: u32,

	/// search for additional pairs (use twice for extended search)
	#[structopt(short = "a", parse(from_occurrences))]
	pub additional: u8,
	/// pattern for additional blocks (ignored if -a is not set)
	#[structopt(short = "p", long = "pattern", default_value = "1111111")]
	pub pattern: String,
	/// range for -a (ignored if -a is not set)
	#[structopt(long = "range", default_value = "500")]
	pub range: i64,

	/// use only perfect pairs (A/A/B/B)
	#[structopt(long = "perfect")]
	pub perfect: bool,
	/// use only imperfect pairs (A/A/B/C)
	#[structopt(long = "imperfect")]
	pub imperfect: bool,
	/// Hide progress output
	#[structopt(long = "hide-progress")]
	pub hide_progress: bool,
}

// === Options for nwk-binary ==================================================

#[derive(StructOpt, Debug)]
#[structopt(name = "nwk", about = "Construct a tree from quartet trees (requires quartet-max-cut to be in path)")]
pub struct Nwk {
	/// use parsimony (requires an input file for phylip pars; paup and seqmagick have to be in path)
	#[structopt(long = "pars")]
	pub pars: bool,
	/// input file with quartet trees
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