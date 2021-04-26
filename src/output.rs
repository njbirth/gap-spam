use crate::{PBlock, QTree};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

// Symbols for parsimony matrix (the last symbol in the array is used for missing information)
const SYMBOLS: [&str; 9] = ["A", "B", "C", "D", "E", "F", "G", "H", "?"];

pub fn to_nwk(qtrees: &[QTree], filename: &str) {
    let mut f = File::create(filename).expect("Unable to create file");
    for tree in qtrees {
        f.write_all(format!("{}\n", tree).as_bytes()).expect("Unable to write data");
    }
}

pub fn pairs_to_file(pairs: &[(PBlock, PBlock)], filename: &str) {
    let mut f = File::create(filename).expect("Unable to create file");
    for pair in pairs {
        f.write_all(PBlock::pair_to_string(pair).as_bytes()).expect("Unable to write data");
    }
}

pub fn to_phylip_pars(pairs: &[(PBlock, PBlock)], filename: &str) {
    let (species, pairs, lines) = format_matrix(pairs, 9);

    let mut f = File::create(filename).expect("Unable to create file");
    f.write_all(format!("{} {}\n{}", species, pairs, lines).as_bytes()).expect("Unable to write data");
}

pub fn to_paup(pairs: &[(PBlock, PBlock)], filename: &str) {
    let (species, pairs, lines) = format_matrix(pairs, -1);
    let head = format!("#NEXUS\n\
                    begin data;\n\
                    \tdimensions ntax={} nchar={};\n\
                    \tformat datatype=standard missing={} interleave symbols=\"{}\";\n\
                    matrix\n", species, pairs, SYMBOLS.last().expect("SYMBOLS array empty"), SYMBOLS[..SYMBOLS.len()-1].join(""));
    let tail = ";\n\
                    end;\n\n\
                    begin paup;\n\
                    \tset maxtrees=1000;\n\
                    \tset increase=auto;\n\
                    \tHSearch addseq=random nreps=20;\n\
                    \tSaveTrees format=newick file=pars.nwk replace=yes;\n\
                    \tquit;\n\
                    end;";

    let mut f = File::create(filename).expect("Unable to create file");
    f.write_all(format!("{}\n{}\n{}", head, lines, tail).as_bytes()).expect("Unable to write data");
}

fn format_matrix(pairs: &[(PBlock, PBlock)], name_len: i32) -> (usize, usize, String) {
    // Collect species
    let mut species = HashSet::new();
    for pair in pairs {
        for name in pair.0.get_sequence_names() {
            species.insert(String::from(name));
        }
    }

    let name_len = if name_len == -1 {
        species.iter().map(|name| name.len()).max().unwrap()
    }
    else {
        name_len as usize
    };

    let species: Vec<String> = species.into_iter().collect();

    // Build lines
    let mut output: HashMap<String, Vec<String>> = HashMap::new();
    for name in &species {
        let name_padded = if name.len() <= name_len {
            format!("{}{}", name, " ".repeat(name_len-name.len()))
        }
        else {
            name[0..name_len].to_string()
        };
        output.insert(name.to_string(), vec![name_padded]);
    }

    let symbols = SYMBOLS.iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    for pair in pairs {
        let mut symbol_index = 0;
        let distances: HashMap<String, i64> = PBlock::get_distances(&pair.0, &pair.1);
        let mut dists_replaced: HashMap<String, String> = HashMap::new();
        for key in pair.0.get_sequence_names() {
            dists_replaced.insert(String::from(key), String::from("0"));
        }

        for (seq, dist) in distances.iter() {
            if dists_replaced[seq] != "0" {
                continue;
            }
            for seq2 in pair.0.get_sequence_names() {
                if distances[seq2] == *dist {
                    dists_replaced.insert(seq2.to_string(), symbols[symbol_index].clone());
                }
            }
            if symbol_index < symbols.len()-1 {
                symbol_index += 1;
            }
        }

        // Add symbols to output lines
        for s in &species {
            output.get_mut(s).unwrap().push(
                dists_replaced.get(s)
                    .unwrap_or_else(|| symbols.last().unwrap())
                    .to_string()
            );
        }
    }

    let lines = output.values()
        .map(|line| line.join(" "))
        .collect::<Vec<String>>()
        .join("\n");

    (species.len(), pairs.len(), lines)
}