//! Types used by backends to produce pretty output.
use crate::Error;
use std::io::Write;

/// You might not realize how hard it can be to type exactly 4 spaces.
pub const FOUR_SPACES: &str = "    ";

/// In some places we can write for docs or for actual code generation.
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum WriteFor {
    Code,
    Docs,
}

/// Convenience helper to allow backends to write code with indentation.
pub struct IndentWriter<'a> {
    one_indent: String,
    current_level: usize,
    writer: &'a mut dyn Write,
}

impl<'a> IndentWriter<'a> {
    pub fn indent(&mut self) {
        self.current_level += 1;
    }

    pub fn unindent(&mut self) {
        if self.current_level == 0 {
            panic!("Tried to un-indent past start of line.")
        }

        self.current_level -= 1;
    }

    pub fn new(writer: &'a mut dyn Write) -> Self {
        Self {
            one_indent: FOUR_SPACES.to_string(),
            current_level: 0,
            writer,
        }
    }

    pub fn with_indent(writer: &'a mut dyn Write, one_indent: &str) -> Self {
        Self {
            one_indent: one_indent.to_string(),
            current_level: 0,
            writer,
        }
    }

    pub fn indented(&mut self, f: impl FnOnce(&mut dyn Write) -> std::io::Result<()>) -> Result<(), Error> {
        for _ in 0..self.current_level {
            write!(&mut self.writer, "{}", self.one_indent)?;
        }

        f(&mut self.writer)?;

        Ok(())
    }

    pub fn unindented(&mut self, f: impl FnOnce(&mut dyn Write) -> std::io::Result<()>) -> Result<(), Error> {
        f(&mut self.writer)?;

        Ok(())
    }

    pub fn indented_block(&mut self, block: Option<(&str, &str)>, f: impl FnOnce(&mut Self) -> Result<(), Error>) -> Result<(), Error> {
        if let Some(block) = block {
            self.indented(|w| writeln!(w, "{}", block.0))?;
        }
        self.indent();

        f(self)?;

        self.unindent();

        if let Some(block) = block {
            self.indented(|w| writeln!(w, "{}", block.1))?;
        }

        Ok(())
    }

    pub fn writer(&mut self) -> &mut dyn Write {
        &mut self.writer
    }

    pub fn newline(&mut self) -> Result<(), Error> {
        writeln!(&mut self.writer)?;
        Ok(())
    }
}

/// Writes a line of code, possibly with multiple indentations. Used in backends.
#[macro_export]
macro_rules! indented {
    ($w:expr, [ $($i:pat)+ ], $x:expr, $($param:expr),*) => {
        {
            $(
                let $i = ();
                $w.indent();
            )*
            let rval = $w.indented(|w| writeln!(w, $x, $($param),*));
            $(
                let $i = ();
                $w.unindent();
            )*
            rval
        }

    };

    ($w:expr, [ $($i:pat)+ ], $x:expr) => {
        {
            $(
                let $i = ();
                $w.indent();
            )*
            let rval = $w.indented(|w| writeln!(w, $x));
            $(
                let $i = ();
                $w.unindent();
            )*
            rval
        }

    };


    ($w:expr, $x:expr, $($param:expr),*) => {
        {
            $w.indented(|w| writeln!(w, $x, $($param),*))
        }
    };

    ($w:expr, $x:expr) => {
        {
            $w.indented(|w| writeln!(w, $x))
        }
    };
}


/// Writes an unindented line of code. Used in backends.
#[macro_export]
macro_rules! unindented {
    ($w:expr, $x:expr, $($param:expr),*) => {
        {
            $w.unindented(|w| writeln!(w, $x, $($param),*))
        }

    };

    ($w:expr, $x:expr, $($param:expr),*) => {
        {
            $w.unindented(|w| writeln!(w, $x, $($param),*))
        }
    };

    ($w:expr, $x:expr) => {
        {
            $w.unindented(|w| writeln!(w, $x))
        }
    };
}
