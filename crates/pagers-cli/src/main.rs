//! pagers — write Markdown, run one command, get a website.
//!
//! ```text
//! pagers build [content]     Markdown folder → static site in ./dist
//! pagers init [dir]          Scaffold a starter project
//! pagers graph [content]     Write the page-link graph (html/json/dot)
//! pagers serve [content]     Build, then serve ./dist locally
//! ```

use pagers_build as build;
mod init;
mod serve;
mod wiki;

use std::process::ExitCode;
use std::time::Instant;

use clap::{Parser, Subcommand};

const DEFAULT_CONTENT: &str = "docs";
const DEFAULT_OUTPUT: &str = "dist";

#[derive(Parser)]
#[command(
    name = "pagers",
    version,
    about = "Turn a directory of Markdown into a static website"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Convert Markdown into a static website.
    Build {
        /// Content directory (default: ./docs).
        content: Option<String>,
        /// Output directory (default: ./dist).
        #[arg(short, long, default_value = DEFAULT_OUTPUT)]
        output: String,
    },
    /// Scaffold a starter project.
    Init {
        /// Directory to create (default: .).
        dir: Option<String>,
    },
    /// Build a link graph of the Markdown files.
    Graph {
        /// Content directory (default: ./docs).
        content: Option<String>,
        /// Output path or directory (default: graph.html in cwd).
        #[arg(short, long)]
        output: Option<String>,
        /// Output format: html, json or dot. Default writes all three.
        #[arg(long)]
        format: Option<String>,
        /// Graph page title.
        #[arg(long, default_value = "File Graph")]
        title: String,
    },
    /// Convert a Wikipedia article URL to Markdown.
    Wiki {
        #[command(subcommand)]
        command: wiki::WikiCommand,
    },
    /// Build, then serve the site locally.
    Serve {
        /// Content directory (default: ./docs).
        content: Option<String>,
        /// Output directory (default: ./dist).
        #[arg(short, long, default_value = DEFAULT_OUTPUT)]
        output: String,
        /// Port to listen on.
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

fn main() -> ExitCode {
    // `--version`/`-V` print the crate version plus the build timestamp —
    // intercepted before clap so the stamp is always shown.
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!(
            "pagers {} (build {})",
            env!("CARGO_PKG_VERSION"),
            env!("PAGERS_BUILD_TIME")
        );
        return ExitCode::SUCCESS;
    }
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Build { content, output } => {
            let content = content.unwrap_or_else(|| DEFAULT_CONTENT.to_string());
            run_build(&content, &output)
        }
        Command::Init { dir } => init::init(&dir.unwrap_or_else(|| ".".to_string())),
        Command::Graph {
            content,
            output,
            format,
            title,
        } => {
            let content = content.unwrap_or_else(|| DEFAULT_CONTENT.to_string());
            run_graph(&content, output.as_deref(), format.as_deref(), &title)
        }
        Command::Wiki { command } => return wiki::run(command),
        Command::Serve {
            content,
            output,
            port,
        } => {
            let content = content.unwrap_or_else(|| DEFAULT_CONTENT.to_string());
            run_serve(&content, &output, port)
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("pagers: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run_build(content: &str, output: &str) -> Result<(), String> {
    let started = Instant::now();
    let stats = build::build(content, output)?;
    println!(
        "Built {} pages and {} assets into {} in {:?}",
        stats.pages,
        stats.assets,
        output,
        started.elapsed()
    );
    Ok(())
}

fn run_serve(content: &str, output: &str, port: u16) -> Result<(), String> {
    let stats = build::build(content, output)?;
    println!(
        "Built {} pages and {} assets into {}",
        stats.pages, stats.assets, output
    );
    let base_path = pagers_site::config::load_for_dir(content)
        .map(|c| c.base_path)
        .unwrap_or_default();
    serve::serve(output, port, &base_path)
}

fn run_graph(
    content: &str,
    output: Option<&str>,
    format: Option<&str>,
    title: &str,
) -> Result<(), String> {
    let mut graph = pagers_graph::scan(content)?;
    if let Ok(config) = pagers_site::config::load_for_dir(content) {
        pagers_graph::apply_base_path(&mut graph, &config);
    }
    let totals = format!(
        "{} files, {} links, {} broken links",
        graph.stats.nodes, graph.stats.edges, graph.stats.broken
    );

    // With no explicit --format, write every representation; an explicit
    // format writes only that one.
    let formats: Vec<&str> = match format {
        Some(f @ ("html" | "json" | "dot")) => vec![f],
        Some(other) => return Err(format!("unknown format {other:?} (html, json or dot)")),
        None => vec!["html", "json", "dot"],
    };
    let multi = formats.len() > 1;

    let mut wrote = Vec::new();
    for f in &formats {
        let path = graph_output_path(output, f, multi);
        if let Some(parent) = std::path::Path::new(&path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("create {}: {e}", parent.display()))?;
            }
        }
        match *f {
            "json" => {
                let json = serde_json::to_string_pretty(&pagers_graph::to_json(&graph))
                    .map_err(|e| e.to_string())?;
                std::fs::write(&path, format!("{json}\n"))
                    .map_err(|e| format!("write {path}: {e}"))?;
            }
            "dot" => {
                std::fs::write(&path, pagers_graph::to_dot(&graph))
                    .map_err(|e| format!("write {path}: {e}"))?;
            }
            _ => {
                let html = pagers_graph::render_html(&graph, title, "light");
                std::fs::write(&path, html).map_err(|e| format!("write {path}: {e}"))?;
            }
        }
        wrote.push(path);
    }
    println!("Wrote {} ({totals})", wrote.join(", "));
    Ok(())
}

/// Resolves the output path for one format. A directory (existing or
/// trailing-slash) gets `graph.<ext>` appended; a single-format file path is
/// used as given; multi-format file paths share the given stem.
fn graph_output_path(output: Option<&str>, format: &str, multi: bool) -> String {
    let default = format!("graph.{format}");
    let Some(output) = output else {
        return default;
    };
    let is_dir = output.ends_with('/') || std::path::Path::new(output).is_dir();
    if is_dir {
        return format!("{output}{default}");
    }
    if !multi {
        return output.to_string();
    }
    let stem = output
        .trim_end_matches(".html")
        .trim_end_matches(".json")
        .trim_end_matches(".dot");
    format!("{stem}.{format}")
}
