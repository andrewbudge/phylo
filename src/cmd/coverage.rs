use cladekit::print_table;
use clap::Args;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Args)]
pub struct CoverageArgs {
    /// Provenance TSV file from concat
    pub tsv: String,

    /// Print Coverage by Loci
    #[arg(short = 'l', long = "loci")]
    pub loci_cov: bool,

    /// Print Coverage by Taxa
    #[arg(short = 't', long = "taxa")]
    pub taxa_cov: bool,

    /// Print output in more human readable way
    #[arg(short = 'p', long = "pretty")]
    pub pretty: bool,
}

pub fn run(args: CoverageArgs) {
    let file = File::open(&args.tsv).expect("Failed to open TSV");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let header = lines
        .next()
        .expect("empty file")
        .expect("Failed to read header");
    let total_loci = header.split('\t').count() - 1;

    // Print the coverage by taxa
    if args.taxa_cov {
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![
            "taxa".into(),
            "loci_present".into(),
            "loci_missing".into(),
            "pct_missing".into(),
        ]);
        for line in lines {
            let line = line.expect("Failed to read line");
            let fields: Vec<&str> = line.split('\t').collect();
            let present_count = fields[1..].iter().filter(|f| **f != "MISSING").count();
            let missing_count = total_loci - present_count;
            let missing_pct = (missing_count as f64 / total_loci as f64) * 100.0;
            rows.push(vec![
                fields[0].to_string(),
                format!("{}/{}", present_count, total_loci),
                format!("{}/{}", missing_count, total_loci),
                format!("{:.1}%", missing_pct),
            ]);
        }
        print_table(&rows, args.pretty);
    }
    // print the coverage by loci
    else if args.loci_cov {
        let mut loci_count: Vec<usize> = vec![0; total_loci];
        let mut total_taxa = 0;
        for line in lines {
            total_taxa += 1;
            let line = line.expect("Failed to read line");
            let fields: Vec<&str> = line.split('\t').collect();
            for (i, field) in fields[1..].iter().enumerate() {
                if *field != "MISSING" {
                    loci_count[i] += 1;
                }
            }
        }

        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![
            "loci".into(),
            "appearance_count".into(),
            "missing_pct".into(),
        ]);
        for (i, loci) in header.split('\t').skip(1).enumerate() {
            let missing = total_taxa - loci_count[i];
            let missing_pct = (missing as f64 / total_taxa as f64) * 100.0;
            rows.push(vec![
                loci.to_string(),
                format!("{}/{}", loci_count[i], total_taxa),
                format!("{:.1}%", missing_pct),
            ]);
        }
        print_table(&rows, args.pretty);
    }
}
