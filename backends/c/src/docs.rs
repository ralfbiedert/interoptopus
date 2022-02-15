use crate::CWriter;
use interoptopus::indented;
use interoptopus::lang::c::{CType, Function};
use interoptopus::util::sort_types_by_dependencies;
use interoptopus::writer::IndentWriter;
use interoptopus::{Error, Inventory};
use std::fs::File;
use std::path::Path;

pub struct DocGenerator<W> {
    inventory: Inventory,
    c_writer: W,
}

impl<W: CWriter> DocGenerator<W> {
    pub fn new(inventory: Inventory, w: W) -> Self {
        Self { inventory, c_writer: w }
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Types "#)?;

        let mut known_function_pointers = vec![];

        for the_type in &sort_types_by_dependencies(self.inventory().ctypes().to_vec()) {
            self.write_type_definition(w, the_type, &mut known_function_pointers)?;
        }

        Ok(())
    }

    pub fn write_type_definition(&self, w: &mut IndentWriter, the_type: &CType, known_function_pointers: &mut Vec<String>) -> Result<(), Error> {
        let meta = match the_type {
            CType::Primitive(_) => return Ok(()),
            CType::Array(_) => return Ok(()),
            CType::Enum(e) => e.meta(),
            CType::Opaque(o) => o.meta(),
            CType::Composite(c) => c.meta(),
            CType::FnPointer(_) => return Ok(()),
            CType::ReadPointer(_) => return Ok(()),
            CType::ReadWritePointer(_) => return Ok(()),
            CType::Pattern(_) => return Ok(()),
        };

        w.newline()?;
        w.newline()?;

        indented!(w, r#"## {} "#, the_type.name_within_lib())?;
        w.newline()?;

        for line in meta.documentation().lines() {
            indented!(w, r#"{}"#, line.trim())?;
            w.newline()?;
        }

        indented!(w, r#"```"#)?;
        self.c_writer.write_type_definition(w, the_type, known_function_pointers)?;
        indented!(w, r#"```"#)?;

        Ok(())
    }

    pub fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Functions "#)?;

        for the_type in self.inventory().functions() {
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
            w.newline()?;
        }

        indented!(w, r#"```"#)?;
        self.c_writer.write_function_declaration(w, function, 80)?;
        indented!(w, r#"```"#)?;

        w.newline()?;

        Ok(())
    }

    pub fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_types(w)?;
        self.write_functions(w)?;

        w.newline()?;

        Ok(())
    }

    pub fn write_file<P: AsRef<Path>>(&self, file_name: P) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        let mut writer = IndentWriter::new(&mut file);

        self.write_to(&mut writer)
    }
}
