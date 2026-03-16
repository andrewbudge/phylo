use clap::Args;
use phylo::{load_taxa_list, parse_fasta};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
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

    /// Provenance TSV output file
    #[arg(short = 'l', long = "log")]
    pub log: String,
}

/// Match taxa names to FASTA headers using case-insensitive substring search.
/// Longer taxa names match first to prevent partial match collisions
/// (e.g., "Mus musculus domesticus" claims before "Mus musculus").
/// Once a header is claimed, no other taxon can match it.
/// Returns taxon -> (original header, sequence) so we can track provenance.
fn match_taxa(
    taxa: &[String],
    sequences: &HashMap<String, String>,
) -> HashMap<String, (String, String)> {
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
                results.insert(taxon.clone(), (header.clone(), sequence.clone()));
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
                entry.push_str(&matched[taxon].1);
            } else {
                entry.push_str(&args.missing.repeat(**length));
            }
        }
    }

    // Build partition boundaries (used by both output formats)
    let mut partitions = Vec::new();
    let mut position = 1;
    for (file, _matched, length) in &matched_genes {
        let name = Path::new(file).file_name().unwrap().to_str().unwrap();
        partitions.push((name.to_string(), position, position + *length - 1));
        position = position + *length;
    }
    let total_length = position - 1;

    let fmt = args.format.to_lowercase();
    if fmt == "nexus" || fmt == "n" || fmt == "nex" {
        // NEXUS: complete file to stdout (data + partitions in one)
        println!("#NEXUS");
        println!("BEGIN DATA;");
        println!("  DIMENSIONS NTAX={} NCHAR={};", taxa.len(), total_length);
        println!("  FORMAT DATATYPE=DNA MISSING={} GAP=-;", args.missing);
        println!("  MATRIX");
        for taxon in &taxa {
            println!("  {}    {}", taxon, supermatrix[taxon]);
        }
        println!(";");
        println!("END;");
        println!("BEGIN SETS;");
        for (name, start, end) in &partitions {
            println!("  CHARSET {} = {}-{};", name, start, end);
        }
        println!("END;");
    } else {
        // FASTA: supermatrix to stdout, partitions to stderr
        for taxon in &taxa {
            println!(">{}", taxon);
            println!("{}", supermatrix[taxon]);
        }
        for (name, start, end) in &partitions {
            eprintln!("{} = {}-{}", name, start, end);
        }
    }

    // Write provenance TSV — shows which original header matched each taxon per gene
    let mut log_file = File::create(&args.log).expect("Failed to create provenance log file");
    // Header row: taxa list filename, then each gene filename
    let gene_names: Vec<String> = matched_genes
        .iter()
        .map(|(file, _, _)| {
            Path::new(file)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect();
    writeln!(log_file, "{}\t{}", args.taxa_list, gene_names.join("\t"))
        .expect("Failed to write to log file");
    // One row per taxon: taxon name, then matched header or MISSING
    for taxon in &taxa {
        let mut row = vec![taxon.clone()];
        for (_file, matched, _length) in &matched_genes {
            if matched.contains_key(taxon) {
                row.push(matched[taxon].0.clone());
            } else {
                row.push("MISSING".to_string());
            }
        }
        writeln!(log_file, "{}", row.join("\t")).expect("Failed to write to log file");
    }
}
