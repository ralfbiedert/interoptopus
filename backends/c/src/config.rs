/// Style of indentation used in generated C code
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CIndentationStyle {
    // Braces on their own lines, not indented
    Allman,
    // Opening brace on same line as declaration, closing brace on own line, not intended
    KAndR,
    // Braces on their own lines, intended by two spaces
    GNU,
    // Braces on their own lines, intended level with members
    Whitesmiths,
}

/// Style of documentation in generated C code
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CDocumentationStyle {
    // No documentation comments are added to header file
    None,
    // Documentation is added inline above relevant declaration
    Inline,
}

/// Configures C code generation.
#[derive(Clone, Debug)]
pub struct Config {
    /// Whether to write conditional directives like `#ifndef _X`.
    pub directives: bool,
    /// Whether to write `#include <>` directives.
    pub imports: bool,
    /// The `_X` in `#ifndef _X` to be used.
    pub ifndef: String,
    /// Multiline string with custom `#define` values.
    pub custom_defines: String,
    /// Prefix to be applied to any function, e.g., `__DLLATTR`.
    pub function_attribute: String,
    /// Comment at the very beginning of the file, e.g., `// (c) My Company.`
    pub file_header_comment: String,
    /// How to prefix everything, e.g., `my_company_`, will be capitalized for constants.
    pub prefix: String,
    // How to indent code
    pub indentation: CIndentationStyle,
    // How to add code documentation
    pub documentation: CDocumentationStyle,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directives: true,
            imports: true,
            file_header_comment: "// Automatically generated by Interoptopus.".to_string(),
            ifndef: "interoptopus_generated".to_string(),
            custom_defines: "".to_string(),
            function_attribute: "".to_string(),
            prefix: "".to_string(),
            indentation: CIndentationStyle::Whitesmiths,
            documentation: CDocumentationStyle::Inline,
        }
    }
}
