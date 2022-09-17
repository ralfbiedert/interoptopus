use crate::config::DocConfig;
use crate::converter::{CSharpTypeConverter, FunctionNameFlavor};
use crate::CSharpWriter;
use interoptopus::lang::c::{CType, CompositeType, Function};
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, non_service_functions};
use interoptopus::{Error, Inventory};
use std::fs::File;
use std::path::Path;

pub struct DocGenerator<'a, W> {
    inventory: &'a Inventory,
    csharp_writer: &'a W,
    doc_config: DocConfig,
}

impl<'a, W: CSharpWriter> DocGenerator<'a, W> {
    pub fn new(inventory: &'a Inventory, w: &'a W, config: DocConfig) -> Self {
        Self {
            inventory,
            csharp_writer: w,
            doc_config: config,
        }
    }

    pub fn inventory(&self) -> &Inventory {
        self.inventory
    }

    pub fn config(&self) -> &DocConfig {
        &self.doc_config
    }

    pub fn write_toc(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"## API Overview"#)?;

        w.newline()?;
        indented!(w, r#"### Functions"#)?;
        indented!(w, r#"Freestanding callables inside the module."#)?;

        for the_type in non_service_functions(self.inventory) {
            let doc = the_type.meta().documentation().lines().first().cloned().unwrap_or_default();

            indented!(w, r#" - **[{}](#{})** - {}"#, the_type.name(), the_type.name(), doc)?;
        }

        w.newline()?;
        indented!(w, r#"### Classes"#)?;
        indented!(w, r#"Methods operating on common state."#)?;

        for pattern in self.inventory.patterns().iter().map(|x| match x {
            LibraryPattern::Service(s) => s,
        }) {
            let prefix = pattern.common_prefix();
            let doc = pattern.the_type().meta().documentation().lines().first().cloned().unwrap_or_default();
            let name = pattern.the_type().rust_name();

            indented!(w, r#" - **[{}](#{})** - {}"#, name, name, doc)?;

            for x in pattern.constructors() {
                let func_name = self
                    .csharp_writer
                    .converter()
                    .function_name_to_csharp_name(x, FunctionNameFlavor::CSharpMethodNameWithoutClass(&prefix));
                let target = format!("{}.{}", name, func_name);
                let doc = x.meta().documentation().lines().first().cloned().unwrap_or_default();
                indented!(w, r#"     - **[{}](#{})** <sup>**ctor**</sup> - {}"#, func_name, target, doc)?;
            }
            for x in pattern.methods() {
                let func_name = self
                    .csharp_writer
                    .converter()
                    .function_name_to_csharp_name(x, FunctionNameFlavor::CSharpMethodNameWithoutClass(&prefix));
                let target = format!("{}.{}", name, func_name);
                let doc = x.meta().documentation().lines().first().cloned().unwrap_or_default();
                indented!(w, r#"     - **[{}](#{})** - {}"#, func_name, target, doc)?;
            }
        }

        w.newline()?;
        indented!(w, r#"### Enums"#)?;
        indented!(w, r#"Groups of related constants."#)?;

        for the_type in self.inventory.ctypes().iter().filter_map(|x| match x {
            CType::Enum(e) => Some(e),
            _ => None,
        }) {
            let doc = the_type.meta().documentation().lines().first().cloned().unwrap_or_default();
            indented!(w, r#" - **[{}](#{})** - {}"#, the_type.rust_name(), the_type.rust_name(), doc)?;
        }

        w.newline()?;
        indented!(w, r#"### Data Structs"#)?;
        indented!(w, r#"Composite data used by functions and methods."#)?;

        for the_type in self.inventory.ctypes().iter() {
            match the_type {
                CType::Composite(c) => {
                    let doc = c.meta().documentation().lines().first().cloned().unwrap_or_default();
                    indented!(w, r#" - **[{}](#{})** - {}"#, c.rust_name(), c.rust_name(), doc)?;
                }
                CType::Pattern(p @ TypePattern::Option(_)) => {
                    let c = p.fallback_type().as_composite_type().cloned().unwrap();
                    indented!(w, r#" - **[{}](#{})** - A boolean flag and optionally data."#, c.rust_name(), c.rust_name())?;
                }
                CType::Pattern(p @ TypePattern::Slice(_)) => {
                    let c = p.fallback_type().as_composite_type().cloned().unwrap();
                    indented!(w, r#" - **[{}](#{})** - A pointer and length of un-owned elements."#, c.rust_name(), c.rust_name())?;
                }
                _ => continue,
            }
        }

        w.newline()?;
        indented!(w, r#"---"#)?;
        w.newline()?;

        Ok(())
    }

    pub fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Types "#)?;

        for the_type in self.inventory().ctypes() {
            match the_type {
                CType::Composite(e) => self.write_composite(w, e)?,
                CType::Pattern(p @ TypePattern::Option(_)) => self.write_composite(w, p.fallback_type().as_composite_type().unwrap())?,
                CType::Pattern(p @ TypePattern::Slice(_)) => self.write_composite(w, p.fallback_type().as_composite_type().unwrap())?,
                _ => continue,
            };

            w.newline()?;
            indented!(w, r#"---"#)?;
            w.newline()?;
        }

        Ok(())
    }

    pub fn write_composite(&self, w: &mut IndentWriter, composite: &CompositeType) -> Result<(), Error> {
        let meta = composite.meta();

        w.newline()?;
        w.newline()?;

        indented!(w, r#" ### <a name="{}">**{}**</a>"#, composite.rust_name(), composite.rust_name())?;
        w.newline()?;

        for line in meta.documentation().lines() {
            indented!(w, r#"{}"#, line.trim())?;
        }

        w.newline()?;

        indented!(w, r#"#### Fields "#)?;
        for f in composite.fields() {
            let doc = f.documentation().lines().join("\n");
            let name = self
                .csharp_writer
                .converter()
                .field_name_to_csharp_name(f, self.csharp_writer.config().rename_symbols);
            indented!(w, r#"- **{}** - {} "#, name, doc)?;
        }

        indented!(w, r#"#### Definition "#)?;
        indented!(w, r#"```csharp"#)?;
        self.csharp_writer.write_type_definition_composite_body(w, composite, WriteFor::Docs)?;
        indented!(w, r#"```"#)?;

        Ok(())
    }

    pub fn write_enums(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Enums "#)?;

        for the_type in self.inventory().ctypes() {
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
            indented!(w, r#"```csharp"#)?;
            self.csharp_writer.write_type_definition_enum(w, the_enum, WriteFor::Docs)?;
            indented!(w, r#"```"#)?;
            w.newline()?;
            indented!(w, r#"---"#)?;
            w.newline()?;
        }

        Ok(())
    }

    pub fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Functions"#)?;

        for the_type in non_service_functions(self.inventory()) {
            self.write_function(w, the_type)?;
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        indented!(w, r#"### <a name="{}">**{}**</a>"#, function.name(), function.name())?;

        for line in function.meta().documentation().lines() {
            if line.trim().starts_with('#') {
                write!(w.writer(), "##")?;
            }
            indented!(w, r#"{}"#, line.trim())?;
        }

        indented!(w, r#"#### Definition "#)?;
        indented!(w, r#"```csharp"#)?;
        self.csharp_writer.write_function(w, function, WriteFor::Docs)?;
        indented!(w, r#"```"#)?;
        w.newline()?;
        indented!(w, r#"---"#)?;
        w.newline()?;

        Ok(())
    }

    pub fn write_services(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Classes"#)?;

        for pattern in self.inventory.patterns().iter().map(|x| match x {
            LibraryPattern::Service(s) => s,
        }) {
            let prefix = pattern.common_prefix();
            let doc = pattern.the_type().meta().documentation().lines();
            let class_name = pattern.the_type().rust_name();

            indented!(w, r#"## <a name="{}">**{}**</a>"#, class_name, class_name)?;

            for line in doc {
                let line = line.replace(" # ", " #### ");
                let line = line.replace(" ## ", " ##### ");
                let line = line.replace(" ### ", " ###### ");

                indented!(w, r#"{}"#, line)?;
            }

            for x in pattern.constructors() {
                let fname = self
                    .csharp_writer
                    .converter()
                    .function_name_to_csharp_name(x, FunctionNameFlavor::CSharpMethodNameWithoutClass(&prefix));
                let target = format!("{}", fname);
                indented!(w, r#"### <a name="{}">**{}**</a> <sup>ctor</sup>"#, target, target)?;

                let doc = x.meta().documentation().lines();
                for line in doc {
                    let line = line.replace(" # ", " #### ");
                    let line = line.replace(" ## ", " ##### ");
                    let line = line.replace(" ### ", " ###### ");
                    indented!(w, r#"{}"#, line)?;
                }
                w.newline()?;
                indented!(w, r#"#### Definition "#)?;
                indented!(w, r#"```csharp"#)?;
                self.csharp_writer
                    .write_pattern_service_method(w, pattern, x, class_name, &fname, true, true, WriteFor::Docs)?;
                indented!(w, r#"```"#)?;
                w.newline()?;
                indented!(w, r#"---"#)?;
                w.newline()?;
            }

            for x in pattern.methods() {
                let fname = self
                    .csharp_writer
                    .converter()
                    .function_name_to_csharp_name(x, FunctionNameFlavor::CSharpMethodNameWithoutClass(&prefix));
                let target = format!("{}", fname);

                let rval = match x.signature().rval() {
                    CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
                    CType::Pattern(TypePattern::AsciiPointer) => "string".to_string(),
                    _ => self.csharp_writer.converter().to_typespecifier_in_rval(x.signature().rval()),
                };

                indented!(w, r#"### <a name="{}">**{}**</a>"#, target, target)?;

                let doc = x.meta().documentation().lines();
                for line in doc {
                    let line = line.replace(" # ", " #### ");
                    let line = line.replace(" ## ", " ##### ");
                    let line = line.replace(" ### ", " ###### ");
                    indented!(w, r#"{}"#, line)?;
                }

                w.newline()?;
                indented!(w, r#"#### Definition "#)?;
                indented!(w, r#"```csharp"#)?;
                indented!(w, r#"{} class {} {{"#, self.csharp_writer.config().visibility_types.to_access_modifier(), class_name)?;
                w.indent();
                self.csharp_writer
                    .write_pattern_service_method(w, pattern, x, &rval, &fname, false, false, WriteFor::Docs)?;
                for overload in self.csharp_writer.overloads() {
                    overload.write_service_method_overload(w, self.csharp_writer.helper(), pattern, x, &fname, WriteFor::Docs)?;
                }
                w.unindent();
                indented!(w, r#"}}"#)?;
                indented!(w, r#"```"#)?;
                w.newline()?;
                indented!(w, r#"---"#)?;
                w.newline()?;
            }

            w.newline()?;
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
