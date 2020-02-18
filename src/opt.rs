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

use structopt::StructOpt;

// === Options for gaps-binary =================================================

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

	/// Write outfile for phylip pars instead of writing quartet trees
	#[structopt(long = "pars")]
	pub pars: bool,
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
	// show all found trees (paup only; max-cut always returns one tree)
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