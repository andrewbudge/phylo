# Phylo

Phylo is a lightweight, composable CLI phylogenetics toolkit. A single binary, Phylo provides many subcommands that replace common chains of bash commands or a collection of individual programs in phylogenetic pipelines. Examples include header extraction, concatenation, and alignment quality control.

**Note:** Phylo is under active development. Subcommands may change or be added as the project matures.

## Development Note

Phylo is being built as both a real research tool and a vehicle for learning Rust. Development is assisted by Claude (Anthropic), which serves as a teaching aid and coding partner. The design, domain knowledge, and direction are the author's own.

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
>Sequence1
>Sequence2
>Sequence1

$ phylo getheaders -u testdata/test_good.fasta
>Sequence1
>Sequence2
```

## Planned Subcommands

- **concat** (liger) — supermatrix concatenation from multiple gene alignments
- **coverage** — taxa coverage across gene files
- **scrub** — alignment outlier detection via pairwise p-distances
- **stats** — basic statistics on FASTA files (length, number of sequences, etc)

## Author

Andrew Budge
