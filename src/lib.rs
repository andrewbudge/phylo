use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Parse a FASTA file into a map of header -> sequence.
/// When validate_equal is true, all sequences must be the same length (for alignments).
pub fn parse_fasta(
    filename: &str,
    validate_equal: bool,
) -> Result<(HashMap<String, String>, usize), String> {
    let file = File::open(filename).map_err(|e| format!("Could not open {}: {}", filename, e))?;
    let reader = BufReader::new(file);

    let mut sequences = HashMap::new();
    let mut current_header = String::new();
    let mut current_seq = String::new();
    let mut expected_length: Option<usize> = None;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error reading file: {}", e))?;
        let line = line.trim().to_string();

        if line.starts_with('>') {
            // Save the previous sequence before starting a new one
            if !current_header.is_empty() {
                current_seq = current_seq.to_uppercase();
                sequences.insert(current_header.clone(), current_seq.clone());
                if validate_equal {
                    match expected_length {
                        None => expected_length = Some(current_seq.len()),
                        Some(len) => {
                            if current_seq.len() != len {
                                return Err(format!(
                                    "Error: Sequence length mismatch in {} : {}",
                                    filename, current_header
                                ));
                            }
                        }
                    }
                }
            }
            current_header = line[1..].to_string();
            current_seq.clear();
        } else if !line.is_empty() {
            current_seq.push_str(&line);
        }
    }

    // Save the last sequence (loop only saves when it hits the next '>')
    if !current_header.is_empty() {
        current_seq = current_seq.to_uppercase();
        // None means this is the only sequence — nothing to compare against
        if validate_equal {
            if let Some(len) = expected_length {
                if current_seq.len() != len {
                    return Err(format!(
                        "Error: Sequence length mismatch in {} : {}",
                        filename, current_header
                    ));
                }
            }
        }
        sequences.insert(current_header, current_seq);
    }

    let length = sequences.values().next().map_or(0, |s| s.len());
    Ok((sequences, length))
}

/// Pretty print a table of rows. When pretty is false, prints tab-separated.
/// When pretty is true, pads columns to align.
pub fn print_table(rows: &[Vec<String>], pretty: bool) {
    if !pretty {
        for row in rows {
            println!("{}", row.join("\t"));
        }
        return;
    }
    let num_cols = rows[0].len();
    let mut widths = vec![0usize; num_cols];
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if cell.len() > widths[i] {
                widths[i] = cell.len();
            }
        }
    }
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                print!("  ");
            }
            print!("{:<width$}", cell, width = widths[i]);
        }
        println!();
    }
}

/// Read a taxa list file (one name per line) into a Vec.
pub fn load_taxa_list(filename: &str) -> Result<Vec<String>, String> {
    let file = File::open(filename).map_err(|e| format!("Could not open {}: {}", filename, e))?;
    let reader = BufReader::new(file);

    let mut taxa = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error reading file: {}", e))?;
        let line = line.trim().to_string();
        if !line.is_empty() {
            taxa.push(line);
        }
    }

    Ok(taxa)
}
