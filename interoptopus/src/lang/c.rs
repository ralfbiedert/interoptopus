//! "Canonical" representation of our FFI boundary.
//!
//! Note, unless in special circumstances (e.g., when implementing [`CTypeInfo`](crate::lang::rust::CTypeInfo)
//! for a type you don't own; or when writing your own backend) you will not need any of the items in this module.
//!
//! In most cases the types here are automatically generated via an attribute; and later consumed
//! by a backend.

use crate::patterns::TypePattern;
use crate::util::ctypes_from_type_recursive;
use std::collections::HashSet;

// /// If a name like `abc::XXX` is given, strips the `abc::` part.
// fn strip_rust_path_prefix(name_with_path: &str) -> String {
//     let parts: Vec<&str> = name_with_path.split("::").collect();
//     parts.last().unwrap_or(&name_with_path).to_string()
// }

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum PrimitiveValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum ConstantValue {
    Primitive(PrimitiveValue),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Constant {
    name: String,
    value: ConstantValue,
    documentation: Documentation,
}

impl Constant {
    pub fn new(name: String, value: ConstantValue, documentation: Documentation) -> Self {
        Self { name, value, documentation }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &ConstantValue {
        &self.value
    }

    pub fn documentation(&self) -> &Documentation {
        &self.documentation
    }

    pub fn the_type(&self) -> CType {
        match &self.value {
            ConstantValue::Primitive(x) => CType::Primitive(match x {
                PrimitiveValue::Bool(_) => PrimitiveType::Bool,
                PrimitiveValue::U8(_) => PrimitiveType::U8,
                PrimitiveValue::U16(_) => PrimitiveType::U16,
                PrimitiveValue::U32(_) => PrimitiveType::U32,
                PrimitiveValue::U64(_) => PrimitiveType::U64,
                PrimitiveValue::I8(_) => PrimitiveType::I8,
                PrimitiveValue::I16(_) => PrimitiveType::I16,
                PrimitiveValue::I32(_) => PrimitiveType::I32,
                PrimitiveValue::I64(_) => PrimitiveType::I64,
                PrimitiveValue::F32(_) => PrimitiveType::F32,
                PrimitiveValue::F64(_) => PrimitiveType::F64,
            }),
        }
    }
}

/// A type that can exist at the FFI boundary.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CType {
    Primitive(PrimitiveType),
    Enum(EnumType),
    Opaque(OpaqueType),
    Composite(CompositeType),
    FnPointer(FnPointerType),
    ReadPointer(Box<CType>),
    ReadWritePointer(Box<CType>),
    /// Special patterns with primitives existing on C-level but special semantics.
    /// useful to higher level languages.
    Pattern(TypePattern),
}

impl Default for CType {
    fn default() -> Self {
        Self::Primitive(PrimitiveType::Void)
    }
}

impl CType {
    pub fn size_of(&self) -> usize {
        123
    }

    pub fn align_of(&self) -> usize {
        456
    }

    pub const fn void() -> Self {
        Self::Primitive(PrimitiveType::Void)
    }

    /// Produces a name unique for that type with respect to this library.
    ///
    /// The name here is supposed to uniquely determine a type relative to a [`Library`],
    /// but it is not guaranteed to be C-compatible and may contain special characters
    /// (e.g., `*const u32`).
    ///
    /// Backends should instead match on the `CType` variant and determine a more appropriate
    /// name on a case-by-case basis; including changing a name entirely.
    pub fn name_within_lib(&self) -> String {
        match self {
            CType::Primitive(x) => x.rust_name().to_string(),
            CType::Enum(x) => x.rust_name().to_string(),
            CType::Opaque(x) => x.rust_name().to_string(),
            CType::Composite(x) => x.rust_name().to_string(),
            CType::FnPointer(x) => x.internal_name(),
            CType::ReadPointer(x) => format!("*const {}", x.name_within_lib()),
            CType::ReadWritePointer(x) => format!("*mut {}", x.name_within_lib()),
            CType::Pattern(x) => x.fallback_type().name_within_lib(),
        }
    }

    /// Lists all _other_ types this type refers to.
    pub fn embedded_types(&self) -> Vec<CType> {
        let mut hash_set: HashSet<CType> = HashSet::new();

        ctypes_from_type_recursive(self, &mut hash_set);

        hash_set.remove(self);
        hash_set.iter().cloned().collect()
    }

    pub fn as_composite_type(&self) -> Option<&CompositeType> {
        match self {
            CType::Composite(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_opaque_type(&self) -> Option<&OpaqueType> {
        match self {
            CType::Opaque(x) => Some(x),
            _ => None,
        }
    }
}

/// A primitive type that natively exists in C and is FFI safe.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PrimitiveType {
    Void,
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl PrimitiveType {
    pub fn rust_name(&self) -> &str {
        match self {
            PrimitiveType::Void => "()",
            PrimitiveType::Bool => "bool",
            PrimitiveType::U8 => "u8",
            PrimitiveType::U16 => "u16",
            PrimitiveType::U32 => "u32",
            PrimitiveType::U64 => "u64",
            PrimitiveType::I8 => "i8",
            PrimitiveType::I16 => "i16",
            PrimitiveType::I32 => "i32",
            PrimitiveType::I64 => "i64",
            PrimitiveType::F32 => "f32",
            PrimitiveType::F64 => "f64",
        }
    }
}

/// A (C-style) enum type consisting of a name and a number of variants.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EnumType {
    name: String,
    variants: Vec<Variant>,
    documentation: Documentation,
}

impl EnumType {
    pub fn new(name: String, documentation: Documentation) -> Self {
        Self {
            name,
            variants: Vec::new(),
            documentation,
        }
    }

    pub fn add_variant(&mut self, variant: Variant) {
        self.variants.push(variant);
    }

    pub fn rust_name(&self) -> &str {
        &self.name
    }

    pub fn variants(&self) -> &[Variant] {
        &self.variants
    }

    pub fn variant_by_name(&self, name: &str) -> Option<Variant> {
        self.variants.iter().find(|x| x.name == name).cloned()
    }

    pub fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}

/// An enum variant and value.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Variant {
    name: String,
    value: usize,
    documentation: Documentation,
}

impl Variant {
    pub fn new(name: String, value: usize, documentation: Documentation) -> Self {
        Self { name, value, documentation }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}

/// Used for Rust structs with named fields, must be `#[repr(C)]`.
///
/// Might translate to a struct or class in another language, equivalent on
/// C-level to:
///
/// ```ignore
/// typedef struct MyComposite
/// {
///     int   field_1;
///     float field_2;
///     char  field_3;
///     // ...
/// } MyComposite;
/// ```
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CompositeType {
    name: String,
    fields: Vec<Field>,
    documentation: Documentation,
}

impl CompositeType {
    /// Creates a new composite with the given name and no fields nor documentation.
    pub fn new(name: String) -> Self {
        Self::with_documentation(name, Documentation::new())
    }

    /// Creates a new composite with the given name and type-level documentation.
    pub fn with_documentation(name: String, documentation: Documentation) -> Self {
        Self {
            name,
            fields: Vec::new(),
            documentation,
        }
    }

    /// Adds a field to this composite.
    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    /// Gets the type's name `
    pub fn rust_name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    /// True if this struct has no contained fields (which happens to be illegal in C99).
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Field {
    name: String,
    the_type: CType,
    documentation: Documentation,
}

impl Field {
    pub fn new(name: String, the_type: CType) -> Self {
        Self::with_documentation(name, the_type, Documentation::new())
    }

    pub fn with_documentation(name: String, the_type: CType, documentation: Documentation) -> Self {
        Self { name, the_type, documentation }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn the_type(&self) -> &CType {
        &self.the_type
    }

    pub fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OpaqueType {
    name: String,
}

impl OpaqueType {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn rust_name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Function {
    name: String,
    documentation: Documentation,
    signature: FunctionSignature,
}

impl Function {
    pub fn new(name: String, signature: FunctionSignature, documentation: Documentation) -> Self {
        Self { name, documentation, signature }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    pub fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct FunctionSignature {
    params: Vec<Parameter>,
    rval: CType,
}

impl FunctionSignature {
    pub fn new() -> Self {
        Self {
            params: vec![],
            rval: CType::Primitive(PrimitiveType::Void),
        }
    }

    pub fn add_param(&mut self, param: Parameter) {
        self.params.push(param);
    }

    pub fn set_rval(&mut self, rval: CType) {
        self.rval = rval;
    }

    pub fn params(&self) -> &[Parameter] {
        &self.params
    }

    pub fn rval(&self) -> &CType {
        &self.rval
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Parameter {
    name: String,
    the_type: CType,
}

impl Parameter {
    pub fn new(name: String, the_type: CType) -> Self {
        Self { name, the_type }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn the_type(&self) -> &CType {
        &self.the_type
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FnPointerType {
    signature: Box<FunctionSignature>,
}

impl FnPointerType {
    pub fn new(signature: FunctionSignature) -> Self {
        Self { signature: Box::new(signature) }
    }

    pub fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    pub fn internal_name(&self) -> String {
        let signature = self.signature();
        let params = signature.params.iter().map(|x| x.the_type().name_within_lib()).collect::<Vec<_>>().join(",");
        let rval = signature.rval.name_within_lib();

        format!("fn({}) -> {}", params, rval)
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Documentation {
    lines: Vec<String>,
}

impl Documentation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_line(joined_line: &str) -> Self {
        if joined_line.is_empty() {
            Documentation::new()
        } else {
            Documentation {
                lines: joined_line.split('\n').map(|x| x.to_string()).collect(),
            }
        }
    }

    pub fn from_lines(lines: Vec<String>) -> Self {
        Documentation { lines }
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}
