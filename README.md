# Cladekit

Cladekit is a lightweight, composable CLI phylogenetics toolkit. A single binary, Cladekit provides many subcommands that replace common chains of bash commands or a collection of individual programs in phylogenetic pipelines. Examples include header extraction, concatenation, and alignment quality control.

**Note:** Cladekit is under active development. Subcommands may change or be added as the project matures.

## Install

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo install --git https://github.com/andrewbudge/cladekit
```

This builds the binary and adds `cladekit` to your PATH.

To update to the latest version:

```bash
cargo install --force --git https://github.com/andrewbudge/cladekit
```

## Subcommands

### getheaders (ghd)

Extract headers from FASTA files.

**Example:**

```bash
$ cladekit getheaders testdata/test_good.fasta
Sequence1
Sequence2
Sequence1

$ cladekit getheaders -u testdata/test_good.fasta
Sequence1
Sequence2
```

### concat (liger)

Concatenate multiple gene alignments into a supermatrix. Unlike other tools, input files can live anywhere and globs are accepted.

**Benchmark vs FASconCAT-G:**

| Scale | Taxa x Genes | cladekit | FASconCAT-G | Speedup |
|---|---|---|---|---|
| Small | 100 x 20 | 19ms | 12s | **637x** |
| Medium | 300 x 50 | 146ms | 4 min | **1,646x** |
| Large | 1,000 x 30 | 225ms | 5.4 min | **1,438x** |
| Mega | 4,000 x 30 | 968ms | crash | - |

Concat runs in two modes:

- **Exact match (default):** headers must match exactly across files, like FASconCAT and AMAS.
- **Smart match (`-a alias.txt`):** pass an alias list — a file of clean output names (one per line, e.g. `Mus_musculus`) that get matched to messy input headers via case-insensitive substring search. Underscores in aliases match spaces in headers, so `Mus_musculus` finds `AB123.1 Mus musculus COX1 gene, partial cds`. Longer aliases match first to prevent partial collisions. The alias list doubles as a rename map — input headers stay messy, output gets clean names. Requires `-l` for a provenance TSV that records exactly which original header matched each alias.

Concat auto-detects DNA vs amino acid data per gene and adjusts missing characters and partition labels accordingly. FASTA output goes to stdout, partition boundaries to stderr in RAxML/IQ-TREE format by default. NEXUS bundles everything into one file.

**Exact match — clean headers:**

```bash
$ cladekit concat gene1.fasta gene2.fasta > supermatrix.fasta
DNA, gene1.fasta = 1-4
DNA, gene2.fasta = 5-8
```

**Smart match — messy headers with an alias list:**

```bash
$ cat alias.txt
Mus_musculus
Rattus_rattus
Xenopus_laevis

$ cladekit concat -a alias.txt -l prov.tsv gene1.fasta gene2.fasta > supermatrix.fasta
DNA, gene1.fasta = 1-4
DNA, gene2.fasta = 5-8

$ cat supermatrix.fasta
>Mus_musculus
ATCGATCG
>Rattus_rattus
ATCGNNNN
>Xenopus_laevis
NNNNATCG

$ cat prov.tsv
alias.txt	gene1.fasta	gene2.fasta
Mus_musculus	AB123.1 Mus musculus gene1 cds	XM456.1 Mus musculus gene2 cds
Rattus_rattus	AB124.1 Rattus rattus gene1 cds	MISSING
Xenopus_laevis	MISSING	XM789.1 Xenopus laevis gene2 cds
```

**NEXUS output:**

```bash
$ cladekit concat -a alias.txt -l prov.tsv -f nexus gene1.fasta gene2.fasta
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
- `-a, --alias` — alias list for smart matching (clean output names that map to messy input headers)
- `-l, --log` — provenance TSV output file (required with `-a`)
- `-f, --format` — output format: fasta (default), nexus (also accepts `n` or `nex`)
- `-m, --missing` — override missing data character (default: auto per data type — N for DNA, X for amino acid, ? for mixed)
- `-p, --partitions` — partition format: raxml (default, also used by IQ-TREE) or nexus

### stats

Get basic alignment statistics from FASTA files. Accepts multiple files via globs. Automatically detects DNA vs amino acid sequences.

**Columns:**
- **file** — filename (path stripped)
- **sequences** — number of sequences
- **length** — alignment length (NA if unaligned)
- **type** — `DNA` or `AA` (auto-detected, supports IUPAC ambiguity codes)
- **gc_pct** — GC content as a percentage of real bases (NA for amino acid data)
- **missing_pct** — percentage of gaps and unknown characters
- **variable** — sites with at least 2 different residues (excluding gaps/unknowns)
- **variable_pct** — variable sites as a percentage of alignment length
- **informative** — parsimony-informative sites (at least 2 residues each appearing 2+ times)
- **informative_pct** — informative sites as a percentage of alignment length

**Example:**

```bash
$ cladekit stats supermatrix.fasta proteins.fasta
file	sequences	length	type	gc_pct	missing_pct	variable	variable_pct	informative	informative_pct
supermatrix.fasta	3	8	DNA	50.0	33.3	0	0.0	0	0.0
proteins.fasta	4	20	AA	NA	0.0	3	15.0	2	10.0
```

## Planned Subcommands

- **coverage** — taxa coverage across gene files
- **scrub** — alignment outlier detection via pairwise p-distances
- **view** - in terminal alignment viewer
- **slice** - cut out and remove sections of an alignment (remove non-homologous seqs, extract homologous seqs)
- **convert** - convert between sequence data file types (FASTA, Nexus, Relaxed and Strict Phylip)

## Development Note

Cladekit is being built as both a real research tool and a vehicle for learning Rust. Development is assisted by Claude (Anthropic), which serves as a teaching aid and coding partner. The design, domain knowledge, and direction are the author's own.

## Author

Andrew Budge
