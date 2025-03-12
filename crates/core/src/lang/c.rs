//! Canonical, _almost_-C representation of items in an FFI boundary.
//!
//! The types in here are the [`crate::Inventory`] building blocks with which
//! a C API can be built. In addition, they contain a few extra, non-C elements
//! (e.g., namespaces, patterns), all of which however can reasonably be mapped to or ignored in C.
//!
//! Except for special circumstances (e.g., when implementing [`CTypeInfo`](crate::lang::rust::CTypeInfo)
//! for a type you don't own; or when writing your own backend) you will not need any of the items in this module.
//! In most cases the **types here are automatically generated by an attribute**; and later **consumed
//! by a backend**.

use crate::patterns::TypePattern;
use crate::patterns::callbacks::AsyncCallback;
use crate::patterns::result::FFIResultType;
use crate::util::{IdPrettifier, capitalize_first_letter, ctypes_from_type_recursive};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
// /// If a name like `abc::XXX` is given, strips the `abc::` part.
// fn strip_rust_path_prefix(name_with_path: &str) -> String {
//     let parts: Vec<&str> = name_with_path.split("::").collect();
//     parts.last().unwrap_or(&name_with_path).to_string()
// }

/// A primitive value expressible on C-level.
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

/// The value of a constant.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum ConstantValue {
    Primitive(PrimitiveValue),
}

impl ConstantValue {
    pub(crate) fn fucking_hash_it_already<H: Hasher>(&self, h: &mut H) {
        match self {
            Self::Primitive(x) => match x {
                PrimitiveValue::Bool(x) => x.hash(h),
                PrimitiveValue::U8(x) => x.hash(h),
                PrimitiveValue::U16(x) => x.hash(h),
                PrimitiveValue::U32(x) => x.hash(h),
                PrimitiveValue::U64(x) => x.hash(h),
                PrimitiveValue::I8(x) => x.hash(h),
                PrimitiveValue::I16(x) => x.hash(h),
                PrimitiveValue::I32(x) => x.hash(h),
                PrimitiveValue::I64(x) => x.hash(h),
                PrimitiveValue::F32(x) => x.to_le_bytes().hash(h),
                PrimitiveValue::F64(x) => x.to_le_bytes().hash(h),
            },
        }
    }
}

/// A Rust `const` definition with a name and value, might become a `#define`.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Constant {
    name: String,
    value: ConstantValue,
    meta: Meta,
}

