use std::fmt;
use crate::PBlock;

#[derive(Debug, Clone)]
pub struct Stats {
    // number of pairs from to input blocks
    pub total_pairs: usize,
    // number of pairs of a distance category
    pub strong_pairs: usize,
    pub weak_pairs: usize,
    // correct pairs in percent
    pub correct_perc: f64,
    // coverage
    pub coverage_perc: f64,
    // RF-distance (use -1 if unknown)
    pub rfdist: i64
}

impl Stats {
    pub fn new(pairs: &Vec<(PBlock, PBlock)>, seq_num: usize) -> Stats {
        let total_pairs = pairs.len();
        let strong_pairs = pairs.iter()
            .filter(|(a, b)| PBlock::strong_pair(a, b))
            .count();
        let weak_pairs = total_pairs - strong_pairs;
        let correct_perc = -1.0;
        let rfdist = -1;

        let max_coverage = (seq_num*(seq_num-1)*(seq_num-2)*(seq_num-3)) as f64 / 24.0;
        let mut unique_pairs = pairs.clone();
        unique_pairs.sort_unstable_by(|a, b| {
            a.0.get_sequence_names().cmp(&b.0.get_sequence_names())
        });
        unique_pairs.dedup_by(|a, b| {
            a.0.get_sequence_names() == b.0.get_sequence_names()
        });
        let coverage_perc = unique_pairs.len() as f64 / max_coverage * 100.0;

        Stats {
            total_pairs,
            strong_pairs,
            weak_pairs,
            correct_perc,
            coverage_perc,
            rfdist
        }
    }
    pub fn stats_to_csv(stats: &[Stats], separator: &str) -> String {
        let header = ["pairs", "pairs_22", "pairs_22_perc",
            "pairs_211", "pairs_211_perc", "correct_perc", "coverage", "rfdist"];

        let mut content = header.join(separator);
        for stat in stats {
            content = format!("{}\n{}", content, stat.to_csv(separator));
        }

        content
    }

    fn to_csv(&self, separator: &str) -> String {
        let values = [
            self.total_pairs.to_string(),
            self.strong_pairs.to_string(),
            (self.strong_pairs as f64 / self.total_pairs as f64 * 100.0).to_string(),
            self.weak_pairs.to_string(),
            (self.weak_pairs as f64 / self.total_pairs as f64 * 100.0).to_string(),
            self.correct_perc.to_string(),
            self.coverage_perc.to_string(),
            self.rfdist.to_string()
        ];

        values.join(separator)
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s = format!("{}================== REPORT ==================\n", s);
        s = format!("{}Total pairs: \t{}\n", s, self.total_pairs);
        s = format!("{}Strong pairs: \t{} \t({:.2}%)\n", s, self.strong_pairs, self.strong_pairs as f64 / self.total_pairs as f64 * 100.0);
        s = format!("{}Weak pairs: \t{} \t({:.2}%)\n", s, self.weak_pairs, self.weak_pairs as f64 / self.total_pairs as f64 * 100.0);
        s = format!("{}Coverage: \t{:.2}%\n", s, self.coverage_perc);
        s = format!("{}============================================", s);

        write!(f, "{}", s)
    }
}