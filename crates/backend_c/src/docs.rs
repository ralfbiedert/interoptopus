use crate::Interop;
use crate::interop::{write_function_declaration, write_type_definition};
use interoptopus::lang::util::sort_types_by_dependencies;
use interoptopus::lang::{Function, Type};
use interoptopus_backend_utils::{Error, IndentWriter, indented};
use std::fs::File;
use std::path::Path;

/// Produces C API documentation.
pub struct Markdown<'a> {
    interop: &'a Interop,
}

impl<'a> Markdown<'a> {
    #[must_use]
    pub const fn new(interop: &'a Interop) -> Self {
        Self { interop }
    }

    fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r"# Types ")?;

        let mut known_function_pointers = vec![];

        for the_type in &sort_types_by_dependencies(self.interop.inventory().c_types().to_vec()) {
            self.write_type_definition(w, the_type, &mut known_function_pointers)?;
        }

        Ok(())
    }

    fn write_type_definition(&self, w: &mut IndentWriter, the_type: &Type, known_function_pointers: &mut Vec<String>) -> Result<(), Error> {
        let meta = match the_type {
            Type::Primitive(_) => return Ok(()),
            Type::Array(_) => return Ok(()),
            Type::Enum(e) => e.meta(),
            Type::Opaque(o) => o.meta(),
            Type::ExternType(_) => return Ok(()),
            Type::Composite(c) => c.meta(),
            Type::Wire(_) => todo!(),
            Type::WirePayload(_) => todo!(),
            Type::FnPointer(_) => return Ok(()),
            Type::ReadPointer(_) => return Ok(()),
            Type::ReadWritePointer(_) => return Ok(()),
            Type::Pattern(_) => return Ok(()),
        };

        w.newline()?;
        w.newline()?;

        indented!(w, r"## {} ", the_type.name_within_lib())?;
        w.newline()?;

        for line in meta.docs().lines() {
            indented!(w, r"{}", line.trim())?;
            w.newline()?;
        }

        indented!(w, r"```")?;
        write_type_definition(self.interop, w, the_type, known_function_pointers)?;
        indented!(w, r"```")?;

        Ok(())
    }

    fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r"# Functions ")?;

        for the_type in self.interop.inventory().functions() {
            self.write_function(w, the_type)?;
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        indented!(w, r"## {} ", function.name())?;

        for line in function.meta().docs().lines() {
            if line.trim().starts_with('#') {
                write!(w.writer(), "##")?;
            }
            indented!(w, r"{}", line.trim())?;
            w.newline()?;
        }

        indented!(w, r"```")?;
        write_function_declaration(self.interop, w, function, 80)?;
        indented!(w, r"```")?;

        w.newline()?;

        Ok(())
    }

    /// Generates FFI binding code and writes them to the [`IndentWriter`].
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_types(w)?;
        self.write_functions(w)?;

        w.newline()?;

        Ok(())
    }

    /// Convenience method to write FFI bindings to the specified file with default indentation.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn write_file<P: AsRef<Path>>(&self, file_name: P) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        let mut writer = IndentWriter::new(&mut file);

        self.write_to(&mut writer)
    }

    /// Convenience method to write FFI bindings to a string.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn to_string(&self) -> Result<String, Error> {
        let mut vec = Vec::new();
        let mut writer = IndentWriter::new(&mut vec);
        self.write_to(&mut writer)?;
        Ok(String::from_utf8(vec)?)
    }
}
