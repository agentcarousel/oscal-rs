//! Writes generated [`TokenStream`]s to formatted `.rs` files.
//!
//! Every file goes through [`prettyplease::unparse`] before being written so
//! the output is indistinguishable from `rustfmt`-formatted code.

use std::{
    fmt::Write as FmtWrite,
    fs,
    io::Write as IoWrite,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Context;
use proc_macro2::TokenStream;

/// Write one module's token stream to `{output_dir}/{module_name}.rs`.
///
/// The token stream is first formatted via `prettyplease` and a file-level
/// warning suppression banner is prepended.
///
/// # Errors
///
/// Returns an error if `syn` fails to parse the token stream or if the file
/// cannot be written.
pub fn write_module(
    output_dir: &Path,
    module_name: &str,
    tokens: TokenStream,
) -> anyhow::Result<()> {
    let syntax: syn::File = syn::parse2(tokens)
        .with_context(|| format!("syn parse failed for module `{module_name}`"))?;

    let prettyprinted = prettyplease::unparse(&syntax);

    let banner = "// @generated — do not edit by hand.\n\
                  // Run `cargo run -p oscal-metaschema-gen` to regenerate.\n\n";

    let content = format!("{banner}{prettyprinted}");

    // Pass through `rustfmt` so the output is byte-for-byte identical to
    // what `cargo fmt` would produce, keeping `cargo fmt --check` green.
    let formatted = rustfmt_string(&content).unwrap_or(content);

    let path = output_dir.join(format!("{module_name}.rs"));
    fs::write(&path, formatted).with_context(|| format!("failed to write {}", path.display()))?;

    Ok(())
}

/// Format a Rust source string through `rustfmt --edition 2021`.
/// Returns the formatted string, or `None` if rustfmt is unavailable or fails.
fn rustfmt_string(src: &str) -> Option<String> {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2021"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    child.stdin.as_mut()?.write_all(src.as_bytes()).ok()?;

    let output = child.wait_with_output().ok()?;
    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}

/// Write `generated/mod.rs` declaring all generated modules and re-exporting
/// every public item via `pub use`.
///
/// # Errors
///
/// Returns an error if the file cannot be written.
pub fn write_mod_rs(output_dir: &Path, modules: &[String]) -> anyhow::Result<()> {
    let mut buf = String::new();
    writeln!(buf, "// @generated — do not edit by hand.")?;
    writeln!(
        buf,
        "// Run `cargo run -p oscal-metaschema-gen` to regenerate."
    )?;
    writeln!(buf)?;

    for m in modules {
        writeln!(buf, "pub mod {m};")?;
    }
    writeln!(buf)?;

    // Re-export everything from every module for ergonomic access at crate root.
    for m in modules {
        writeln!(buf, "pub use {m}::*;")?;
    }

    let path = output_dir.join("mod.rs");
    fs::write(&path, buf).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}
