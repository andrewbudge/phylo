use cladekit::{is_dna, parse_fasta};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Args;

#[derive(Args)]
pub struct ConvertArgs {
    /// Input file
    pub input_file: String,

    /// Output format: f (fasta), n (nexus), sp (strict phylip), rp (relaxed phylip)
    #[arg(short = 'o', long = "output_format")]
    pub output_format: String,
}

pub fn run(args: ConvertArgs) {
    let file = File::open(&args.input_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let first_line = lines
        .next()
        .expect("empty file")
        .expect("Failed to read line");

    let sequences = if first_line.starts_with('>') {
        let (seqs, _) = parse_fasta(&args.input_file, false).expect("Failed to parse fasta file");
        seqs
    } else if first_line.starts_with('#') {
        // TODO: parse_nexus
        todo!("NEXUS parsing not yet implemented")
    } else if first_line.chars().next().expect("empty line").is_ascii_digit() {
        // TODO: parse_phylip
        todo!("PHYLIP parsing not yet implemented")
    } else {
        eprintln!("Error: could not detect input format");
        std::process::exit(1);
    };

    // TODO: write sequences in requested output format
    println!("Parsed {} sequences", sequences.len());
}
