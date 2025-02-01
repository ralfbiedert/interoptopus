use crate::interop::functions::write_function;
use crate::interop::patterns::{write_pattern_class_ctor, write_pattern_class_method};
use crate::interop::types::{write_enum, write_struct};
use crate::Interop;
use interoptopus::lang::c::{CType, CompositeType, Function};
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, non_service_functions};
use interoptopus::{Bindings, Error};

/// Configures Python documentation generation.
#[derive(Clone, Debug, Default)]
pub struct MarkdownConfig {
    /// Header to append to the generated documentation.
    pub header: String,
}

/// Produces Python API documentation.
pub struct Markdown<'a> {
    interop: &'a Interop,
    doc_config: MarkdownConfig,
}

impl<'a> Markdown<'a> {
    #[must_use]
    pub const fn new(interop: &'a Interop, doc_config: MarkdownConfig) -> Self {
        Self { interop, doc_config }
    }

    fn write_toc(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r"## API Overview")?;

        w.newline()?;
        indented!(w, r"### Functions")?;
        indented!(w, r"Freestanding callables inside the module.")?;

        for the_type in non_service_functions(&self.interop.inventory) {
            let doc = the_type.meta().documentation().lines().first().cloned().unwrap_or_default();

            indented!(w, r" - **[{}](#{})** - {}", the_type.name(), the_type.name(), doc)?;
        }

        w.newline()?;
        indented!(w, r"### Classes")?;
        indented!(w, r"Methods operating on common state.")?;

        for pattern in self.interop.inventory.patterns().iter().map(|x| match x {
            LibraryPattern::Service(s) => s,
            _ => panic!("Pattern not explicitly handled"),
        }) {
            let prefix = pattern.common_prefix();
            let doc = pattern.the_type().meta().documentation().lines().first().cloned().unwrap_or_default();
            let name = pattern.the_type().rust_name();

            indented!(w, r" - **[{}](#{})** - {}", name, name, doc)?;

            for x in pattern.constructors() {
                let target = format!("{}.{}", name, x.name().replace(&prefix, ""));
                let doc = x.meta().documentation().lines().first().cloned().unwrap_or_default();
                indented!(w, r"     - **[{}](#{})** <sup>**ctor**</sup> - {}", x.name().replace(&prefix, ""), target, doc)?;
            }
            for x in pattern.methods() {
                let target = format!("{}.{}", name, x.name().replace(&prefix, ""));
                let doc = x.meta().documentation().lines().first().cloned().unwrap_or_default();
                indented!(w, r"     - **[{}](#{})** - {}", x.name().replace(&prefix, ""), target, doc)?;
            }
        }

        w.newline()?;
        indented!(w, r"### Enums")?;
        indented!(w, r"Groups of related constants.")?;

        for the_type in self.interop.inventory.ctypes().iter().filter_map(|x| match x {
            CType::Enum(e) => Some(e),
            _ => None,
        }) {
            let doc = the_type.meta().documentation().lines().first().cloned().unwrap_or_default();
            indented!(w, r" - **[{}](#{})** - {}", the_type.rust_name(), the_type.rust_name(), doc)?;
        }

        w.newline()?;
        indented!(w, r"### Data Structs")?;
        indented!(w, r"Composite data used by functions and methods.")?;

