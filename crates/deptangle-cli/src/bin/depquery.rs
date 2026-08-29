use std::io::{Read, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use deptangle_cli::stdio::get_input_reader;
use deptangle_io::parse::InputFormat;
use deptangle_ops::query::edges::EdgesArgs;
use deptangle_ops::query::nodes::NodesArgs;
use deptangle_ops::query::{OutputFields, metrics};

/// Query properties of dependency graphs.
///
/// Produces plain text output (not graph output) answering
/// "what's in this graph?" -- listing nodes, edges, and computing metrics.
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

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List nodes with optional filtering and sorting
    Nodes(NodesArgs),
    /// List edges with optional filtering and sorting
    Edges(EdgesArgs),
    /// Compute and display graph metrics
    Metrics,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();
    deptangle_cli::init(args.log_level)?;

    // Normalize Some("-") to None so we can use the filepath for auto-detecting graph format below
    let is_stdio = |p: &PathBuf| p.as_os_str() == "-";
    let input_path = args.input.filter(|p| !is_stdio(p));

    let mut input = get_input_reader(&input_path)?;
    let mut input_text = String::new();
    input.read_to_string(&mut input_text)?;

    let input_format = deptangle_io::parse::resolve_input_format(
        args.input_format,
        input_path.as_deref(),
        &input_text,
    )?;

    let graph = deptangle_io::parse::parse(input_format, &input_text)?;
    tracing::info!(
        "Parsed graph with {} nodes, {} edges, and {} subgraphs",
        graph.all_nodes().len(),
        graph.all_edges().len(),
        graph.subgraphs.len()
    );

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    match &args.command {
        Command::Nodes(nodes_args) => {
            let result = deptangle_ops::query::nodes::nodes(&graph, nodes_args)?;
            for (id, label, count) in &result {
                let field = match nodes_args.format {
                    OutputFields::Id => id.as_str(),
                    OutputFields::Label => label.as_str(),
                };
                match count {
                    Some(n) => writeln!(out, "{field}\t{n}")?,
                    None => writeln!(out, "{field}")?,
                }
            }
        }
        Command::Edges(edges_args) => {
            let result = deptangle_ops::query::edges::edges(&graph, edges_args)?;
            for (source, target, label) in &result {
                match label {
                    Some(l) if !l.is_empty() => writeln!(out, "{source}\t{target}\t{l}")?,
                    _ => writeln!(out, "{source}\t{target}")?,
                }
            }
        }
        Command::Metrics => {
            let m = metrics::metrics(&graph);
            write!(out, "{m}")?;
        }
    }

    Ok(())
}
