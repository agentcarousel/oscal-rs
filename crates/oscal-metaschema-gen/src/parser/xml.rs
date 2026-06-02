//! Event-based OSCAL Metaschema XML parser.
#![allow(clippy::pedantic, clippy::nursery)]
//!
//! Uses [`quick_xml::Reader`] in streaming mode to avoid loading large XML into
//! memory. Each call to [`parse_file`] returns the definitions declared in one
//! metaschema file; [`parse_metaschema_dir`] merges all files into a single
//! [`IrDb`].
//!
//! # Parsing strategy
//!
//! The OSCAL metaschema XML uses a recursive grammar:
//! - `<METASCHEMA>` → zero or more `<define-assembly>`, `<define-field>`, `<define-flag>`
//! - `<define-assembly>` → flags + `<model>` containing nested model items
//! - `<model>` → mixed `<assembly>`, `<field>`, `<define-assembly>`, `<define-field>`, `<choice>`
//!
//! We implement a recursive-descent parser over the XML event stream. The parser
//! skips elements it does not need (`<constraint>`, `<remarks>`, `<example>`, `<prop>`) by
//! consuming their sub-trees.

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};

use crate::ir::types::{
    AssemblyDef, Cardinality, DataType, FieldDef, FlagDef, FlagInstance, GroupingConfig, IrDb,
    JsonGrouping, ModelItem, ModelRef,
};

// ── Public entry points ───────────────────────────────────────────────────────

/// Parse every `*.xml` file in `dir` and merge their definitions into one [`IrDb`].
///
/// Files are processed in lexicographic order for deterministic output.
///
/// # Errors
///
/// Returns an error if any file cannot be read or contains malformed XML.
pub fn parse_metaschema_dir(dir: &Path) -> anyhow::Result<IrDb> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("cannot read metaschema dir: {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "xml"))
        .collect();
    paths.sort();

    let mut db = IrDb::default();
    let mut seen = HashSet::new();

    for path in &paths {
        parse_file_into(path, &mut db, &mut seen)?;
    }
    Ok(db)
}

/// Parse a single metaschema file and merge its definitions into `db`.
pub fn parse_file_into(
    path: &Path,
    db: &mut IrDb,
    seen: &mut HashSet<PathBuf>,
) -> anyhow::Result<()> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("cannot canonicalize {}", path.display()))?;
    if !seen.insert(canonical.clone()) {
        return Ok(());
    }

    let src = fs::read_to_string(&canonical)
        .with_context(|| format!("cannot read {}", canonical.display()))?;

    let mut p = MetaschemaParser::new(&src, path);
    p.parse_root(db)
        .with_context(|| format!("parse error in {}", path.display()))?;

    // Process any <import> hrefs discovered during parsing
    let imports = p.imports.clone();
    let base_dir = path.parent().unwrap_or(Path::new("."));
    for href in imports {
        let import_path = base_dir.join(&href);
        if import_path.exists() {
            parse_file_into(&import_path, db, seen)?;
        }
    }
    Ok(())
}

// ── Parser state ──────────────────────────────────────────────────────────────

struct MetaschemaParser<'src> {
    reader: Reader<&'src [u8]>,
    /// Short-name of this metaschema (from `<short-name>` element).
    short_name: String,
    /// `<import href="..."/>` hrefs to process after this file is done.
    imports: Vec<String>,
}

impl<'src> MetaschemaParser<'src> {
    fn new(src: &'src str, _path: &Path) -> Self {
        let mut reader = Reader::from_str(src);
        reader.config_mut().trim_text(true);
        Self {
            reader,
            short_name: String::new(),
            imports: Vec::new(),
        }
    }

    // ── Root parser ───────────────────────────────────────────────────────────

