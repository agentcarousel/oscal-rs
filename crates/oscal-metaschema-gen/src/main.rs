//! `oscal-metaschema-gen` — generates `crates/oscal/src/generated/*.rs` from the
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
//! NIST OSCAL Metaschema XML files in `metaschemas/`.
//!
//! # Usage
//!
//! ```bash
//! cargo run -p oscal-metaschema-gen
//! # or with explicit paths:
//! cargo run -p oscal-metaschema-gen -- \
//!   --metaschema-dir ./metaschemas \
//!   --output-dir ./crates/oscal/src/generated
//! ```

mod codegen;
mod ir;
mod parser;

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "oscal-metaschema-gen",
    about = "Generate Rust types from NIST OSCAL Metaschema XML files"
)]
struct Cli {
    /// Directory containing NIST OSCAL metaschema XML files.
    #[arg(long, default_value = "./metaschemas")]
    metaschema_dir: PathBuf,

    /// Output directory for generated `.rs` source files.
    #[arg(long, default_value = "./crates/oscal/src/generated")]
    output_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    eprintln!(
        "oscal-metaschema-gen: reading metaschemas from {}",
        cli.metaschema_dir.display()
    );

    let db = parser::parse_metaschema_dir(&cli.metaschema_dir)
        .context("failed to parse metaschema directory")?;

    eprintln!(
        "  parsed {} assemblies, {} fields, {} flags across {} source files",
        db.assemblies.len(),
        db.fields.len(),
        db.flags.len(),
        db.sources.len()
    );

    let modules = codegen::generate_all(&db).context("code generation failed")?;

    std::fs::create_dir_all(&cli.output_dir)
        .with_context(|| format!("cannot create output dir: {}", cli.output_dir.display()))?;

    let mut module_names: Vec<String> = Vec::with_capacity(modules.len());
    for (name, tokens) in &modules {
        codegen::emit::write_module(&cli.output_dir, name, tokens.clone())
            .with_context(|| format!("failed to write module {name}"))?;
        module_names.push(name.clone());
    }

    codegen::emit::write_mod_rs(&cli.output_dir, &module_names)
        .context("failed to write generated/mod.rs")?;

    eprintln!(
        "oscal-metaschema-gen: wrote {} modules to {}",
        module_names.len(),
        cli.output_dir.display()
    );

    Ok(())
}
