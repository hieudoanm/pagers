//! `pagers landing`: YAML → single-file landing page (landify port).
//! Thin clap layer over `pagers_landing`.

use clap::{Args, Subcommand};
use pagers_landing as landify;
use std::path::{Path, PathBuf};

#[derive(Subcommand)]
pub enum LandingCommand {
    /// Create a landify.yaml with placeholder content.
    New(NewArgs),
    /// Validate the schema of landify.yaml.
    Validate(FileArgs),
    /// Build index.html from landify.yaml.
    Build(BuildArgs),
    /// List the built-in theme presets.
    Themes,
    /// Serve the built files over HTTP.
    Preview(PreviewArgs),
}

#[derive(Args)]
pub struct FileArgs {
    /// YAML file to read (default: landify.yaml).
    #[arg(short, long, default_value = "landify.yaml")]
    pub file: String,
}

#[derive(Args)]
pub struct NewArgs {
    #[command(flatten)]
    pub file: FileArgs,
    /// Page type to scaffold.
    #[arg(short, long, default_value = "product")]
    pub page_type: String,
    /// Overwrite an existing file.
    #[arg(short = 'F', long)]
    pub force: bool,
}

#[derive(Args)]
pub struct BuildArgs {
    #[command(flatten)]
    pub file: FileArgs,
    /// Path to write the generated page.
    #[arg(short, long, default_value = "index.html")]
    pub output: String,
    /// Built-in theme preset overriding the YAML theme.
    #[arg(short, long)]
    pub theme: Option<String>,
}

#[derive(Args)]
pub struct PreviewArgs {
    /// Directory to serve.
    #[arg(short, long, default_value = ".")]
    pub dir: String,
    /// Address to bind.
    #[arg(short, long, default_value = "127.0.0.1")]
    pub bind: String,
    /// Port to listen on.
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

pub fn run(command: LandingCommand) -> std::process::ExitCode {
    match run_inner(command) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("pagers landing: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run_inner(command: LandingCommand) -> Result<(), String> {
    match command {
        LandingCommand::New(args) => new(args),
        LandingCommand::Validate(args) => validate(args),
        LandingCommand::Build(args) => build(args),
        LandingCommand::Themes => themes(),
        LandingCommand::Preview(args) => preview(args),
    }
}

fn new(args: NewArgs) -> Result<(), String> {
    let path = Path::new(&args.file.file);
    if path.exists() && !args.force {
        return Err(format!(
            "{} already exists (use --force to overwrite)",
            path.display()
        ));
    }
    landify::placeholder::write_placeholder(path, &args.page_type).map_err(|e| e.to_string())?;
    println!("Created {}", path.display());
    println!("Next: pagers landing validate");
    println!("then: pagers landing build");
    Ok(())
}

fn validate(args: FileArgs) -> Result<(), String> {
    let path = PathBuf::from(&args.file);
    landify::load::validate_file(&path).map_err(|e| e.to_string())?;
    println!("{} is valid", path.display());
    Ok(())
}

fn build(args: BuildArgs) -> Result<(), String> {
    let path = PathBuf::from(&args.file.file);
    let output = PathBuf::from(&args.output);
    let theme = args.theme.as_deref().unwrap_or("");
    let result = landify::build::build_file(&path, &output, theme).map_err(|e| e.to_string())?;
    println!("Built {} from {}", result.page.display(), path.display());
    if let Some(card) = result.card {
        let png = card.with_extension("png");
        println!("Built {}", card.display());
        println!(
            "Rasterize the card: rsvg-convert -o {} {}",
            png.display(),
            card.display()
        );
    }
    if !result.meta.is_empty() {
        println!("Built {} metadata file(s)", result.meta.len());
    }
    Ok(())
}

fn themes() -> Result<(), String> {
    let mut themes = landify::themes::themes();
    themes.sort_by(|a, b| a.name.cmp(b.name));
    for nt in themes {
        println!("{}\t{}", nt.name, nt.description);
    }
    Ok(())
}

fn preview(args: PreviewArgs) -> Result<(), String> {
    crate::serve::serve_dir(&args.dir, &args.bind, args.port)
}
