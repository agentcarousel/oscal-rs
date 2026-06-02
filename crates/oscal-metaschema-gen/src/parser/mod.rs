//! Parses NIST OSCAL Metaschema XML files into an [`IrDb`].
pub mod xml;

pub use xml::parse_metaschema_dir;
