mod constants;
mod defines;
mod docs;
mod functions;
mod imports;
mod types;

pub use functions::write_function_declaration;
pub use types::write_type_definition;

use crate::interop::constants::write_constants;
use crate::interop::defines::{write_custom_defines, write_ifdefcpp, write_ifndef};
use crate::interop::docs::write_file_header_comments;
use crate::interop::functions::write_functions;
use crate::interop::imports::write_imports;
use crate::interop::types::write_type_definitions;
use derive_builder::Builder;
use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use interoptopus::Error;
use interoptopus::backend::IndentWriter;
use interoptopus::inventory::{Bindings, Inventory};

/// How to lay out functions.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Functions {
    Typedefs,
    #[default]
    ForwardDeclarations,
}

/// How to indent (Allman, K&R, ...)
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Indentation {
    /// Braces on their own lines, not indented
    Allman,
    /// Opening brace on same line as declaration, closing brace on own line, not intended
    KAndR,
    /// Braces on their own lines, intended by two spaces
    GNU,
    /// Braces on their own lines, intended level with members
    #[default]
    Whitesmiths,
}

/// Naming style, like lower or UPPER case.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum NameCase {
    /// Names all in lowercase without spacing e.g. 'thetypename'
    Lower,
    /// Names all in uppercase without spacing e.g. 'THETYPENAME'
    #[default]
    Upper,
    /// Names in mixed case starting with lowercase without spacing e.g. 'theTypeName'
    LowerCamel,
    /// Names in mixed case starting with uppercase without spacing e.g. '`TheTypeName`'
    UpperCamel,
    /// Names in lower case with '_' as spacing e.g. '`the_type_name`'
    Snake,
    /// Names in upper case with '_' as spacing e.g. '`THE_TYPE_NAME`'
    ShoutySnake,
}

pub trait ToNamingStyle {
    fn to_naming_style(&self, style: &NameCase) -> String;
}

impl ToNamingStyle for String {
    fn to_naming_style(&self, style: &NameCase) -> String {
        self.as_str().to_naming_style(style)
    }
}

impl ToNamingStyle for &str {
    fn to_naming_style(&self, style: &NameCase) -> String {
        match style {
            NameCase::Lower => self.to_lowercase(),
            NameCase::Upper => self.to_uppercase(),
            NameCase::LowerCamel => self.to_lower_camel_case(),
            NameCase::UpperCamel => self.to_upper_camel_case(),
            NameCase::Snake => self.to_snake_case(),
            NameCase::ShoutySnake => self.to_shouty_snake_case(),
        }
    }
}

/// Documentation style used in generated C code
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum DocStyle {
    // No documentation comments are added to header file
    None,
    // Documentation is added inline above relevant declaration
    #[default]
    Inline,
}

/// Generates C header files, **get this with [`InteropBuilder`]**.🐙
#[derive(Clone, Debug, Builder, Default)]
#[builder(default)]
pub struct Interop {
    /// Whether to write conditional directives like `#ifndef _X`.
    #[builder(default = "true")]
    directives: bool,
    /// Whether to write `#include <>` directives.
    #[builder(default = "true")]
    imports: bool,
    /// Additional `#include` lines in the form of `<item.h>` or `"item.h"`.
    additional_includes: Vec<String>,
    /// The `_X` in `#ifndef _X` to be used.
    #[builder(default = "\"interoptopus_generated\".to_string()")]
    ifndef: String,
    /// Multiline string with custom `#define` values.
    #[builder(setter(into))]
    custom_defines: String,
    /// Prefix to be applied to any function, e.g., `__DLLATTR`.
    #[builder(setter(into))]
    function_attribute: String,
    /// Comment at the very beginning of the file, e.g., `// (c) My Company.`
    #[builder(setter(into))]
    file_header_comment: String,
    /// How to prefix everything, e.g., `my_company_`, will be capitalized for constants.
    #[builder(setter(into))]
    pub(crate) prefix: String,
    /// How to indent code
    #[builder(setter(into))]
    indentation: Indentation,
    /// How to add code documentation
    #[builder(setter(into))]
    documentation: DocStyle,
    /// How to convert type names
    #[builder(setter(into))]
    pub(crate) type_naming: NameCase,
    /// How to convert enum variant names
    #[builder(setter(into))]
    pub(crate) enum_variant_naming: NameCase,
    /// How to convert const names
    #[builder(setter(into))]
    pub(crate) const_naming: NameCase,
    /// How to convert function parameter names
    #[builder(setter(into))]
    function_parameter_naming: NameCase,
    /// How to emit functions
    #[builder(setter(into))]
    function_style: Functions,
    pub(crate) inventory: Inventory,
}

impl Interop {
    pub(crate) fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        write_file_header_comments(self, w)?;
        w.newline()?;

        write_ifndef(self, w, |w| {
            write_ifdefcpp(self, w, |w| {
                if self.imports {
                    write_imports(self, w)?;
                    w.newline()?;
                }

                write_custom_defines(self, w)?;
                w.newline()?;

                write_constants(self, w)?;
                w.newline()?;

                write_type_definitions(self, w)?;
                w.newline()?;

                write_functions(self, w)?;

                Ok(())
            })?;

            Ok(())
        })?;

        Ok(())
    }
}

impl Bindings for Interop {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_to(w)
    }
}

impl InteropBuilder {
    /// Creates a new builder instance, **start here**.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
