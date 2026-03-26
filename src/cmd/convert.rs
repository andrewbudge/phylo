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

    if first_line.starts_with('>') {
        println!("This is a fasta")
    }

    if first_line.starts_with('#') {
        println!("This is a nexus")
    }

    if first_line
        .chars()
        .next()
        .expect("empty file")
        .is_ascii_digit()
    {
        println!("This is a phylip file")
    }

    println!("This is working")
}
