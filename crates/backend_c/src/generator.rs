use crate::converters::{
    composite_to_typename, const_name_to_name, constant_value_to_value, enum_to_typename, enum_variant_to_name, fnpointer_to_typename, function_name_to_c_name,
    named_callback_to_typename, opaque_to_typename, primitive_to_typename, to_type_specifier,
};
use derive_builder::Builder;
use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use interoptopus::lang::c;
use interoptopus::lang::c::{CType, CompositeType, Constant, EnumType, Field, FnPointerType, Function, OpaqueType, Variant};
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::TypePattern;
use interoptopus::util::sort_types_by_dependencies;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Bindings};
use interoptopus::{Error, Inventory};

/// Function style used in generated C code
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Functions {
    Typedefs,
    #[default]
    ForwardDeclarations,
}

/// Indentation style used in generated C code
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

/// Naming style used in generated C code
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

/// Generates C header files, **start here**.
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
    custom_defines: String,
    /// Prefix to be applied to any function, e.g., `__DLLATTR`.
    function_attribute: String,
    /// Comment at the very beginning of the file, e.g., `// (c) My Company.`
    file_header_comment: String,
    /// How to prefix everything, e.g., `my_company_`, will be capitalized for constants.
    pub(crate) prefix: String,
    /// How to indent code
    indentation: Indentation,
    /// How to add code documentation
    documentation: DocStyle,
    /// How to convert type names
    pub(crate) type_naming: NameCase,
    /// How to convert enum variant names
    pub(crate) enum_variant_naming: NameCase,
    /// How to convert const names
    pub(crate) const_naming: NameCase,
    /// How to convert function parameter names
    function_parameter_naming: NameCase,
    /// How to emit functions
    function_style: Functions,
    pub(crate) inventory: Inventory,
}

/// Writes the C file format, `impl` this trait to customize output.
impl Interop {
    #[must_use]
    pub fn new(inventory: Inventory) -> Self {
        Self { inventory, ..Self::default() }
    }

