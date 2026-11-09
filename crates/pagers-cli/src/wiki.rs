//! `pagers wiki`: convert a Wikipedia article URL to clean Markdown.

use std::path::Path;
use std::process::ExitCode;

use clap::{Args, Subcommand};
use pagers_wiki::{self, ConvertOptions};

#[derive(Subcommand)]
pub enum WikiCommand {
    /// Convert a Wikipedia article to Markdown.
    Convert(ConvertArgs),
    /// Show article metadata without converting.
    Info(InfoArgs),
}

#[derive(Args)]
pub struct ConvertArgs {
    /// Wikipedia article URL, e.g. https://en.wikipedia.org/wiki/Rust_(programming_language)
    pub url: String,
    /// Write output to this file, or into this directory using the
    /// article's suggested filename (e.g. rust_programming_language.md).
    #[arg(short, long)]
    pub output: Option<String>,
    /// Output format: markdown (default) or json.
    #[arg(long, default_value = "markdown")]
    pub format: String,
    /// Include references and citations.
    #[arg(long)]
    pub include_references: bool,
    /// Keep absolute wikipedia.org links instead of local .md filenames.
    #[arg(long)]
    pub wiki_links: bool,
    /// Suppress non-error output (for scripting).
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Args)]
pub struct InfoArgs {
    /// Wikipedia article URL.
    pub url: String,
    /// Output format: text (default) or json.
    #[arg(long, default_value = "text")]
    pub format: String,
}

pub fn run(command: WikiCommand) -> ExitCode {
    match command {
        WikiCommand::Convert(args) => convert(&args),
        WikiCommand::Info(args) => info(&args),
    }
}

fn convert(args: &ConvertArgs) -> ExitCode {
    let options = ConvertOptions {
        include_references: args.include_references,
        local_links: !args.wiki_links,
    };
    let article = match pagers_wiki::convert(&args.url, &options) {
        Ok(a) => a,
        Err(e) => return fail(&e.to_string()),
    };

    let output = match args.format.as_str() {
        "markdown" => article.markdown.clone(),
        "json" => match serde_json::to_string_pretty(&article.to_json()) {
            Ok(json) => json,
            Err(e) => return fail(&format!("serialize JSON: {e}")),
        },
        other => return fail(&format!("unknown format {other:?} (markdown or json)")),
    };

    if let Some(path) = &args.output {
        let path = resolve_output_path(path, &article.metadata.file_name());
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                return fail(&format!("create {}: {e}", parent.display()));
            }
        }
        if let Err(e) = std::fs::write(&path, format!("{output}\n")) {
            return fail(&format!("write {}: {e}", path.display()));
        }
        if !args.quiet {
            eprintln!("Wrote {} ({} words)", path.display(), article.word_count());
        }
    } else {
        print!("{output}");
        if !output.ends_with('\n') {
            println!();
        }
    }
    ExitCode::SUCCESS
}

fn info(args: &InfoArgs) -> ExitCode {
    match pagers_wiki::info(&args.url) {
        Ok(metadata) => {
            let text = match args.format.as_str() {
                "text" => metadata.info_text(),
                "json" => {
                    let value = serde_json::json!({
                        "title": metadata.title,
                        "url": metadata.url,
                        "language": metadata.language,
                        "revision_id": metadata.revision_id,
                        "revision_timestamp": metadata.revision_timestamp,
                    });
                    serde_json::to_string_pretty(&value).unwrap_or_default()
                }
                other => return fail(&format!("unknown format {other:?} (text or json)")),
            };
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => fail(&e.to_string()),
    }
}

/// `-o` may name a file or a directory; directories get the article's
/// suggested filename appended.
fn resolve_output_path(output: &str, file_name: &str) -> std::path::PathBuf {
    let path = Path::new(output);
    if path.is_dir() || output.ends_with('/') || output.ends_with('\\') {
        path.join(file_name)
    } else {
        path.to_path_buf()
    }
}

fn fail(message: &str) -> ExitCode {
    eprintln!("pagers wiki: {message}");
    ExitCode::FAILURE
}
