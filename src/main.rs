mod body_formatter;
mod commands;
mod config;
mod defaults;
mod gh;
mod git;
mod github_templates;
mod types;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "gripe", about = "Submit structured feedback as GitHub issues")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Submit feedback as a GitHub issue
    Submit {
        /// JSON string with field values
        #[arg(long)]
        json: Option<String>,
        /// Read JSON from stdin
        #[arg(long)]
        stdin: bool,
        /// Preview without creating an issue
        #[arg(long)]
        dry_run: bool,
        /// Output result as JSON
        #[arg(long)]
        output_json: bool,
        /// Target repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
        /// Field values as key=value pairs
        #[arg(trailing_var_arg = true)]
        fields: Vec<String>,
    },
    /// Create a gripe.yaml in the current directory
    Init {
        /// Overwrite existing gripe.yaml
        #[arg(long)]
        force: bool,
    },
    /// Show the resolved schema
    Schema {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Submit {
            json,
            stdin,
            dry_run,
            output_json,
            repo,
            fields,
        } => commands::submit::run(json, stdin, dry_run, output_json, repo, fields),
        Commands::Init { force } => commands::init::run(force),
        Commands::Schema { json } => commands::schema::run(json),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