    fn parse_root(&mut self, db: &mut IrDb) -> anyhow::Result<()> {
        loop {
            match self.reader.read_event()? {
                Event::Start(e) => {
                    let name = local_name(&e);
                    match name.as_str() {
                        "METASCHEMA" => {}
                        "short-name" => {
                            self.short_name = self.read_text()?;
                        }
                        "define-assembly" => {
                            let def = self.parse_define_assembly(&e)?;
                            let source = def.source.clone();
                            let name = def.name.clone();
                            // Only record in sources on first encounter; subsequent
                            // re-definitions (from imported files) are deduplicated.
                            if !db.assemblies.contains_key(&name) {
                                db.assemblies.insert(name.clone(), def);
                                db.sources.entry(source).or_default().push(name);
                            }
                        }
                        "define-field" => {
                            let def = self.parse_define_field(&e)?;
                            let source = def.source.clone();
                            let name = def.name.clone();
                            if !db.fields.contains_key(&name) {
                                db.fields.insert(name.clone(), def);
                                db.sources.entry(source).or_default().push(name);
                            }
                        }
                        "define-flag" => {
                            let def = self.parse_define_flag_top(&e)?;
                            db.flags.entry(def.name.clone()).or_insert(def);
                        }
                        _ => self.skip_element(&e)?,
                    }
                }
                Event::Empty(e) => {
                    let name = local_name(&e);
                    if name == "import" {
                        if let Some(href) = attr_value(&e, "href") {
                            self.imports.push(href);
                        }
                    }
                }
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }
        Ok(())
    }

    // ── define-assembly ───────────────────────────────────────────────────────

    fn parse_define_assembly(&mut self, start: &BytesStart<'_>) -> anyhow::Result<AssemblyDef> {
        let name = require_attr(start, "name")?;
        let mut def = AssemblyDef {
            name,
            formal_name: String::new(),
            description: String::new(),
            root_name: None,
            flags: Vec::new(),
            model: Vec::new(),
            source: self.short_name.clone(),
        };

        loop {
            match self.reader.read_event()? {
                Event::Start(e) => match local_name(&e).as_str() {
                    "formal-name" => def.formal_name = self.read_text()?,
                    "description" => def.description = self.read_inner_text()?,
                    "root-name" => def.root_name = Some(self.read_text()?),
                    "define-flag" => def.flags.push(self.parse_define_flag_instance(&e)?),
                    "model" => def.model = self.parse_model()?,
                    _ => self.skip_element(&e)?,
                },
                Event::Empty(e) => {
                    if local_name(&e) == "flag" {
                        def.flags.push(self.parse_flag_ref(&e)?);
                    }
                }
                Event::End(e) if local_name_end(&e) == "define-assembly" => break,
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }
        Ok(def)
    }

    // ── define-field (top-level) ──────────────────────────────────────────────

    fn parse_define_field(&mut self, start: &BytesStart<'_>) -> anyhow::Result<FieldDef> {
        let name = require_attr(start, "name")?;
        let as_type = DataType::from_str(&attr_value(start, "as-type").unwrap_or_default());
        let mut def = FieldDef {
            name,
            formal_name: String::new(),
            description: String::new(),
            as_type,
            flags: Vec::new(),
            source: self.short_name.clone(),
        };

        loop {
            match self.reader.read_event()? {
                Event::Start(e) => match local_name(&e).as_str() {
                    "formal-name" => def.formal_name = self.read_text()?,
                    "description" => def.description = self.read_inner_text()?,
                    "define-flag" => def.flags.push(self.parse_define_flag_instance(&e)?),
                    _ => self.skip_element(&e)?,
                },
                Event::Empty(e) => {
                    if local_name(&e) == "flag" {
                        def.flags.push(self.parse_flag_ref(&e)?);
                    }
                }
                Event::End(e) if local_name_end(&e) == "define-field" => break,
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }
        Ok(def)
    }

    // ── define-flag (top-level) ───────────────────────────────────────────────

    fn parse_define_flag_top(&mut self, start: &BytesStart<'_>) -> anyhow::Result<FlagDef> {
        let name = require_attr(start, "name")?;
        let as_type = DataType::from_str(&attr_value(start, "as-type").unwrap_or_default());
        let required = attr_value(start, "required").map_or(false, |v| v == "yes");
        let mut def = FlagDef {
            name,
            formal_name: String::new(),
            description: String::new(),
            as_type,
            required,
            source: self.short_name.clone(),
        };

        loop {
            match self.reader.read_event()? {
                Event::Start(e) => match local_name(&e).as_str() {
                    "formal-name" => def.formal_name = self.read_text()?,
                    "description" => def.description = self.read_inner_text()?,
                    _ => self.skip_element(&e)?,
                },
                Event::End(e) if local_name_end(&e) == "define-flag" => break,
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }
        Ok(def)
    }

    // ── Inline flag definition (inside assembly/field) ────────────────────────

    fn parse_define_flag_instance(
        &mut self,
        start: &BytesStart<'_>,
    ) -> anyhow::Result<FlagInstance> {
        let name = require_attr(start, "name")?;
        let as_type = DataType::from_str(&attr_value(start, "as-type").unwrap_or_default());
        let required = attr_value(start, "required").map_or(false, |v| v == "yes");
        let default = attr_value(start, "default");

        let mut inst = FlagInstance {
            name,
            as_type,
            required,
            description: String::new(),
            default,
            allowed: Vec::new(),
        };

        loop {
            match self.reader.read_event()? {
                Event::Start(e) => match local_name(&e).as_str() {
                    "description" => inst.description = self.read_inner_text()?,
                    _ => self.skip_element(&e)?,
                },
                Event::End(e) if local_name_end(&e) == "define-flag" => break,
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }
        Ok(inst)
    }

    // ── <flag ref="..."/> ─────────────────────────────────────────────────────

    fn parse_flag_ref(&self, e: &BytesStart<'_>) -> anyhow::Result<FlagInstance> {
        let name = require_attr(e, "ref")?;
        let as_type = DataType::from_str(&attr_value(e, "as-type").unwrap_or_default());
        let required = attr_value(e, "required").map_or(false, |v| v == "yes");
        Ok(FlagInstance {
            name,
            as_type,
            required,
            description: String::new(),
            default: None,
            allowed: Vec::new(),
        })
    }

    // ── Model parser ──────────────────────────────────────────────────────────

    fn parse_model(&mut self) -> anyhow::Result<Vec<ModelItem>> {
        let mut items = Vec::new();
        loop {
            match self.reader.read_event()? {
                Event::Start(e) => match local_name(&e).as_str() {
                    "assembly" => items.push(ModelItem::AssemblyRef(
                        self.parse_model_ref(&e, "assembly")?,
                    )),
                    "field" => items.push(ModelItem::FieldRef(self.parse_model_ref(&e, "field")?)),
                    "define-assembly" => {
                        let min = attr_value(&e, "min-occurs");
                        let max = attr_value(&e, "max-occurs");
                        let card = Cardinality::from_min_max(min.as_deref(), max.as_deref());
                        items.push(ModelItem::InlineAssembly(
                            self.parse_define_assembly(&e)?,
                            card,
                        ));
                    }
                    "define-field" => {
                        let min = attr_value(&e, "min-occurs");
                        let max = attr_value(&e, "max-occurs");
                        let card = Cardinality::from_min_max(min.as_deref(), max.as_deref());
                        items.push(ModelItem::InlineField(self.parse_define_field(&e)?, card));
                    }
                    "choice" => items.push(ModelItem::Choice(self.parse_choice()?)),
                    _ => self.skip_element(&e)?,
                },
                Event::Empty(e) => match local_name(&e).as_str() {
                    "assembly" => {
                        items.push(ModelItem::AssemblyRef(self.parse_model_ref_empty(&e)?))
                    }
                    "field" => items.push(ModelItem::FieldRef(self.parse_model_ref_empty(&e)?)),
                    _ => {}
                },
                Event::End(e) if local_name_end(&e) == "model" => break,
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }
        Ok(items)
    }

    /// Parse a non-empty `<assembly>` or `<field>` model reference (has children
    /// like `<group-as>` or `<remarks>`).
    fn parse_model_ref(&mut self, start: &BytesStart<'_>, tag: &str) -> anyhow::Result<ModelRef> {
        let name = require_attr(start, "ref")?;
        let use_name_attr = attr_value(start, "use-name");
        let min = attr_value(start, "min-occurs");
        let max = attr_value(start, "max-occurs");
        let cardinality = Cardinality::from_min_max(min.as_deref(), max.as_deref());

        let mut mr = ModelRef {
            name,
            use_name: use_name_attr,
            cardinality,
            grouping: None,
            description: String::new(),
        };

        loop {
            match self.reader.read_event()? {
                Event::Empty(e) if local_name(&e) == "group-as" => {
                    mr.grouping = parse_group_as(&e);
                }
                Event::Start(e) => match local_name(&e).as_str() {
                    "use-name" => mr.use_name = Some(self.read_text()?),
                    _ => self.skip_element(&e)?,
                },
                Event::End(e) if local_name_end(&e) == tag => break,
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }
        Ok(mr)
    }

    /// Parse a self-closing `<assembly ref="..."/>` or `<field ref="..."/>`.
    fn parse_model_ref_empty(&self, e: &BytesStart<'_>) -> anyhow::Result<ModelRef> {
        let name = require_attr(e, "ref")?;
        let use_name = attr_value(e, "use-name");
        let min = attr_value(e, "min-occurs");
        let max = attr_value(e, "max-occurs");
        let cardinality = Cardinality::from_min_max(min.as_deref(), max.as_deref());
        Ok(ModelRef {
            name,
            use_name,
            cardinality,
            grouping: None,
            description: String::new(),
        })
    }

    // ── Choice block ──────────────────────────────────────────────────────────

    fn parse_choice(&mut self) -> anyhow::Result<Vec<ModelItem>> {
        let mut items = Vec::new();
        loop {
            match self.reader.read_event()? {
                Event::Start(e) => match local_name(&e).as_str() {
                    "assembly" => items.push(ModelItem::AssemblyRef(
                        self.parse_model_ref(&e, "assembly")?,
                    )),
                    "field" => items.push(ModelItem::FieldRef(self.parse_model_ref(&e, "field")?)),
                    _ => self.skip_element(&e)?,
                },
                Event::Empty(e) => match local_name(&e).as_str() {
                    "assembly" => {
                        items.push(ModelItem::AssemblyRef(self.parse_model_ref_empty(&e)?))
                    }
                    "field" => items.push(ModelItem::FieldRef(self.parse_model_ref_empty(&e)?)),
                    _ => {}
                },
                Event::End(e) if local_name_end(&e) == "choice" => break,
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }
        Ok(items)
    }

    // ── Text helpers ──────────────────────────────────────────────────────────

    /// Read text until the next `</tag>` close, stripping inner XML tags.
    fn read_text(&mut self) -> anyhow::Result<String> {
        let mut buf = String::new();
        loop {
            match self.reader.read_event()? {
                Event::Text(t) => buf.push_str(&t.unescape()?),
                Event::CData(c) => {
                    if let Ok(s) = std::str::from_utf8(&c) {
                        buf.push_str(s);
                    }
                }
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }
        Ok(buf.trim().to_owned())
    }

    /// Read inner text including nested element text (strips tags, keeps words).
    fn read_inner_text(&mut self) -> anyhow::Result<String> {
        let mut buf = String::new();
        let mut depth: u32 = 0;
        loop {
            match self.reader.read_event()? {
                Event::Text(t) => buf.push_str(&t.unescape()?),
                Event::CData(c) => {
                    if let Ok(s) = std::str::from_utf8(&c) {
                        buf.push_str(s);
                    }
                }
                Event::Start(_) => depth += 1,
                Event::End(_) => {
                    if depth == 0 {
                        break;
                    }
                    depth -= 1;
                }
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(buf.split_whitespace().collect::<Vec<_>>().join(" "))
    }

    /// Consume and discard an element and all its descendants.
    fn skip_element(&mut self, start: &BytesStart<'_>) -> anyhow::Result<()> {
        let tag = local_name(start);
        let mut depth: u32 = 1;
        loop {
            match self.reader.read_event()? {
                Event::Start(_) => depth += 1,
                Event::End(e) => {
                    depth -= 1;
                    if depth == 0 && local_name_end(&e) == tag {
                        break;
                    }
                    if depth == 0 {
                        break;
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(())
    }
}

// ── XML helpers ───────────────────────────────────────────────────────────────

fn local_name(e: &BytesStart<'_>) -> String {
    let raw = e.local_name();
    String::from_utf8_lossy(raw.as_ref()).into_owned()
}

fn local_name_end(e: &quick_xml::events::BytesEnd<'_>) -> String {
    let raw = e.local_name();
    String::from_utf8_lossy(raw.as_ref()).into_owned()
}

fn attr_value(e: &BytesStart<'_>, key: &str) -> Option<String> {
    e.attributes()
        .filter_map(|a| a.ok())
        .find(|a| {
            let k = a.key.local_name();
            k.as_ref() == key.as_bytes()
        })
        .and_then(|a| a.unescape_value().ok().map(|v| v.into_owned()))
}

fn require_attr(e: &BytesStart<'_>, key: &str) -> anyhow::Result<String> {
    attr_value(e, key).ok_or_else(|| {
        anyhow::anyhow!(
            "element <{}> missing required attribute '{key}'",
            String::from_utf8_lossy(e.local_name().as_ref())
        )
    })
}

fn parse_group_as(e: &BytesStart<'_>) -> Option<GroupingConfig> {
    let name = attr_value(e, "name")?;
    let in_json = attr_value(e, "in-json")
        .as_deref()
        .map(JsonGrouping::from_str)
        .unwrap_or(JsonGrouping::SingletonOrArray);
    Some(GroupingConfig { name, in_json })
}
