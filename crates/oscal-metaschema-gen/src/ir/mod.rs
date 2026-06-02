//! Intermediate representation of OSCAL Metaschema definitions.
//!
//! The parser produces an [`types::IrDb`] by reading XML; the codegen modules
//! consume it to emit Rust source. Nothing in this module performs I/O or token
//! generation.
pub mod types;
