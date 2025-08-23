use clap::Parser;

/// StaticScript: A fast transpiler that allows writing simple widgets for ewwii.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct AppArgs {
    /// Transpile files from stpl to rhai.
    #[clap(short, long)]
    pub transpile: Option<Vec<String>>,

    /// Path to output the transpiled files.
    #[clap(short, long)]
    pub out: Option<String>,

    /// Format transpiled files for redability.
    #[clap(short, long)]
    pub format: bool,

    /// Show debug logs.
    #[arg(long)]
    pub debug: bool,
}
