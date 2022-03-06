use gaps_rs::opt;
use gaps_rs::Stats;
use gaps_rs::build_tree;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let opt = opt::Benchmark::from_args();
    let mut gaps_opt = opt::Gaps {
        infile: "replaced in loop".to_string(),
        fastafile: opt.fastafile,
        outfile: "will get replaced anyway".to_string(),
        format: opt.format,
        pattern: opt.pattern,
        range: opt.range,
        strong: opt.strong,
        weak: opt.weak,
        hide_progress: opt.hide_progress,
        print_pairs: false
    };

    let mut stats = Vec::new();
    for infile in fs::read_dir(opt.infolder).unwrap() {
        gaps_opt.infile = infile.unwrap().path().to_str().unwrap().to_string();
        stats.push(get_stats(gaps_opt.clone(), &opt.nwkfile));
    }

    let mut f = File::create(opt.outfile)
        .expect("Unable to create file");
    f.write_all(Stats::stats_to_csv(&stats, ",").as_bytes())
        .expect("Unable to write data");
}

// runs gaps, nwk and rfdist binaries and returns a stats struct (with valid rfdist)
fn get_stats(mut opt: opt::Gaps, nwk_file: &str) -> Stats {
    // Create temporary stuff
    let tmp_dir = gaps_rs::tools::create_tmp_folder();
    let mut tmp_outfile = tmp_dir.clone();
    tmp_outfile.push("outfile");
    let mut tmp_intree = tmp_dir.clone();
    tmp_intree.push("intree");

    // run gaps
    opt.outfile = tmp_outfile.to_str().unwrap().to_string();
    let mut stats = gaps_rs::run(opt.clone()).unwrap();

    // run qcheck if format is max-cut
    if opt.format == "max-cut" {
        let correct = gaps_rs::tools::qcheck(&opt.outfile, &opt.fastafile, nwk_file);
        stats.correct_perc = correct.1 as f64 / (correct.1 as f64 + correct.0 as f64) * 100.0;
    }

    // run nwk
    let tree = match &opt.format[..] {
        "max-cut" => build_tree::max_cut_from_file(&opt.outfile),
        "paup" => build_tree::pars(opt::Nwk { method: opt.format.clone(), infile: opt.outfile.clone(), verbose: false, all: false }),
        "phylip" => panic!("phylip pars not yet supported"),
        _ => panic!("This shouldn't happen, because structopt catches invalid inputs")
    };

    // prepare intree file
    fs::copy(nwk_file, &tmp_intree).unwrap();
    let mut file = fs::OpenOptions::new().append(true).open(&tmp_intree).unwrap();
    file.write_all(tree.as_bytes()).unwrap();

    // get rf distance
    stats.rfdist = gaps_rs::tools::rfdist(tmp_intree.to_str().unwrap()) as i64;

    // remove tmp dir
    fs::remove_dir_all(tmp_dir).unwrap();

    stats
}