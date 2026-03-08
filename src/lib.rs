use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Function to parse fasta (incredible naming)
// Takes in a fasta and makes it into a hashmap to be used by the subcommands
pub fn parse_fasta(
    filename: &str,
    validate_equal: bool,
) -> Result<(HashMap<String, String>, usize), String> {
    let file = File::open(filename).map_err(|e| format!("Could not open {}: {}", filename, e))?;

    // file reader
    let reader = BufReader::new(file);

    // Main map and then vars to track seqs as we read
    let mut sequences = HashMap::new();
    let mut current_header = String::new();
    let mut current_seq = String::new();
    let mut expected_length: Option<usize> = None;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error reading file: {}", e))?;
        let line = line.trim().to_string();

        if line.starts_with('>') {
            // save previous sequence if we have one
            if !current_header.is_empty() {
                // TODO: uppercase current_seq and insert into sequences
                // TODO: validate length if validate_equal is true
            }
            // start new header
            current_header = line[1..].to_string();
            current_seq.clear();
        } else if !line.is_empty() {
            current_seq.push_str(&line);
        }
    }
    // make the complier happy
    Ok((sequences, 0))
}
