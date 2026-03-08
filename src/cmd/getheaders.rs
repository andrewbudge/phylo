use clap::Args;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Args)]
pub struct GetheadersArgs {
    /// input fasta file, read from stdin if no file is provided
    pub input: Option<String>,

    /// print only unique headers
    #[arg(short, long)]
    pub unique: bool,
}

pub fn run(args: GetheadersArgs) {
    // create a reader from file or stdin
    let reader: Box<dyn BufRead> = match args.input {
        Some(filename) => {
            let file = File::open(&filename).expect("Unable to open file");
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(std::io::stdin().lock())),
    };

    // track seen headers for unique mode
    let mut seen = HashSet::new();

    // read the file and print only the lines that start with ">" (the seq ids)
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if line.starts_with('>') {
            let header = &line[1..];
            if args.unique {
                if seen.insert(header.to_string()) {
                    println!("{}", header);
                }
            } else {
                println!("{}", header);
            }
        }
    }
}
