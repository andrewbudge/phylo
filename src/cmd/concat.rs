use clap::Args;
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

pub fn run(args: ConcatArgs) {}
