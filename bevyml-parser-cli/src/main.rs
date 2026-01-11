use bevyml_parser::{BevymlParser, inode::BevyNodeTree, tree_sitter::LanguageError};
use clap::{Args, Parser, Subcommand};
use std::{
    fmt, io,
    path::{Path, PathBuf},
    time::Instant,
};
use tokio::fs;

/// Simple CLI for parsing a single Bevyml file.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Parse a Bevyml file with the tree-sitter grammar",
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Parse a file and report the root node kind.
    Parse(ParseArgs),
}

#[derive(Args, Debug)]
struct ParseArgs {
    /// File system path to the Bevyml file you want to inspect.
    #[arg(value_name = "PATH", default_value = ".")]
    path: PathBuf,
}

#[derive(Debug)]
enum CliError {
    Io {
        path: PathBuf,
        action: &'static str,
        source: io::Error,
    },
    Language(LanguageError),
    NotAFile(PathBuf),
}

impl CliError {
    fn io(path: PathBuf, action: &'static str, source: io::Error) -> Self {
        Self::Io {
            path,
            action,
            source,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Io {
                path,
                action,
                source,
            } => {
                write!(f, "failed to {action} `{}`: {source}", path.display())
            }
            CliError::Language(err) => write!(f, "language initialization failed: {err}"),
            CliError::NotAFile(path) => write!(f, "`{}` is not a readable file", path.display()),
        }
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CliError::Io { source, .. } => Some(source),
            CliError::Language(err) => Some(err),
            CliError::NotAFile(_) => None,
        }
    }
}

impl From<LanguageError> for CliError {
    fn from(err: LanguageError) -> Self {
        Self::Language(err)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Parse(args) => run_parse(args).await,
    }
}

async fn run_parse(args: ParseArgs) -> anyhow::Result<()> {
    let path = resolve_path(&args.path).await?;

    println!("Parsing file: {}", path.display());
    let content = fs::read_to_string(&path)
        .await
        .map_err(|err| CliError::io(path.clone(), "read file content", err))?;

    let mut parser = BevymlParser::try_new()?;
    let parse_start = Instant::now();
    let tree = parser.parse(&content)?;
    let parse_duration = parse_start.elapsed();

    let trees: Vec<BevyNodeTree> = tree.into();
    dbg!(trees);
    println!("Parsing took {:.3}us", parse_duration.as_micros());

    Ok(())
}

async fn resolve_path(path: &Path) -> Result<PathBuf, CliError> {
    let metadata = fs::metadata(path)
        .await
        .map_err(|err| CliError::io(path.to_owned(), "inspect path", err))?;

    if !metadata.is_file() {
        return Err(CliError::NotAFile(path.to_owned()));
    }

    fs::canonicalize(path)
        .await
        .map_err(|err| CliError::io(path.to_owned(), "canonicalize path", err))
}