impl Constant {
    #[must_use]
    pub const fn new(name: String, value: ConstantValue, meta: Meta) -> Self {
        Self { name, value, meta }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn value(&self) -> &ConstantValue {
        &self.value
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    /// Returns the type of this constant.
    #[must_use]
    pub const fn the_type(&self) -> CType {
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
    Array(ArrayType),
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
    #[must_use]
    pub const fn size_of(&self) -> usize {
        match self {
            Self::Primitive(p) => match p {
                PrimitiveType::Void => 0,
                PrimitiveType::Bool => 1,
                PrimitiveType::U8 => 1,
                PrimitiveType::U16 => 2,
                PrimitiveType::U32 => 4,
                PrimitiveType::U64 => 8,
                PrimitiveType::I8 => 1,
                PrimitiveType::I16 => 2,
                PrimitiveType::I32 => 4,
                PrimitiveType::I64 => 8,
                PrimitiveType::F32 => 4,
                PrimitiveType::F64 => 8,
            },
            // TODO
            _ => 999,
        }
    }

    #[must_use]
    pub fn align_of(&self) -> usize {
        unimplemented!()
    }

    #[must_use]
    pub const fn void() -> Self {
        Self::Primitive(PrimitiveType::Void)
    }

    /// Produces a name unique for that type with respect to this library.
    ///
    /// The name here is supposed to uniquely determine a type relative to a library ([`crate::Inventory`]).
    ///
    /// Backends may instead match on the `CType` variant and determine a more appropriate
    /// name on a case-by-case basis; including changing a name entirely.
    #[must_use]
    pub fn name_within_lib(&self) -> String {
        match self {
            Self::Primitive(x) => x.rust_name().to_string(),
            Self::Enum(x) => x.rust_name().to_string(),
            Self::Opaque(x) => x.rust_name().to_string(),
            Self::Composite(x) => x.rust_name().to_string(),
            Self::FnPointer(x) => x.rust_name(),
            Self::ReadPointer(x) => format!("ConstPtr{}", capitalize_first_letter(x.name_within_lib().as_str())),
            Self::ReadWritePointer(x) => format!("MutPtr{}", capitalize_first_letter(x.name_within_lib().as_str())),
            Self::Pattern(x) => match x {
                TypePattern::Bool => "Bool".to_string(),
                _ => x.fallback_type().name_within_lib(),
            },
            Self::Array(x) => x.rust_name(),
        }
    }

    /// Lists all _other_ types this type refers to.
    #[must_use]
    pub fn embedded_types(&self) -> Vec<Self> {
        let mut hash_set: HashSet<Self> = HashSet::new();

        ctypes_from_type_recursive(self, &mut hash_set);

        hash_set.remove(self);
        hash_set.iter().cloned().collect()
    }

    /// If this were a pointer, tries to deref it and return the inner type.
    #[must_use]
    pub fn try_deref_pointer(&self) -> Option<&Self> {
        match self {
            Self::Primitive(_) => None,
            Self::Enum(_) => None,
            Self::Opaque(_) => None,
            Self::Composite(_) => None,
            Self::FnPointer(_) => None,
            Self::ReadPointer(x) => Some(x.as_ref()),
            Self::ReadWritePointer(x) => Some(x.as_ref()),
            Self::Pattern(_) => None,
            Self::Array(_) => None,
        }
    }

    /// Convenience method attempting to convert the contained type as a composite.
    #[must_use]
    pub const fn as_composite_type(&self) -> Option<&CompositeType> {
        match self {
            Self::Composite(x) => Some(x),
            _ => None,
        }
    }

    /// Convenience method attempting to convert the contained type as an opaque.
    #[must_use]
    pub const fn as_opaque_type(&self) -> Option<&OpaqueType> {
        match self {
            Self::Opaque(x) => Some(x),
            _ => None,
        }
    }

    /// Convenience method attempting to convert the contained type as a composite.
    #[must_use]
    pub const fn as_result(&self) -> Option<&FFIResultType> {
        match self {
            Self::Pattern(TypePattern::Result(x)) => Some(x),
            _ => None,
        }
    }

    /// Convenience method attempting to convert the contained type as an Async callback.
    #[must_use]
    pub const fn as_async_callback(&self) -> Option<&AsyncCallback> {
        match self {
            Self::Pattern(TypePattern::AsyncCallback(x)) => Some(x),
            _ => None,
        }
    }

    /// Convenience method attempting to get the pointer target of a contained type.
    #[must_use]
    pub const fn pointer_target(&self) -> Option<&Self> {
        match self {
            Self::ReadPointer(x) => Some(x),
            Self::ReadWritePointer(x) => Some(x),
            _ => None,
        }
    }

    /// Checks if this is a [`PrimitiveType::Void`].
    #[must_use]
    pub const fn is_void(&self) -> bool {
        matches!(self, Self::Primitive(PrimitiveType::Void))
    }

    /// Returns the namespace of the type.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        match self {
            Self::Array(t) => t.array_type().namespace(),
            Self::Enum(t) => Some(t.meta.namespace()),
            Self::Opaque(t) => Some(t.meta.namespace()),
            Self::Composite(t) => Some(t.meta.namespace()),
            Self::Pattern(TypePattern::NamedCallback(t)) => Some(t.meta().namespace()),
            _ => None,
        }
    }
}

/// A primitive type that natively exists in C and is FFI safe.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
    #[must_use]
    pub const fn rust_name(&self) -> &str {
        match self {
            Self::Void => "()",
            Self::Bool => "bool",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::F32 => "f32",
            Self::F64 => "f64",
        }
    }
}

/// A (C-style) `type[N]` containing a fixed number of elements of the same type.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ArrayType {
    array_type: Box<CType>,
    len: usize,
}

impl ArrayType {
    #[must_use]
    pub fn new(array_type: CType, len: usize) -> Self {
        Self { array_type: Box::new(array_type), len }
    }

    #[must_use]
    pub fn rust_name(&self) -> String {
        format!("{}[{}]", self.array_type.name_within_lib(), self.len)
    }

    #[must_use]
    pub const fn array_type(&self) -> &CType {
        &self.array_type
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// A (C-style) `enum` containing numbered variants.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EnumType {
    name: String,
    variants: Vec<Variant>,
    repr: Representation,
    meta: Meta,
}

impl EnumType {
    #[must_use]
    pub const fn new(name: String, variants: Vec<Variant>, meta: Meta, repr: Representation) -> Self {
        Self { name, variants, repr, meta }
    }

