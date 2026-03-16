use clap::Args;
use phylo::{load_taxa_list, parse_fasta};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Args)]
pub struct ConcatArgs {
    /// final taxa alias list (one name per line)
    #[arg(value_name = "TAXA LIST")]
    pub taxa_list: String,

    /// FASTA files
    pub files: Vec<String>,

    /// Output format (FASTA or Nexus)
    #[arg(short, long, default_value = "FASTA")]
    pub format: String,

    /// Missing character
    #[arg(short, long, default_value = "N")]
    pub missing: String,
}

/// Match taxa names to FASTA headers using case-insensitive substring search.
/// Longer taxa names match first to prevent partial match collisions
/// (e.g., "Mus musculus domesticus" claims before "Mus musculus").
/// Once a header is claimed, no other taxon can match it.
fn match_taxa(taxa: &[String], sequences: &HashMap<String, String>) -> HashMap<String, String> {
    let mut sorted_taxa = taxa.to_vec();
    sorted_taxa.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut claimed_headers = HashSet::new();
    let mut results = HashMap::new();

    for taxon in &sorted_taxa {
        for (header, sequence) in sequences {
            if claimed_headers.contains(header) {
                continue;
            }
            if header
                .to_lowercase()
                .contains(&taxon.to_lowercase().replace("_", " "))
            {
                claimed_headers.insert(header.clone());
                results.insert(taxon.clone(), sequence.clone());
                break;
            }
        }
    }
    results
}

pub fn run(args: ConcatArgs) {
    let taxa = load_taxa_list(&args.taxa_list).expect("Failed to load taxa list");

    let mut gene_data = Vec::new();
    for file in &args.files {
        let (sequences, length) = parse_fasta(file, true).expect("Failed to parse fasta file");
        gene_data.push((file, sequences, length));
    }

    let mut matched_genes = Vec::new();
    for (file, sequences, length) in &gene_data {
        let matched = match_taxa(&taxa, sequences);
        matched_genes.push((file, matched, length));
    }

    // Build supermatrix: concatenate matched sequences per taxon, fill gaps with missing char
    let mut supermatrix: HashMap<String, String> = HashMap::new();
    for taxon in &taxa {
        for (_file, matched, length) in &matched_genes {
            let entry = supermatrix.entry(taxon.clone()).or_insert(String::new());
            if matched.contains_key(taxon) {
                entry.push_str(&matched[taxon]);
            } else {
                entry.push_str(&args.missing.repeat(**length));
            }
        }
    }

    // Output supermatrix as FASTA to stdout
    for taxon in &taxa {
        println!(">{}", taxon);
        println!("{}", supermatrix[taxon]);
    }

    // Output partition boundaries to stderr
    let mut position = 1;
    for (file, _matched, length) in matched_genes {
        eprintln!(
            "{} = {}-{}",
            Path::new(file).file_name().unwrap().to_str().unwrap(),
            position,
            position + length - 1
        );
        position = position + length;
    }
}
