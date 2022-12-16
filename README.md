# Gap-SpaM

This is a program that utilizes information provided by insertion and deletion events in DNA sequences as phylogenetic signals. For more information, see:

Birth N, Dencker T, Morgenstern B (2022) Insertions and deletions as phylogenetic signal in an alignment-free context.  
PLOS Computational Biology 18(8): e1010303. https://doi.org/10.1371/journal.pcbi.1010303

---
**Please note:** This program is a prototype implementation; we wanted to demonstrate that insertions and deletions contain useful information for phylogeny reconstruction. One may use our software in phylogeny studies in addition to established approaches, since it is based on a complementary source of information. However, we do not recommend to use our tool as an alternative to those established methods.

---
## Prerequisites

You will need Rust and Cargo for compilation. See https://www.rust-lang.org/tools/install for installation instructions.

## Usage

```
cargo run --release -- -i <input file> -f <FASTA file>
```

- `-i <input file>`: The input file which contains the reference blocks. See section [Input file](#Input-file) for more information.
- `-f <FASTA file>`: A FASTA file that contains all sequences from the reference blocks.

There are some other, optional flags and parameters.

- `-o <output file>`: The name of the ouput file (Default: `outfile`).
- `--format (max-cut|paup)`: The format of the output file (Default: `paup`). `max-cut` saves the constructed quartet trees in Newick notation. `paup` generates an outfile that can be used as input for [PAUP*](https://paup.phylosolutions.com). 
- `-p <pattern>`: The pattern of the newly generated blocks (Default: `1111111`).
- `--range <range>`: The size of the window in which the new blocks are searched (Default: 500).
- `--hide-progress`: Don't show any progress output. The summary at the end of program execution is still shown. If you don't want any output at all, just send everything to `/dev/null`.
- `--all`: Use all block pairs, regardless of strong or weak support (by default, only pairs that strongly support a topology are used).
- `--weak`: Only use block pairs that weakly support a tree topology. (If `--all` and `--weak` are both set, `--weak` is ignored.)

### Input file

The input file is expected to contain the reference quartet blocks in the following format:

```
>Sequence_1 (Pos: <position> RevComp: <0 or 1>)
<Spaced Word>
>Sequence_2 (Pos: <position> RevComp: <0 or 1>)
<Spaced Word>
>Sequence_3 (Pos: <position> RevComp: <0 or 1>)
<Spaced Word>
>Sequence_4 (Pos: <position> RevComp: <0 or 1>)
<Spaced Word>


[...]
```

Example (for one of the benchmark datasets of [AFProject](http://afproject.org)):

```
>D1Sd197 (Pos: 1286433 RevComp: 1)
TGCCATATCCGCCAGGTCTGCACGGAATTTATCAACCAGCGCACCAAACTGTGCGGCATGATTTGCCGGCGTACCGCCAAAATCAAATTCCCCCTGCATCCACACCACGG
>CB9615 (Pos: 1748612 RevComp: 1)
TGCCATATCCGCCAGGTCTGCACGGAATTTATCAACCAGCGCACCAAACTGTGCTGCGTGATTTACCGGCGTACCGCCAAAATCAAATTCCCCCTGCATCCACACCACGG
>B18BS512 (Pos: 1159899 RevComp: 1)
TGCCATATCCGCCAGGTCTGCACGGAATTTATCAACCAGCGCACCAAACTGTGCGGCATGATTTGCCGGCGTACCGCCAAAATCAAATTCCCCCTGCATCCACACCACGG
>Sakai (Pos: 1269524 RevComp: 1)
TGCCATATCCGCCAGGTCTGCACGGAATTTATCAACCAGCGCACCAAACTGTGCGGCGTGATTTACCGGCGTACCGCCAAAATCAAATTCCCCCTGCATCCACACCACGG


>B4Sb227 (Pos: 3793443 RevComp: 0)
TACGGTGCTGAAGCAACAAATGCCCTGCTTCCAGGAAAAGCCTCTAAGCATCAGGTAACATCAAATCGTACCCCAAACCGACACAGGTGGTCAGGTAGAGAATACCAAGG
>F2a301 (Pos: 218546 RevComp: 0)
TACGGTGCTGAAGCAACAAATGCCCTGCTTCCAGGAAAAGCCTCTAAGCATCAGGTAACATCAAATCGTACCCCAAACCGACACAGGTGGTCAGGTAGAGAATACCAAGG
>SMS35 (Pos: 4225021 RevComp: 0)
TACGGTGCTGAAGCAACAAATGCCCTGCTTCCAGGAAAAGCCTCTAAGCATCAGGTAACATTAAATCGTACCCCAAACCGACACAGGTGGTCAGGTAGAGAATACCAAGG
>SE11 (Pos: 4439744 RevComp: 0)
TACGGTGCTGAAGCAACAAATGCCCTGCTTCCAGGAAAAGCCTCTAAGCATCAGGTAACATCAAATCGTACCCCAAACCGACACAGGTGGTCAGGTAGAGAATACCAAGG


[...]
```

You can generate these input files by using the [Multi-SpaM](https://github.com/tdencker/Multi-SpaM) binary (not the python wrapper) with the (undocumented) flag `--print-only`. However, you have to change line 65 of [multi-SpaM.cpp](https://github.com/tdencker/multi-SpaM/blob/3230d3f586bc617f71a289813cf28624a0222046/src/multi-SpaM.cpp#L65) to output the reverse complement:

```
out_file << ">" << sequences[w_it.getSeq()].id << " (Pos: " << std::distance(sequences[w_it.getSeq()].content.begin(), w_it.getPos()) << ")" << std::endl;
```

should be

```
out_file << ">" << sequences[w_it.getSeq()].id << " (Pos: " << std::distance(sequences[w_it.getSeq()].content.begin(), w_it.getPos()) << " RevComp: " << w_it.revComp() << ")" << std::endl;
```

## Other binaries

This repository contains some additional binaries. They are probably not too relevant for anyone else. However, for the sake of completeness, this section provides a short description of them.

### rfdist

Usage: `cargo run --release --bin rfdist -- <input file>`

This is a wrapper for [phylip treedist](https://evolution.genetics.washington.edu/phylip/doc/treedist.html). It expects an input file that contains two trees in Newick notation and outputs the Robinson-Foulds distance between them. In order for this to run, `treedist` has to be in path.

(So far, I haven't found any Rust crate that can calculate the RF distance. Maybe, sometime, I will try to write a one myself, but for now this serves as an adequate workaround.)

### nwk

Usage: `cargo run --release --bin nwk -- <input file> --method (max-cut|paup)`

This binary expects an output file from the main program as input file and uses this data to build a supertree. Depending on the used method, this expects either `paup` or `max-cut-tree` to be in path. Use `-h` to show more options.

### benchmark

Usage: `cargo run --release --bin benchmark -- -i <input folder> -f <FASTA file> -n <reference tree>`

I used this binary for a better automation of my tests. The parameters are similar to the main program. However, instead of an input file it expects an input folder and executes the main program for each of the contained files. It also requires a reference tree in Newick format for some additional tests. However, because this might be a bit confusing, I would recommend that you just use the main program instead of this one. 
