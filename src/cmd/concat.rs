use clap::Args;
use phylo::{load_taxa_list, parse_fasta};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Args)]
pub struct ConcatArgs {
    /// taxa name list
    #[arg(value_name = "TAXA LIST")]
    pub taxa_list: String,

    /// fasta files
    pub files: Vec<String>,

    /// Output format (FASTA or Nexus)
    #[arg(short, long, default_value = "fasta")]
    pub format: String,

    /// Missing character
    #[arg(short, long, default_value = "N")]
    pub missing: String,
}

pub fn run(args: ConcatArgs) {
    // load in taxa list
    let taxa = load_taxa_list(&args.taxa_list).expect("Failed to load taxa list");

    let mut gene_data = Vec::new();

    for file in &args.files {
        let (sequences, length) = parse_fasta(file, true).expect("Failed to parse fasta file");
        gene_data.push((file, sequences, length));
    }
}
