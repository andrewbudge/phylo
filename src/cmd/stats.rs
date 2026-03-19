use clap::Args;
use cladekit::parse_fasta;
use std::collections::HashMap;
use std::path::Path;

#[derive(Args)]
pub struct StatsArgs {
    /// FASTA alignment files
    pub files: Vec<String>,
}

/// Detect whether sequences are DNA or protein.
/// Allows IUPAC ambiguity codes (R, Y, S, W, K, M, B, D, H, V) in addition to A/T/C/G/N/-.
fn is_dna(sequences: &HashMap<String, String>) -> bool {
    for seq in sequences.values() {
        for ch in seq.chars() {
            match ch {
                'A' | 'T' | 'C' | 'G' | 'N' | '-'
                | 'R' | 'Y' | 'S' | 'W' | 'K' | 'M'
                | 'B' | 'D' | 'H' | 'V' | '?' => {}
                _ => return false,
            }
        }
    }
    true
}

/// Count variable and parsimony-informative sites in an alignment.
/// Works for both DNA and protein — counts any non-gap, non-unknown character.
/// Variable: column has at least 2 different residues (ignoring gaps/X/N/?).
/// Parsimony-informative: column has at least 2 different residues, each appearing at least twice.
fn count_informative_sites(
    sequences: &HashMap<String, String>,
    length: usize,
) -> (usize, usize) {
    let mut variable = 0;
    let mut informative = 0;
    let seqs: Vec<&String> = sequences.values().collect();

    for i in 0..length {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for seq in &seqs {
            let ch = seq.as_bytes()[i] as char;
            match ch {
                '-' | 'N' | 'X' | '?' => {} // ignore gaps and unknowns
                _ => *counts.entry(ch).or_insert(0) += 1,
            }
        }

        let bases_present = counts.len();
        let bases_twice = counts.values().filter(|&&c| c >= 2).count();

        if bases_present > 1 {
            variable += 1;
        }
        if bases_twice >= 2 {
            informative += 1;
        }
    }

    (variable, informative)
}

pub fn run(args: StatsArgs) {
    println!("file\tsequences\tlength\ttype\tgc_pct\tmissing_pct\tvariable\tvariable_pct\tinformative\tinformative_pct");

    for file in args.files {
        let (sequences, length) = parse_fasta(&file, false).expect("Failed to parse fasta file");
        let num_sequences = sequences.len();
        let dna = is_dna(&sequences);

        let mut gc_count = 0;
        let mut missing_count = 0;
        let mut total_chars = 0;
        for sequence in sequences.values() {
            for ch in sequence.chars() {
                total_chars += 1;
                match ch {
                    'G' | 'C' => gc_count += 1,
                    '-' | 'N' | 'X' | '?' => missing_count += 1,
                    _ => {}
                }
            }
        }

        let missing_pct = missing_count as f64 / total_chars as f64 * 100.0;
        let gc_str = if dna {
            let gc_pct = gc_count as f64 / (total_chars - missing_count) as f64 * 100.0;
            format!("{:.1}", gc_pct)
        } else {
            "NA".to_string()
        };

        let data_type = if dna { "DNA" } else { "AA" };
        let all_equal = sequences.values().all(|s| s.len() == length);
        let filename = Path::new(&file).file_name().unwrap().to_str().unwrap();

        if all_equal {
            let (variable, informative) = count_informative_sites(&sequences, length);
            let variable_pct = variable as f64 / length as f64 * 100.0;
            let informative_pct = informative as f64 / length as f64 * 100.0;
            println!(
                "{}\t{}\t{}\t{}\t{}\t{:.1}\t{}\t{:.1}\t{}\t{:.1}",
                filename, num_sequences, length, data_type, gc_str, missing_pct,
                variable, variable_pct, informative, informative_pct
            );
        } else {
            println!(
                "{}\t{}\tNA\t{}\t{}\t{:.1}\tNA\tNA\tNA\tNA",
                filename, num_sequences, data_type, gc_str, missing_pct
            );
        }
    }
}
