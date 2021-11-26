use crate::writer::WriteFor;
use crate::{DocConfig, PythonWriter};
use interoptopus::lang::c::{CType, Function};
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, sort_types_by_dependencies};
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, non_service_functions};
use interoptopus::{Error, Library};
use std::fs::File;
use std::path::Path;

pub struct DocGenerator<'a, W> {
    library: &'a Library,
    python_writer: &'a W,
    doc_config: DocConfig,
}

impl<'a, W: PythonWriter> DocGenerator<'a, W> {
    pub fn new(library: &'a Library, w: &'a W, config: DocConfig) -> Self {
        Self {
            library,
            python_writer: w,
            doc_config: config,
        }
    }

    pub fn library(&self) -> &Library {
        &self.library
    }

    pub fn config(&self) -> &DocConfig {
        &self.doc_config
    }

    pub fn write_toc(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"## API Overview"#)?;

        w.newline()?;
        indented!(w, r#"### Functions"#)?;
        indented!(w, r#"Freestanding callables inside the module."#)?;

        for the_type in non_service_functions(self.library) {
            let doc = the_type.meta().documentation().lines().first().cloned().unwrap_or_default();

            indented!(w, r#" - **[{}](#{})** - {}"#, the_type.name(), the_type.name(), doc)?;
        }

        w.newline()?;
        indented!(w, r#"### Classes"#)?;
        indented!(w, r#"Methods operating on common state."#)?;

        for pattern in self.library.patterns().iter().filter_map(|x| match x {
            LibraryPattern::Service(s) => Some(s),
            _ => None,
        }) {
            let prefix = longest_common_prefix(pattern.methods());
            let doc = pattern.the_type().meta().documentation().lines().first().cloned().unwrap_or_default();
            let name = pattern.the_type().rust_name();

            indented!(w, r#" - **[{}](#{})** - {}"#, name, name, doc)?;

            for x in pattern.constructors() {
                let target = format!("{}.{}", name, x.name().replace(&prefix, ""));
                let doc = x.meta().documentation().lines().first().cloned().unwrap_or_default();
                indented!(w, r#"     - **[{}](#{})** <sup>ctor</sup> - {}"#, x.name().replace(&prefix, ""), target, doc)?;
            }
            for x in pattern.methods() {
                let target = format!("{}.{}", name, x.name().replace(&prefix, ""));
                let doc = x.meta().documentation().lines().first().cloned().unwrap_or_default();
                indented!(w, r#"     - **[{}](#{})** - {}"#, x.name().replace(&prefix, ""), target, doc)?;
            }
        }

        w.newline()?;
        indented!(w, r#"### Enums"#)?;
        indented!(w, r#"Groups of related constants."#)?;

        for the_type in self.library.ctypes().iter().filter_map(|x| match x {
            CType::Enum(e) => Some(e),
            _ => None,
        }) {
            let doc = the_type.meta().documentation().lines().first().cloned().unwrap_or_default();
            indented!(w, r#" - **[{}](#{})** - {}"#, the_type.rust_name(), the_type.rust_name(), doc)?;
        }

        w.newline()?;
        indented!(w, r#"### Data Structs"#)?;
        indented!(w, r#"Composite data used by functions and methods."#)?;

        for the_type in self.library.ctypes().iter().filter_map(|x| match x {
            CType::Composite(c) => Some(c.clone()),
            CType::Pattern(p @ TypePattern::Option(_)) => Some(p.fallback_type().as_composite_type().cloned().unwrap()),
            CType::Pattern(p @ TypePattern::Slice(_)) => Some(p.fallback_type().as_composite_type().cloned().unwrap()),
            _ => None,
        }) {
            let doc = the_type.meta().documentation().lines().first().cloned().unwrap_or_default();
            indented!(w, r#" - **[{}](#{})** - {}"#, the_type.rust_name(), the_type.rust_name(), doc)?;
        }

        Ok(())
    }

    pub fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Types "#)?;

        for the_type in self.library().ctypes() {
            let composite = match the_type {
                CType::Composite(e) => e.clone(),
                CType::Pattern(p @ TypePattern::Option(_)) => p.fallback_type().as_composite_type().cloned().unwrap(),
                CType::Pattern(p @ TypePattern::Slice(_)) => p.fallback_type().as_composite_type().cloned().unwrap(),
                _ => continue,
            };

            let meta = composite.meta();

            w.newline()?;
            w.newline()?;

            indented!(w, r#" ### <a name="{}">**{}**</a>"#, the_type.name_within_lib(), the_type.name_within_lib())?;
            w.newline()?;

            for line in meta.documentation().lines() {
                indented!(w, r#"{}"#, line.trim())?;
            }

            indented!(w, r#"#### Fields "#)?;
            for f in composite.fields() {
                let doc = f.documentation().lines().join("\n");
                indented!(w, r#"- **{}** - {} "#, f.name(), doc)?;
            }

            indented!(w, r#"#### Definition "#)?;
            indented!(w, r#"```python"#)?;
            self.python_writer.write_struct(w, &composite, WriteFor::Docs);
            indented!(w, r#"```"#)?;

            w.newline()?;
        }

        Ok(())
    }

    pub fn write_enums(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Enums "#)?;

        for the_type in self.library().ctypes() {
            let the_enum = match the_type {
                CType::Enum(e) => e,
                _ => continue,
            };

            let meta = the_enum.meta();

            w.newline()?;
            w.newline()?;

            indented!(w, r#" ### <a name="{}">**{}**</a>"#, the_type.name_within_lib(), the_type.name_within_lib())?;
            w.newline()?;

            for line in meta.documentation().lines() {
                indented!(w, r#"{}"#, line.trim())?;
            }
            w.newline()?;

            indented!(w, r#"#### Variants "#)?;
            for v in the_enum.variants() {
                let doc = v.documentation().lines().join("\n");
                indented!(w, r#"- **{}** - {} "#, v.name(), doc)?;
            }

            indented!(w, r#"#### Definition "#)?;
            indented!(w, r#"```python"#)?;
            self.python_writer.write_enum(w, the_enum, WriteFor::Docs)?;
            indented!(w, r#"```"#)?;
        }

        Ok(())
    }

    pub fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Functions"#)?;

        for the_type in non_service_functions(self.library()) {
            self.write_function(w, the_type)?;
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        indented!(w, r#"## {} "#, function.name())?;

        for line in function.meta().documentation().lines() {
            if line.trim().starts_with('#') {
                write!(w.writer(), "##")?;
            }
            indented!(w, r#"{}"#, line.trim())?;
        }

        indented!(w, r#"#### Definition "#)?;
        indented!(w, r#"```python"#)?;
        self.python_writer.write_function(w, function, WriteFor::Docs)?;
        indented!(w, r#"```"#)?;

        w.newline()?;

        Ok(())
    }

    pub fn write_services(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Services"#)?;

        for pattern in self.library.patterns().iter().filter_map(|x| match x {
            LibraryPattern::Service(s) => Some(s),
            _ => None,
        }) {
            let prefix = longest_common_prefix(pattern.methods());
            let doc = pattern.the_type().meta().documentation().lines();
            let class_name = pattern.the_type().rust_name();

            indented!(w, r#" ## <a name="{}">**{}**</a> <sup>ctor</sup>"#, class_name, class_name)?;

            for line in doc {
                indented!(w, r#"{}"#, line)?;
            }

            for x in pattern.constructors() {
                let fname = x.name().replace(&prefix, "");
                let target = format!("{}.{}", class_name, fname);
                indented!(w, r#" ### <a name="{}">**{}**</a> <sup>ctor</sup>"#, target, fname)?;

                let doc = x.meta().documentation().lines();
                for line in doc {
                    indented!(w, r#"{}"#, line)?;
                }

                indented!(w, r#"#### Definition "#)?;
                indented!(w, r#"```python"#)?;
                indented!(w, r#"class {}:"#, class_name)?;
                w.newline()?;
                self.python_writer.write_pattern_class_ctor(w, pattern, x, WriteFor::Docs)?;
                indented!(w, [_ _], r#"..."#)?;
                indented!(w, r#"```"#)?;
            }

            for x in pattern.methods() {
                let fname = x.name().replace(&prefix, "");
                let target = format!("{}.{}", class_name, fname);
                indented!(w, r#" ### <a name="{}">**{}**</a>"#, target, fname)?;

                for line in doc {
                    indented!(w, r#"{}"#, line)?;
                }

                indented!(w, r#"#### Definition "#)?;
                indented!(w, r#"```python"#)?;
                indented!(w, r#"class {}:"#, class_name)?;
                w.newline()?;
                self.python_writer.write_pattern_class_method(w, pattern, x, WriteFor::Docs)?;
                indented!(w, [_ _], r#"..."#)?;
                indented!(w, r#"```"#)?;
            }

            w.newline()?;
        }

        Ok(())
    }

    pub fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        writeln!(w.writer(), "{}", self.config().header)?;

        self.write_toc(w)?;
        self.write_types(w)?;
        self.write_enums(w)?;
        self.write_functions(w)?;
        self.write_services(w)?;

        w.newline()?;

        Ok(())
    }

    pub fn write_file<P: AsRef<Path>>(&self, file_name: P) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        let mut writer = IndentWriter::new(&mut file);

        self.write_to(&mut writer)
    }
}
