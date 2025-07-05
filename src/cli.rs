use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Converts a Markdown file to a Typst file.
    Md2typ(Md2typArgs),
    
    // 未来可以添加 Typ2md 子命令
    // Typ2md(Typ2mdArgs),
}

#[derive(Parser, Debug)]
pub struct Md2typArgs {
    /// The path to the input Markdown file.
    pub input: PathBuf,

    /// The path to the output Typst file.
    /// If not provided, output will be generated next to the input file
    /// with a .typ extension.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}