        for the_type in self.interop.inventory.ctypes() {
            match the_type {
                CType::Composite(c) => {
                    let doc = c.meta().documentation().lines().first().cloned().unwrap_or_default();
                    indented!(w, r" - **[{}](#{})** - {}", c.rust_name(), c.rust_name(), doc)?;
                }
                CType::Pattern(p @ TypePattern::Option(_)) => {
                    let c = p.fallback_type().as_composite_type().cloned().unwrap();
                    indented!(w, r" - **[{}](#{})** - A boolean flag and optionally data.", c.rust_name(), c.rust_name())?;
                }
                CType::Pattern(p @ TypePattern::Slice(_)) => {
                    let c = p.fallback_type().as_composite_type().cloned().unwrap();
                    indented!(w, r" - **[{}](#{})** - A pointer and length of un-owned elements.", c.rust_name(), c.rust_name())?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r"# Types ")?;

        for the_type in self.interop.inventory.ctypes() {
            match the_type {
                CType::Composite(e) => self.write_composite(w, e)?,
                CType::Pattern(p @ TypePattern::Option(_)) => self.write_composite(w, p.fallback_type().as_composite_type().unwrap())?,
                CType::Pattern(p @ TypePattern::Slice(_)) => self.write_composite(w, p.fallback_type().as_composite_type().unwrap())?,
                _ => continue,
            }

            w.newline()?;
            indented!(w, r"---")?;
            w.newline()?;
        }

        Ok(())
    }

    fn write_composite(&self, w: &mut IndentWriter, composite: &CompositeType) -> Result<(), Error> {
        let meta = composite.meta();

        w.newline()?;
        w.newline()?;

        indented!(w, r#" ### <a name="{}">**{}**</a>"#, composite.rust_name(), composite.rust_name())?;
        w.newline()?;

        for line in meta.documentation().lines() {
            indented!(w, r"{}", line.trim())?;
        }

        w.newline()?;

        indented!(w, r"#### Fields ")?;
        for f in composite.fields() {
            let doc = f.documentation().lines().join("\n");
            indented!(w, r"- **{}** - {} ", f.name(), doc)?;
        }

        indented!(w, r"#### Definition ")?;
        indented!(w, r"```python")?;
        write_struct(self.interop, w, composite, WriteFor::Docs)?;
        indented!(w, r"```")?;

        Ok(())
    }

    fn write_enums(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r"# Enums ")?;

        for the_type in self.interop.inventory.ctypes() {
            let CType::Enum(the_enum) = the_type else { continue };
            let meta = the_enum.meta();

            w.newline()?;
            w.newline()?;

            indented!(w, r#" ### <a name="{}">**{}**</a>"#, the_type.name_within_lib(), the_type.name_within_lib())?;
            w.newline()?;

            for line in meta.documentation().lines() {
                indented!(w, r"{}", line.trim())?;
            }
            w.newline()?;

            indented!(w, r"#### Variants ")?;
            for v in the_enum.variants() {
                let doc = v.documentation().lines().join("\n");
                indented!(w, r"- **{}** - {} ", v.name(), doc)?;
            }

            indented!(w, r"#### Definition ")?;
            indented!(w, r"```python")?;
            write_enum(self.interop, w, the_enum, WriteFor::Docs)?;
            indented!(w, r"```")?;
            w.newline()?;
            indented!(w, r"---")?;
            w.newline()?;
        }

        Ok(())
    }

    fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r"# Functions")?;

        for the_type in non_service_functions(&self.interop.inventory) {
            self.write_function(w, the_type)?;
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        indented!(w, r"## {} ", function.name())?;

        for line in function.meta().documentation().lines() {
            if line.trim().starts_with('#') {
                write!(w.writer(), "##")?;
            }
            indented!(w, r"{}", line.trim())?;
        }

        indented!(w, r"#### Definition ")?;
        indented!(w, r"```python")?;
        write_function(self.interop, w, function, WriteFor::Docs)?;
        indented!(w, r"```")?;
        w.newline()?;
        indented!(w, r"---")?;
        w.newline()?;

        Ok(())
    }

    fn write_services(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r"# Services")?;

        for pattern in self.interop.inventory.patterns().iter().map(|x| match x {
            LibraryPattern::Service(s) => s,
            _ => panic!("Pattern not explicitly handled"),
        }) {
            let prefix = pattern.common_prefix();
            let doc = pattern.the_type().meta().documentation().lines();
            let class_name = pattern.the_type().rust_name();

            indented!(w, r#"## <a name="{}">**{}**</a> <sup>ctor</sup>"#, class_name, class_name)?;

            for line in doc {
                let line = line.replace(" # ", " #### ");
                let line = line.replace(" ## ", " ##### ");
                let line = line.replace(" ### ", " ###### ");

                indented!(w, r"{}", line)?;
            }

            for x in pattern.constructors() {
                let fname = x.name().replace(&prefix, "");
                let target = format!("{class_name}.{fname}");
                indented!(w, r#"### <a name="{}">**{}**</a> <sup>ctor</sup>"#, target, fname)?;

                let doc = x.meta().documentation().lines();
                for line in doc {
                    let line = line.replace(" # ", " #### ");
                    let line = line.replace(" ## ", " ##### ");
                    let line = line.replace(" ### ", " ###### ");
                    indented!(w, r"{}", line)?;
                }

                w.newline()?;
                indented!(w, r"#### Definition ")?;
                indented!(w, r"```python")?;
                indented!(w, r"class {}:", class_name)?;
                w.newline()?;
                write_pattern_class_ctor(self.interop, w, pattern, x, WriteFor::Docs)?;
                indented!(w, [()()], r"...")?;
                indented!(w, r"```")?;
                w.newline()?;
                indented!(w, r"---")?;
                w.newline()?;
            }

            for x in pattern.methods() {
                let fname = x.name().replace(&prefix, "");
                let target = format!("{class_name}.{fname}");
                indented!(w, r#"### <a name="{}">**{}**</a>"#, target, fname)?;

                let doc = x.meta().documentation().lines();
                for line in doc {
                    let line = line.replace(" # ", " #### ");
                    let line = line.replace(" ## ", " ##### ");
                    let line = line.replace(" ### ", " ###### ");
                    indented!(w, r"{}", line)?;
                }

                w.newline()?;
                indented!(w, r"#### Definition ")?;
                indented!(w, r"```python")?;
                indented!(w, r"class {}:", class_name)?;
                w.newline()?;
                write_pattern_class_method(self.interop, w, pattern, x, WriteFor::Docs)?;
                indented!(w, [()()], r"...")?;
                indented!(w, r"```")?;
                w.newline()?;
                indented!(w, r"---")?;
                w.newline()?;
            }

            w.newline()?;
            w.newline()?;
        }

        Ok(())
    }

    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        writeln!(w.writer(), "{}", self.doc_config.header)?;

        self.write_toc(w)?;
        self.write_types(w)?;
        self.write_enums(w)?;
        self.write_functions(w)?;
        self.write_services(w)?;

        w.newline()?;

        Ok(())
    }
}

impl Bindings for Markdown<'_> {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_to(w)
    }
}
