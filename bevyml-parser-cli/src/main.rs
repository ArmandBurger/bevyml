use bevyml_parser::{
    BevymlParser,
    tree_sitter::{LanguageError, Node},
};
use clap::{Args, Parser, Subcommand};
use std::{
    fmt, io,
    path::{Path, PathBuf},
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
    /// Print the full tree dump as a debug view of the parser output.
    Debug(ParseArgs),
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
async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Command::Parse(args) => run_parse(args, false).await,
        Command::Debug(args) => run_parse(args, true).await,
    }
}

async fn run_parse(args: ParseArgs, debug_tree: bool) -> Result<(), CliError> {
    let path = resolve_path(&args.path).await?;

    println!("Parsing file: {}", path.display());
    let content = fs::read_to_string(&path)
        .await
        .map_err(|err| CliError::io(path.clone(), "read file content", err))?;

    let mut parser = BevymlParser::try_new()?;
    match parser.parse(&content) {
        Some(tree) => {
            let root = tree.root_node();
            println!(
                "Successfully parsed `{}`: root node `{}`",
                path.display(),
                root.kind()
            );
            if debug_tree {
                println!("Dumping tree nodes:");
                print_tree(root, &content, 0);
            }
        }
        None => {
            eprintln!(
                "Parser produced no tree for `{}`. The file might be empty or invalid.",
                path.display()
            );
        }
    }

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

fn print_tree<'tree>(node: Node<'tree>, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    let text = node
        .utf8_text(source.as_bytes())
        .unwrap_or("<invalid utf8>");
    println!(
        "{indent}{} [{}..{}]: {text}",
        node.kind(),
        node.start_byte(),
        node.end_byte()
    );

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_tree(child, source, depth + 1);
    }
}
