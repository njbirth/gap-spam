use crate::{PBlock, QTree};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

// Symbols for parsimony matrix (the last symbol in the array is used for missing information)
const SYMBOLS: [&str; 9] = ["A", "B", "C", "D", "E", "F", "G", "H", "?"];

pub fn to_nwk(qtrees: &Vec<QTree>, filename: &str) {
    let mut f = File::create(filename).expect("Unable to create file");
    for i in 0..qtrees.len() {
        f.write_all(format!("{}\n", qtrees[i]).as_bytes()).expect("Unable to write data");
    }
}

pub fn to_phylip_pars(pairs: &Vec<(PBlock, PBlock)>, filename: &str) {
    let (species, pairs, lines) = format_matrix(pairs, 10);

    let mut f = File::create(filename).expect("Unable to create file");
    f.write_all(format!("{} {}\n{}", species, pairs, lines).as_bytes()).expect("Unable to write data");
}

pub fn to_paup(pairs: &Vec<(PBlock, PBlock)>, filename: &str) {
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

fn format_matrix(pairs: &Vec<(PBlock, PBlock)>, name_len: i32) -> (usize, usize, String) {
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
    } - 1;

    let species: Vec<String> = species.into_iter()
        .map(|name| {
            if name.len() <= name_len {
                format!("{}{}", name, " ".repeat(name_len - name.len()))
            }
            else {
                String::from(&name[..name_len])
            }
        })
        .collect();

    // Build lines
    let mut output: HashMap<String, Vec<&str>> = HashMap::new();
    for i in 0..species.len() {
        output.insert(species[i].clone(), vec![&species[i]]);
    }

    for pair in pairs {
        let mut symbol_index = 0;
        let distances: HashMap<String, i64> = PBlock::get_distances(&pair.0, &pair.1);
        let mut dists_replaced: HashMap<String, &str> = HashMap::new();
        for key in pair.0.get_sequence_names() {
            dists_replaced.insert(String::from(key), "0");
        }

        for (seq, dist) in distances.iter() {
            if dists_replaced[seq] != "0" {
                continue;
            }
            for seq2 in pair.0.get_sequence_names() {
                if distances[seq2] == *dist {
                    dists_replaced.insert(seq2.to_string(), SYMBOLS[symbol_index]);
                }
            }
            if symbol_index < SYMBOLS.len()-1 {
                symbol_index += 1;
            }
        }

        // Ausgabezeilen ergänzen
        for s in &species {
            let c = dists_replaced.get(s);
            if c.is_none() {
                output.get_mut(s).unwrap().push(SYMBOLS.last().expect("SYMBOLS array empty"));
            }
            else {
                output.get_mut(s).unwrap().push(c.unwrap());
            }
        }
    }

    let lines = output.values()
        .map(|line| line.join(" "))
        .collect::<Vec<String>>()
        .join("\n");

    (species.len(), pairs.len(), lines)
}