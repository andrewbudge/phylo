# Phylo

Phylo is a lightweight, composable CLI phylogenetics toolkit. A single binary, Phylo provides many subcommands that replace common chains of bash commands or a collection of individual programs in phylogenetic pipelines. Examples include header extraction, concatenation, and alignment quality control.

**Note:** Phylo is under active development. Subcommands may change or be added as the project matures.

## Install

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo install --git https://github.com/andrewbudge/phylo
```

This builds the binary and adds `phylo` to your PATH.

To update to the latest version:

```bash
cargo install --force --git https://github.com/andrewbudge/phylo
```

## Subcommands

### getheaders (ghd)

Extract headers from FASTA files.

**Example:**

```bash
$ phylo getheaders testdata/test_good.fasta
Sequence1
Sequence2
Sequence1

$ phylo getheaders -u testdata/test_good.fasta
Sequence1
Sequence2
```

### concat (liger)

Concatenate multiple gene alignments into a supermatrix. Uses smart substring matching to link taxa names to FASTA headers, so input files don't need clean headers. Longer taxon names match first to prevent partial collisions.

Supermatrix FASTA is written to stdout, partition boundaries to stderr, and a provenance TSV to a required log file. The provenance TSV records exactly which original FASTA header matched each taxon for each gene — an audit trail for verifying the automated matching.

**Example:**

```bash
$ cat taxa.txt
Mus_musculus
Rattus_rattus
Xenopus_laevis

$ phylo concat -l prov.tsv taxa.txt gene1.fasta gene2.fasta > supermatrix.fasta
gene1.fasta = 1-4
gene2.fasta = 5-8

$ cat supermatrix.fasta
>Mus_musculus
ATCGATCG
>Rattus_rattus
ATCGNNNN
>Xenopus_laevis
NNNNATCG

$ cat prov.tsv
taxa.txt        gene1.fasta                       gene2.fasta
Mus_musculus    AB123.1 Mus musculus gene1 cds    XM456.1 Mus musculus gene2 cds
Rattus_rattus   AB124.1 Rattus rattus gene1 cds   MISSING
Xenopus_laevis  MISSING                           XM789.1 Xenopus laevis gene2 cds
```

NEXUS output bundles the alignment and partitions into one file:

```bash
$ phylo concat -l prov.tsv -f nexus taxa.txt gene1.fasta gene2.fasta
#NEXUS
BEGIN DATA;
  DIMENSIONS NTAX=3 NCHAR=8;
  FORMAT DATATYPE=DNA MISSING=N GAP=-;
  MATRIX
  Mus_musculus    ATCGATCG
  Rattus_rattus   ATCGNNNN
  Xenopus_laevis  NNNNATCG
;
END;
BEGIN SETS;
  CHARSET gene1.fasta = 1-4;
  CHARSET gene2.fasta = 5-8;
END;
```

**Flags:**
- `-l, --log` — provenance TSV output file (required)
- `-f, --format` — output format: fasta (default), nexus (also accepts `n` or `nex`)
- `-m, --missing` — character for missing data (default: N)

## Planned Subcommands

- **coverage** — taxa coverage across gene files
- **scrub** — alignment outlier detection via pairwise p-distances
- **stats** — basic statistics on FASTA files (length, number of sequences, etc)
- **view** - in terminal alignment viewer
- **slice** - cut out and remove sections of an alignment (remove non-homologous seqs, extract homologous seqs)
- **convert** - convert between sequence data file types (FASTA, Nexus, Relaxed and Strict Phylip)

## Development Note

Phylo is being built as both a real research tool and a vehicle for learning Rust. Development is assisted by Claude (Anthropic), which serves as a teaching aid and coding partner. The design, domain knowledge, and direction are the author's own.

## Author

Andrew Budge
