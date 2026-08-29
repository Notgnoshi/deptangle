use std::io::Read;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use deptangle_cli::stdio::{get_input_reader, get_output_writer};
use deptangle_graph::DepGraph;
use deptangle_io::emit::OutputFormat;
use deptangle_io::parse::InputFormat;
use deptangle_ops::transform;
use deptangle_ops::transform::shorten::ShortenArgs;
use deptangle_ops::transform::sub::{SubKey, Substitution};

/// Arguments for the `sub` subcommand.
#[derive(Debug, clap::Parser)]
struct SubArgs {
    /// Sed-style substitution: s/pattern/replacement/
    ///
    /// Uses Rust regex syntax: (...) for capture groups, $1/${name} in replacement.
    /// Supports alternate delimiters: s|...|...|, s#...#...#, etc.
    expr: String,

    /// Field to apply substitution to: id, node:NAME, or edge:NAME
    #[clap(long, default_value = "id")]
    key: String,
}

/// Arguments for the `merge` subcommand.
#[derive(Debug, clap::Parser)]
struct MergeArgs {
    /// Input files to merge (use '-' for stdin, at most once).
    /// The global --input/-i flag, if set, is included as an additional file.
    #[clap(required = true)]
    files: Vec<PathBuf>,
}

/// Structural transformations on dependency graphs.
///
/// Operations are performed via subcommands.
/// Chain operations by piping: deptransform ... | deptransform ...
#[derive(Debug, Parser)]
#[clap(version, verbatim_doc_comment)]
struct Args {
    /// Logging level
    #[clap(long, default_value_t = tracing::Level::INFO)]
    log_level: tracing::Level,

    /// Input file (stdin if '-' or omitted)
    #[clap(short, long, global = true)]
    input: Option<PathBuf>,

    /// Input format (auto-detected from extension/content if omitted)
    #[clap(short = 'I', long, global = true)]
    input_format: Option<InputFormat>,

    /// Output file (stdout if '-' or omitted)
    #[clap(short, long, global = true)]
    output: Option<PathBuf>,

    /// Output format (auto-detected from extension, defaults to DOT)
    #[clap(short = 'O', long, global = true)]
    output_format: Option<OutputFormat>,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Reverse the direction of all edges
    Reverse,
    /// Remove redundant edges via transitive reduction
    Simplify,
    /// Shorten node IDs and/or labels using path transforms
    Shorten(ShortenArgs),
    /// Apply sed-style regex substitution to graph fields
    ///
    /// Uses Rust regex syntax: (...) for capture groups, $1/${name} in replacement.
    /// When applied to node IDs, nodes that map to the same ID are merged.
    Sub(SubArgs),
    /// Merge multiple graphs into one
    ///
    /// Nodes are unioned by ID (later files overwrite on collision).
    /// Edges are deduplicated by (from, to); first label wins, attributes are merged.
    /// The global --input/-i flag, if set, is included as the first file.
    Merge(MergeArgs),
    /// Flatten subgraphs into a single top-level graph
    Flatten,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();
    deptangle_cli::init(args.log_level)?;

    // Normalize Some("-") to None so we can use the filepath for auto-detecting graph format below
    let is_stdio = |p: &PathBuf| p.as_os_str() == "-";
    let output_path = args.output.filter(|p| !is_stdio(p));
    let output_format =
        deptangle_io::emit::resolve_output_format(args.output_format, output_path.as_deref())?;

    let graph = match &args.command {
        // Merge can't handle the same input handling as the rest of the commands
        Command::Merge(merge_args) => {
            let mut files = Vec::new();
            if let Some(input) = &args.input {
                files.push(input);
            }
            files.extend(&merge_args.files);
            if files.len() < 2 {
                eyre::bail!("merge requires at least 2 input files");
            }
            let mut graphs = Vec::new();
            for file in &files {
                graphs.push(read_graph(Some(file), args.input_format)?);
            }
            transform::merge::merge(&graphs)
        }
        command => {
            let graph = read_graph(args.input.as_ref(), args.input_format)?;
            tracing::info!(
                "Parsed graph with {} nodes, {} edges, and {} subgraphs",
                graph.all_nodes().len(),
                graph.all_edges().len(),
                graph.subgraphs.len()
            );

            match command {
                Command::Reverse => transform::reverse::reverse(&graph),
                Command::Simplify => transform::simplify::simplify(&graph)?,
                Command::Shorten(shorten_args) => {
                    let transforms = transform::shorten::build_transforms(shorten_args);
                    transform::shorten::shorten(
                        &graph,
                        &shorten_args.separator,
                        shorten_args.key,
                        &transforms,
                    )
                }
                Command::Sub(sub_args) => {
                    let substitution = Substitution::parse(&sub_args.expr)?;
                    let key = SubKey::parse(&sub_args.key)?;
                    transform::sub::sub(&graph, &substitution, &key)
                }
                Command::Flatten => transform::flatten::flatten(&graph),
                Command::Merge(_) => unreachable!(),
            }
        }
    };

    let mut output = get_output_writer(&output_path)?;
    deptangle_io::emit::emit(output_format, &graph, &mut output)?;

    Ok(())
}

/// Read and parse a graph from a file path (or stdin if None / "-").
fn read_graph(path: Option<&PathBuf>, input_format: Option<InputFormat>) -> eyre::Result<DepGraph> {
    let is_stdio = |p: &PathBuf| p.as_os_str() == "-";
    let file_path: Option<PathBuf> = path.filter(|p| !is_stdio(p)).cloned();
    let mut reader = get_input_reader(&file_path)?;
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    let fmt = deptangle_io::parse::resolve_input_format(input_format, file_path.as_deref(), &text)?;
    deptangle_io::parse::parse(fmt, &text)
}
