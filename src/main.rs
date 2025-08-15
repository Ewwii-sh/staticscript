mod grammar;
mod opts;
mod transpiler;
mod formatter;

use transpiler::transpile_code;
use formatter::format_code;
use opts::AppArgs;

use log::Level;
use clap::Parser as ClapParser;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args = AppArgs::parse();

    set_debug_levels(args.debug);

    if let Some(files) = args.transpile.as_deref() {
        for file_path in files {
            let dsl_code = fs::read_to_string(file_path)
                .expect("Encoutered an unknown error while reading files");
                
            let mut transpiled_code = transpile_code(&dsl_code)
                .unwrap_or_else(|e| {
                    log::error!("Error transpiling code:\n{}", e);
                    std::process::exit(1);
                });

            // argument to format code
            if args.format {
                let formatted_code = format_code(transpiled_code)
                    .unwrap_or_else(|e| {
                        log::error!("Error formatting code:\n{}", e);
                        std::process::exit(1);
                    });
                transpiled_code = formatted_code;
            }

            let file_name = Path::new(file_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();

            let out_path: PathBuf = if let Some(out_dir) = args.out.as_deref() {
                Path::new(out_dir).join(format!("{}.rhai", file_name))
            } else {
                PathBuf::from(format!("{}.rhai", file_name))
            };

            // outputting the transpiled file
            fs::write(&out_path, transpiled_code)
                .expect("Failed to write transpiled file");

            println!("Wrote {}", out_path.display());
        }
    } else {
        println!("No command provided!");
    }
}

fn set_debug_levels(debug_mode: bool) {
    let mut builder = env_logger::Builder::from_default_env();

    if debug_mode {
        builder
            .filter_level(log::LevelFilter::Debug)
            .format_timestamp_secs()
            .format_module_path(true)
            .format_level(true);
    } else {
        builder.format(|buf, record| {
            use std::io::Write;

            match record.level() {
                Level::Warn => writeln!(buf, "[WARN] {}", record.args()),
                Level::Error => writeln!(buf, "[ERROR] {}", record.args()),
                _ => writeln!(buf, "{}", record.args()),
            }
        })
        .filter_level(log::LevelFilter::Info);
    }

    builder.init();
}
