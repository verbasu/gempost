use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Clone)]
pub struct Init {
    /// The directory to initialize
    pub directory: Option<PathBuf>,
}

#[derive(Args, Clone)]
pub struct Build {
    /// The path of the gempost config file
    #[arg(long, value_name = "PATH", default_value = "./gempost.yaml")]
    pub config_file: PathBuf,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Initialize a new gempost project
    Init(Init),

    /// Build your capsule
    Build(Build),
}
