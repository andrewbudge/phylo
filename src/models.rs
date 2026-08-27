use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// A single nucleotide record returned by `query`, populated from an NCBI
/// esummary docsum. No homology/locus information is present at this stage —
/// that is determined later by MMseqs2 in `extract`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Accession {
    pub accession: String,
    pub taxon_name: String,
    /// The record's own source-organism TaxID (from the docsum), e.g. a single
    /// species. Distinct from [`Accession::query_taxid`].
    pub taxid: u64,
    pub length: usize,
    /// GenBank title string (e.g. "...cytochrome c oxidase subunit I..."). Recorded
    /// for traceability only; deliberately NOT used as a homology/quality filter.
    pub gene_annotation: String,
    pub refseq: bool,
    pub source_db: String,
    /// The higher-level TaxID that was queried to surface this record (the
    /// ingroup/outgroup root the user supplied). Stamped here so provenance
    /// survives flattening, and so the lineage check has the root to compare
    /// [`Accession::taxid`] against.
    pub query_taxid: u64,
    /// Which group the querying root belongs to. Stamped per-record (rather than
    /// only on the enclosing [`QueryResult`]) so it travels with the accession
    /// once the array is flattened downstream.
    pub taxon_group: TaxonGroup,
    /// Flagged (never silently dropped) when a record's source TaxID does not fall
    /// within the queried taxonomic root. Always `false` until the lineage check is
    /// implemented.
    pub taxonomic_outlier: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaxonGroup {
    Ingroup,
    Outgroup,
}

/// The result of one esearch over a single taxon. `query_results.json` is a
/// JSON array of these — one element per queried taxon, across both the ingroup
/// and outgroup TaxIDs given on the command line.
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult {
    pub taxid: u64,
    pub taxon_name: String,
    pub taxon_group: TaxonGroup,
    pub total_accessions: usize,
    pub accessions: Vec<Accession>,
}

/// Read a query TSV — an accession-keyed join table mapping each accession to
/// its TaxID, taxon name, and other query-time metadata — into a flat list of
/// [`Accession`]s. Columns are located by header name rather than position, so
/// a user can reorder or drop columns with `awk`/`cut` before handing the file
/// to `clean`; only `accession` is required, and any other missing column
/// falls back to a sensible default (0 / empty / ingroup / false).
pub fn read_query_tsv(path: &Path) -> Result<Vec<Accession>> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut lines = content.lines();

    let header = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("{}: empty query file (no header row)", path.display()))?;
    let index: HashMap<&str, usize> = header
        .split('\t')
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect();
    if !index.contains_key("accession") {
        bail!(
            "{}: query TSV missing required 'accession' column",
            path.display()
        );
    }

    let mut records = Vec::new();
    for (n, line) in lines.enumerate() {
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        // Closure over this row: fetch a column's value by name, or "" if the
        // column is absent from this file / short in this row.
        let get = |name: &str| {
            index
                .get(name)
                .and_then(|&i| cols.get(i))
                .copied()
                .unwrap_or("")
        };

        let line_no = n + 2; // +1 for the header row, +1 for 1-based counting
        let parse_u64 = |name: &str| -> Result<u64> {
            let v = get(name);
            if v.is_empty() {
                return Ok(0);
            }
            v.parse()
                .with_context(|| format!("{}:{line_no}: bad {name} value {v:?}", path.display()))
        };

        records.push(Accession {
            accession: get("accession").to_string(),
            taxon_name: get("taxon_name").to_string(),
            taxid: parse_u64("taxid")?,
            length: parse_u64("length")? as usize,
            gene_annotation: get("gene_annotation").to_string(),
            refseq: get("refseq") == "true",
            source_db: get("source_db").to_string(),
            query_taxid: parse_u64("query_taxid")?,
            taxon_group: match get("taxon_group") {
                "" | "ingroup" => TaxonGroup::Ingroup,
                "outgroup" => TaxonGroup::Outgroup,
                other => bail!(
                    "{}:{line_no}: unknown taxon_group {other:?} (expected ingroup/outgroup)",
                    path.display()
                ),
            },
            taxonomic_outlier: get("taxonomic_outlier") == "true",
        });
    }
    Ok(records)
}
