use clap::{Parser, Subcommand};

// Exit silently on broken pipe (e.g., piping to head/tail)
#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

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
    #[cfg(unix)]
    reset_sigpipe();

    let cli = Cli::parse();
    match cli.command {
        Commands::Getheaders(args) => cmd::getheaders::run(args),
        Commands::Concat(args) => cmd::concat::run(args),
    }
}