    pub(crate) fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    fn write_custom_defines(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, "{}", &self.custom_defines)
    }

    fn write_file_header_comments(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, "{}", &self.file_header_comment)
    }

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r"#include <stdint.h>")?;
        indented!(w, r"#include <stdbool.h>")?;

        // Write any user supplied includes into the file.
        for include in &self.additional_includes {
            indented!(w, "#include {}", include)?;
        }

        Ok(())
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for constant in self.inventory.constants() {
            self.write_constant(w, constant)?;
        }

        Ok(())
    }

    fn write_constant(&self, w: &mut IndentWriter, constant: &Constant) -> Result<(), Error> {
        let name = const_name_to_name(self, constant);
        let the_type = match constant.the_type() {
            CType::Primitive(x) => primitive_to_typename(x),
            _ => return Err(Error::Null),
        };

        if self.documentation == DocStyle::Inline {
            self.write_documentation(w, constant.meta().documentation())?;
        }

        indented!(w, r"const {} {} = {};", the_type, name, constant_value_to_value(constant.value()))?;

        Ok(())
    }

    fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for function in self.inventory.functions() {
            self.write_function(w, function)?;
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        if self.documentation == DocStyle::Inline {
            self.write_documentation(w, function.meta().documentation())?;
        }

        match self.function_style {
            Functions::Typedefs => self.write_function_as_typedef_declaration(w, function)?,
            Functions::ForwardDeclarations => self.write_function_declaration(w, function, 999)?,
        }

        if self.documentation == DocStyle::Inline {
            w.newline()?;
        }

        Ok(())
    }

    pub(crate) fn write_function_declaration(&self, w: &mut IndentWriter, function: &Function, max_line: usize) -> Result<(), Error> {
        let attr = &self.function_attribute;
        let rval = to_type_specifier(self, function.signature().rval());
        let name = function_name_to_c_name(function);

        let mut params = Vec::new();

        for p in function.signature().params() {
            match p.the_type() {
                CType::Array(a) => {
                    params.push(format!(
                        "{} {}[{}]",
                        to_type_specifier(self, a.array_type()),
                        p.name().to_naming_style(&self.function_parameter_naming),
                        a.len(),
                    ));
                }
                _ => {
                    params.push(format!(
                        "{} {}",
                        to_type_specifier(self, p.the_type()),
                        p.name().to_naming_style(&self.function_parameter_naming)
                    ));
                }
            }
        }

        // Test print line to see if we need to break it
        let line = format!(r"{}{} {}({});", attr, rval, name, params.join(", "));

        if line.len() <= max_line {
            indented!(w, r"{}{} {}({});", attr, rval, name, params.join(", "))?;
        } else {
            indented!(w, r"{}{} {}(", attr, rval, name)?;
            for p in params {
                indented!(w, [()], r"{}", p)?;
            }
            indented!(w, [()], r");")?;
        }

        Ok(())
    }

    fn write_function_as_typedef_declaration(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        let rval = to_type_specifier(self, function.signature().rval());
        let name = function_name_to_c_name(function);

        let mut params = Vec::new();

        for p in function.signature().params() {
            match p.the_type() {
                CType::Array(a) => {
                    params.push(format!("{} [{}]", to_type_specifier(self, a.array_type()), a.len(),));
                }
                _ => {
                    params.push(to_type_specifier(self, p.the_type()).to_string());
                }
            }
        }
        indented!(w, r"typedef {} (*{})({});", rval, name, params.join(", "))?;

        Ok(())
    }

    #[allow(clippy::unused_self)]
    fn write_documentation(&self, w: &mut IndentWriter, documentation: &c::Documentation) -> Result<(), Error> {
        for line in documentation.lines() {
            indented!(w, r"///{}", line)?;
        }

        Ok(())
    }

    fn write_type_definitions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        let mut known_function_pointers = vec![];

        for the_type in &sort_types_by_dependencies(self.inventory.ctypes().to_vec()) {
            self.write_type_definition(w, the_type, &mut known_function_pointers)?;
        }

        Ok(())
    }

    pub(crate) fn write_type_definition(&self, w: &mut IndentWriter, the_type: &CType, known_function_pointers: &mut Vec<String>) -> Result<(), Error> {
        match the_type {
            CType::Primitive(_) => {}
            CType::Array(_) => {}
            CType::Enum(e) => {
                self.write_type_definition_enum(w, e)?;
                w.newline()?;
            }
            CType::Opaque(o) => {
                self.write_type_definition_opaque(w, o)?;
            }

            CType::Composite(c) => {
                self.write_type_definition_composite(w, c)?;
                w.newline()?;
            }
            CType::FnPointer(f) => {
                self.write_type_definition_fn_pointer(w, f, known_function_pointers)?;
                w.newline()?;
            }
            CType::ReadPointer(_) => {}
            CType::ReadWritePointer(_) => {}
            CType::Pattern(p) => match p {
                TypePattern::CStrPointer => {}
                TypePattern::NamedCallback(e) => {
                    self.write_type_definition_named_callback(w, e)?;
                    w.newline()?;
                }
                TypePattern::FFIErrorEnum(e) => {
                    self.write_type_definition_enum(w, e.the_enum())?;
                    w.newline()?;
                }
                TypePattern::Slice(x) => {
                    self.write_type_definition_composite(w, x)?;
                    w.newline()?;
                }
                TypePattern::SliceMut(x) => {
                    self.write_type_definition_composite(w, x)?;
                    w.newline()?;
                }
                TypePattern::Option(x) => {
                    self.write_type_definition_composite(w, x)?;
                    w.newline()?;
                }
                TypePattern::Bool => {}
                TypePattern::CChar => {}
                TypePattern::APIVersion => {}
                _ => panic!("Pattern not explicitly handled"),
            },
        }
        Ok(())
    }

    fn write_type_definition_fn_pointer(&self, w: &mut IndentWriter, the_type: &FnPointerType, known_function_pointers: &mut Vec<String>) -> Result<(), Error> {
        self.write_type_definition_fn_pointer_body(w, the_type, known_function_pointers)
    }

    fn write_type_definition_fn_pointer_body(&self, w: &mut IndentWriter, the_type: &FnPointerType, known_function_pointers: &mut Vec<String>) -> Result<(), Error> {
        let rval = to_type_specifier(self, the_type.signature().rval());
        let name = fnpointer_to_typename(self, the_type);

        let mut params = Vec::new();
        for (i, param) in the_type.signature().params().iter().enumerate() {
            params.push(format!("{} x{}", to_type_specifier(self, param.the_type()), i));
        }

        let fn_pointer = format!("typedef {} (*{})({});", rval, name, params.join(", "));

        if !known_function_pointers.contains(&fn_pointer) {
            indented!(w, "{}", fn_pointer)?;
            known_function_pointers.push(fn_pointer);
        }

        Ok(())
    }

    fn write_type_definition_named_callback(&self, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
        self.write_type_definition_named_callback_body(w, the_type)
    }

    fn write_type_definition_named_callback_body(&self, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
        let rval = to_type_specifier(self, the_type.fnpointer().signature().rval());
        let name = named_callback_to_typename(self, the_type);

        let mut params = Vec::new();
        for param in the_type.fnpointer().signature().params() {
            params.push(format!(
                "{} {}",
                to_type_specifier(self, param.the_type()),
                param.name().to_naming_style(&self.function_parameter_naming)
            ));
        }

        indented!(w, "{}", format!("typedef {} (*{})({});", rval, name, params.join(", ")))?;

        Ok(())
    }

    fn write_type_definition_enum(&self, w: &mut IndentWriter, the_type: &EnumType) -> Result<(), Error> {
        let name = enum_to_typename(self, the_type);

        if self.documentation == DocStyle::Inline {
            self.write_documentation(w, the_type.meta().documentation())?;
        }

        self.write_braced_declaration_opening(w, &format!("typedef enum {name}"))?;

        for variant in the_type.variants() {
            self.write_type_definition_enum_variant(w, variant, the_type)?;
        }

        self.write_braced_declaration_closing(w, name.as_str())
    }

    fn write_type_definition_enum_variant(&self, w: &mut IndentWriter, variant: &Variant, the_enum: &EnumType) -> Result<(), Error> {
        let variant_name = enum_variant_to_name(self, the_enum, variant);
        let variant_value = variant.value();

        if self.documentation == DocStyle::Inline {
            self.write_documentation(w, variant.documentation())?;
        }

        indented!(w, r"{} = {},", variant_name, variant_value)
    }

    fn write_type_definition_opaque(&self, w: &mut IndentWriter, the_type: &OpaqueType) -> Result<(), Error> {
        if self.documentation == DocStyle::Inline {
            self.write_documentation(w, the_type.meta().documentation())?;
        }

        self.write_type_definition_opaque_body(w, the_type)?;

        if self.documentation == DocStyle::Inline {
            w.newline()?;
        }

        Ok(())
    }

    fn write_type_definition_opaque_body(&self, w: &mut IndentWriter, the_type: &OpaqueType) -> Result<(), Error> {
        let name = opaque_to_typename(self, the_type);
        indented!(w, r"typedef struct {} {};", name, name)
    }

    fn write_type_definition_composite(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        if self.documentation == DocStyle::Inline {
            self.write_documentation(w, the_type.meta().documentation())?;
        }

        let name = composite_to_typename(self, the_type);

        if the_type.is_empty() {
            // C doesn't allow us writing empty structs.
            indented!(w, r"typedef struct {} {};", name, name)?;
            Ok(())
        } else {
            self.write_type_definition_composite_body(w, the_type)
        }
    }

    fn write_type_definition_composite_body(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        let name = composite_to_typename(self, the_type);

        let alignment = the_type.repr().alignment();
        if let Some(align) = alignment {
            indented!(w, "#pragma pack(push, {})", align)?;
        }

        self.write_braced_declaration_opening(w, format!(r"typedef struct {name}").as_str())?;

        for field in the_type.fields() {
            self.write_type_definition_composite_body_field(w, field, the_type)?;
        }

        self.write_braced_declaration_closing(w, name.as_str())?;

        if alignment.is_some() {
            indented!(w, "#pragma pack(pop)")?;
        }
        Ok(())
    }

    fn write_type_definition_composite_body_field(&self, w: &mut IndentWriter, field: &Field, _the_type: &CompositeType) -> Result<(), Error> {
        if self.documentation == DocStyle::Inline {
            self.write_documentation(w, field.documentation())?;
        }

        let field_name = field.name();

        if let CType::Array(x) = field.the_type() {
            let type_name = to_type_specifier(self, x.array_type());
            indented!(w, r"{} {}[{}];", type_name, field_name, x.len())
        } else {
            let field_name = field.name();
            let type_name = to_type_specifier(self, field.the_type());
            indented!(w, r"{} {};", type_name, field_name)
        }
    }

    fn write_ifndef(&self, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        if self.directives {
            indented!(w, r"#ifndef {}", self.ifndef)?;
            indented!(w, r"#define {}", self.ifndef)?;
            w.newline()?;
        }

        f(w)?;

        if self.directives {
            w.newline()?;
            indented!(w, r"#endif /* {} */", self.ifndef)?;
        }

        Ok(())
    }

    fn write_ifdefcpp(&self, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        if self.directives {
            indented!(w, r"#ifdef __cplusplus")?;
            indented!(w, r#"extern "C" {{"#)?;
            indented!(w, r"#endif")?;
            w.newline()?;
        }

        f(w)?;

        if self.directives {
            w.newline()?;
            indented!(w, r"#ifdef __cplusplus")?;
            indented!(w, r"}}")?;
            indented!(w, r"#endif")?;
        }
        Ok(())
    }

    fn write_all(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_file_header_comments(w)?;
        w.newline()?;

        self.write_ifndef(w, |w| {
            self.write_ifdefcpp(w, |w| {
                if self.imports {
                    self.write_imports(w)?;
                    w.newline()?;
                }

                self.write_custom_defines(w)?;
                w.newline()?;

                self.write_constants(w)?;
                w.newline()?;

                self.write_type_definitions(w)?;
                w.newline()?;

                self.write_functions(w)?;

                Ok(())
            })?;

            Ok(())
        })?;

        Ok(())
    }

    fn write_braced_declaration_opening(&self, w: &mut IndentWriter, definition: &str) -> Result<(), Error> {
        match self.indentation {
            Indentation::Allman => {
                indented!(w, "{}", definition)?;
                indented!(w, "{{")?;
                w.indent();
            }
            Indentation::KAndR => {
                indented!(w, "{} {{", definition)?;
                w.indent();
            }
            Indentation::GNU => {
                indented!(w, "{}", definition)?;
                indented!(w, "  {{")?;
                w.indent();
            }
            Indentation::Whitesmiths => {
                indented!(w, "{}", definition)?;
                indented!(w, [()], "{{")?;
                w.indent();
            }
        }

        Ok(())
    }

    fn write_braced_declaration_closing(&self, w: &mut IndentWriter, name: &str) -> Result<(), Error> {
        match self.indentation {
            Indentation::Allman | Indentation::KAndR => {
                w.unindent();
                indented!(w, "}} {};", name)?;
            }
            Indentation::GNU => {
                w.unindent();
                indented!(w, "  }} {};", name)?;
            }
            Indentation::Whitesmiths => {
                w.unindent();
                indented!(w, [()], "}} {};", name)?;
            }
        }

        Ok(())
    }
}

impl Bindings for Interop {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_all(w)
    }
}
