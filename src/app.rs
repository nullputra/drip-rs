use clap::{Command, IntoApp, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    /// Execute scripts according to extension
    Exec {
        #[clap(value_parser)]
        file_path: String,
        #[clap(value_parser)]
        args: Vec<String>,
    },
    /// Running scripts independent of extension
    Misc {
        #[clap(value_parser)]
        subsubcmd: String,
        #[clap(value_parser)]
        args: Vec<String>,
    },
}

pub fn command() -> Commands {
    let cli = Cli::parse();
    cli.command
}

#[allow(dead_code)]
fn build_app() -> Command<'static> {
    Cli::into_app()
}

#[test]
fn verify_app() {
    build_app().debug_assert()
}
