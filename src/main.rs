use clap::{Parser, Subcommand};

// point to my command
mod cmd;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract headers from FASTA files
    #[command(visible_alias = "ghd")]
    Getheaders(cmd::getheaders::GetheadersArgs),

    /// Concatenate alignments into a supermatrix
    #[command(visible_alias = "liger")]
    Concat(cmd::concat::ConcatArgs),
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Getheaders(args) => cmd::getheaders::run(args),
        Commands::Concat(args) => cmd::concat::run(args),
    }
}