    #[must_use]
    pub fn rust_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn variants(&self) -> &[Variant] {
        &self.variants
    }

    #[must_use]
    pub fn variant_by_name(&self, name: &str) -> Option<Variant> {
        self.variants.iter().find(|x| x.name == name).cloned()
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    #[must_use]
    pub const fn repr(&self) -> &Representation {
        &self.repr
    }
}

/// Variant and value of a [`EnumType`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Variant {
    name: String,
    value: usize,
    documentation: Documentation,
}

impl Variant {
    #[must_use]
    pub const fn new(name: String, value: usize, documentation: Documentation) -> Self {
        Self { name, value, documentation }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn value(&self) -> usize {
        self.value
    }

    #[must_use]
    pub const fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}

/// Used for Rust and C `struct` with named fields, must be `#[repr(C)]`.
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
    repr: Representation,
    meta: Meta,
}

impl CompositeType {
    /// Creates a new composite with the given name and fields and no documentation.
    #[must_use]
    pub fn new(name: String, fields: Vec<Field>) -> Self {
        Self::with_meta(name, fields, Meta::new())
    }

    /// Creates a new composite with the given name and type-level documentation.
    #[must_use]
    pub fn with_meta(name: String, fields: Vec<Field>, meta: Meta) -> Self {
        Self { name, fields, meta, repr: Representation::default() }
    }

    /// Creates a new composite with the given name and type-level documentation.
    #[must_use]
    pub const fn with_meta_repr(name: String, fields: Vec<Field>, meta: Meta, repr: Representation) -> Self {
        Self { name, fields, repr, meta }
    }

    /// Gets the type's name.
    #[must_use]
    pub fn rust_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    /// If this were a wrapper over a pointer type, get the type of what we're pointing go.
    #[must_use]
    pub fn try_deref_pointer(&self) -> Option<CType> {
        self.fields().first()?.the_type().try_deref_pointer().cloned()
    }

    /// True if this struct has no contained fields (which happens to be illegal in C99).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    #[must_use]
    pub const fn repr(&self) -> &Representation {
        &self.repr
    }

    #[must_use]
    pub fn into_ctype(&self) -> CType {
        CType::Composite(self.clone())
    }
}

/// Doesn't exist in C, but other languages can benefit from accidentally using 'private' fields.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Visibility {
    Public,
    Private,
}

/// How a struct is laid out in memory.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Layout {
    C,
    Transparent,
    Packed,
    Opaque,
    /// For use with enum discriminant.
    Primitive(PrimitiveType),
}

/// How a type is represented in memory.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Representation {
    layout: Layout,
    alignment: Option<usize>,
}

impl Default for Representation {
    fn default() -> Self {
        Self { layout: Layout::C, alignment: None }
    }
}

impl Representation {
    #[must_use]
    pub const fn new(layout: Layout, alignment: Option<usize>) -> Self {
        Self { layout, alignment }
    }

    #[must_use]
    pub const fn layout(&self) -> Layout {
        self.layout
    }

    #[must_use]
    pub const fn alignment(&self) -> Option<usize> {
        self.alignment
    }
}

/// Fields of a [`CompositeType`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Field {
    name: String,
    visibility: Visibility,
    the_type: CType,
    documentation: Documentation,
}

impl Field {
    #[must_use]
    pub fn new(name: String, the_type: CType) -> Self {
        Self::with_documentation(name, the_type, Visibility::Public, Documentation::new())
    }

    #[must_use]
    pub const fn with_documentation(name: String, the_type: CType, visibility: Visibility, documentation: Documentation) -> Self {
        Self { name, visibility, the_type, documentation }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn the_type(&self) -> &CType {
        &self.the_type
    }

    #[must_use]
    pub const fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    #[must_use]
    pub const fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}

/// A named `struct` that becomes a fieldless `typedef struct S S;` in C.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OpaqueType {
    name: String,
    meta: Meta,
}

impl OpaqueType {
    #[must_use]
    pub const fn new(name: String, meta: Meta) -> Self {
        Self { name, meta }
    }

    #[must_use]
    pub fn rust_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }
}

/// Additional information for user-defined types.
#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Meta {
    documentation: Documentation,
    namespace: String,
}

impl Meta {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_namespace_documentation(namespace: String, documentation: Documentation) -> Self {
        Self { documentation, namespace }
    }

