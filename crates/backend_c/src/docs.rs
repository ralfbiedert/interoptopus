use crate::Interop;
use crate::interop::{write_function_declaration, write_type_definition};
use interoptopus::Error;
use interoptopus::backend::IndentWriter;
use interoptopus::backend::sort_types_by_dependencies;
use interoptopus::indented;
use interoptopus::inventory::Bindings;
use interoptopus::lang::{Function, Type};

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
            Type::Composite(c) => c.meta(),
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

    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_types(w)?;
        self.write_functions(w)?;

        w.newline()?;

        Ok(())
    }
}

impl Bindings for Markdown<'_> {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_to(w)
    }
}