    #[must_use]
    pub const fn with_documentation(documentation: Documentation) -> Self {
        Self::with_namespace_documentation(String::new(), documentation)
    }

    #[must_use]
    pub const fn documentation(&self) -> &Documentation {
        &self.documentation
    }

    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Convenience method used in generators
    #[must_use]
    pub fn is_namespace(&self, namespace: &str) -> bool {
        self.namespace == namespace
    }
}

/// Indicates the final desired return type in FFI'ed user code.
pub enum SugaredReturnType {
    Sync(CType),
    Async(CType),
}

impl SugaredReturnType {
    #[must_use]
    pub fn is_async(&self) -> bool {
        matches!(self, Self::Async(_))
    }

    #[must_use]
    pub fn is_sync(&self) -> bool {
        matches!(self, Self::Sync(_))
    }
}

/// A named, exported `#[no_mangle] extern "C" fn f()` function.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Function {
    name: String,
    meta: Meta,
    signature: FunctionSignature,
}

impl Function {
    #[must_use]
    pub const fn new(name: String, signature: FunctionSignature, meta: Meta) -> Self {
        Self { name, meta, signature }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    #[must_use]
    pub fn prettifier(&self) -> IdPrettifier {
        IdPrettifier::from_rust_lower(self.name())
    }

    #[must_use]
    pub fn first_param_type(&self) -> Option<&CType> {
        self.signature().params.first().map(|x| &x.the_type)
    }

    #[must_use]
    pub const fn returns_ffi_error(&self) -> bool {
        matches!(self.signature().rval(), CType::Pattern(TypePattern::FFIErrorEnum(_)))
    }

    /// Indicates the return type of a method from user code.
    ///
    /// Sync methods have their return type as-is, in async methods
    /// this indicates the type of the async callback helper.
    #[must_use]
    pub fn sugared_return_type(&self) -> SugaredReturnType {
        let ctype = self
            .signature
            .params
            .last()
            .and_then(|x| x.the_type().as_async_callback())
            .map(|async_callback: &AsyncCallback| async_callback.target());

        match ctype {
            None => SugaredReturnType::Sync(self.signature.rval().clone()),
            Some(x) => SugaredReturnType::Async(x.clone()),
        }
    }
}

/// Represents multiple `in` and a single `out` parameters.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct FunctionSignature {
    params: Vec<Parameter>,
    rval: CType,
}

impl FunctionSignature {
    #[must_use]
    pub const fn new(params: Vec<Parameter>, rval: CType) -> Self {
        Self { params, rval }
    }

    #[must_use]
    pub fn params(&self) -> &[Parameter] {
        &self.params
    }

    #[must_use]
    pub const fn rval(&self) -> &CType {
        &self.rval
    }
}

/// Parameters of a [`FunctionSignature`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Parameter {
    name: String,
    the_type: CType,
}

impl Parameter {
    #[must_use]
    pub const fn new(name: String, the_type: CType) -> Self {
        Self { name, the_type }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn the_type(&self) -> &CType {
        &self.the_type
    }
}

/// Represents `extern "C" fn()` types in Rust and `(*f)().` in C.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FnPointerType {
    name: Option<String>,
    signature: Box<FunctionSignature>,
}

impl FnPointerType {
    #[must_use]
    pub fn new(signature: FunctionSignature) -> Self {
        Self { signature: Box::new(signature), name: None }
    }

    #[must_use]
    pub fn new_named(signature: FunctionSignature, name: String) -> Self {
        Self { signature: Box::new(signature), name: Some(name) }
    }

    #[must_use]
    pub const fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[must_use]
    pub fn internal_name(&self) -> String {
        let signature = self.signature();
        let params = signature.params.iter().map(|x| x.the_type().name_within_lib()).collect::<Vec<_>>().join(",");
        let rval = signature.rval.name_within_lib();

        format!("fn({params}) -> {rval}")
    }

    #[must_use]
    pub fn rust_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| self.internal_name())
    }
}

/// Markdown generated from the `///` you put on Rust code.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Documentation {
    lines: Vec<String>,
}

impl Documentation {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from_line(joined_line: &str) -> Self {
        if joined_line.is_empty() {
            Self::new()
        } else {
            Self { lines: joined_line.split('\n').map(std::string::ToString::to_string).collect() }
        }
    }

    #[must_use]
    pub const fn from_lines(lines: Vec<String>) -> Self {
        Self { lines }
    }

    #[must_use]
    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}
