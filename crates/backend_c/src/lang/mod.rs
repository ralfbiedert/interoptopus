//! C language model — defines the constructs the C backend can emit.
//!
//! This is intentionally minimal compared to the C# backend's lang module,
//! reflecting C's simpler type system: no generics, no managed/unmanaged
//! split, no namespaces, no overloads, no services.

mod naming;

pub use naming::{NamingConfig, NamingStyle, apply_naming_style, apply_prefix};

use interoptopus::inventory::TypeId as RustTypeId;
use std::collections::HashMap;

/// A mapped C type with its name and kind.
#[derive(Debug, Clone)]
pub struct CType {
    pub name: String,
    pub kind: CTypeKind,
}

/// The kind of a C type — each variant maps to a distinct C construct.
#[derive(Debug, Clone)]
pub enum CTypeKind {
    /// `void`, `bool`, `uint8_t`, `float`, etc.
    Primitive(CPrimitive),
    /// `typedef struct { fields... } Name;`
    Struct(CStruct),
    /// `typedef enum { A = 0, B = 1, ... } Name;`
    SimpleEnum(CSimpleEnum),
    /// Tag enum + struct with anonymous union (C11).
    TaggedUnion(CTaggedUnion),
    /// `typedef rval (*Name)(params);`
    FnPointer(CFnPointer),
    /// Fn typedef + 3-field callback struct.
    Callback(CCallback),
    /// `struct { const T* data; uint64_t len; }`
    Slice(CSlice),
    /// `struct { T* data; uint64_t len; }`
    SliceMut(CSlice),
    /// `struct { T* ptr; uint64_t len; uint64_t capacity; }`
    Vec(CVec),
    /// `struct { uint8_t* ptr; uint64_t len; uint64_t capacity; }`
    Utf8String,
    /// Tag enum + struct with `some` union member.
    Option(COption),
    /// Tag enum + struct with `ok` / `err` union members.
    Result(CResult),
    /// Forward declaration: `typedef struct Name Name;`
    Opaque,
    /// `const T*` or `T*` — not emitted standalone, participates in name resolution.
    Pointer(CPointer),
    /// Inline `T name[N]` inside struct fields — not emitted standalone.
    Array(CArray),
}

/// C primitive type names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CPrimitive {
    Void,
    Bool,
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
}

impl CPrimitive {
    /// The C type keyword for this primitive.
    #[must_use]
    pub fn c_name(self) -> &'static str {
        match self {
            Self::Void => "void",
            Self::Bool => "bool",
            Self::U8 => "uint8_t",
            Self::U16 => "uint16_t",
            Self::U32 => "uint32_t",
            Self::U64 => "uint64_t",
            Self::Usize => "size_t",
            Self::I8 => "int8_t",
            Self::I16 => "int16_t",
            Self::I32 => "int32_t",
            Self::I64 => "int64_t",
            Self::Isize => "ptrdiff_t",
            Self::F32 => "float",
            Self::F64 => "double",
        }
    }
}

/// A single field in a C struct.
#[derive(Debug, Clone)]
pub struct CField {
    pub name: String,
    pub type_name: String,
    pub array_len: Option<usize>,
}

/// A C `typedef struct` with named fields.
#[derive(Debug, Clone)]
pub struct CStruct {
    pub fields: Vec<CField>,
}

/// A variant of a simple (non-data-carrying) C enum.
#[derive(Debug, Clone)]
pub struct CEnumVariant {
    pub name: String,
    pub value: usize,
}

/// A C `typedef enum` with integer-valued variants.
#[derive(Debug, Clone)]
pub struct CSimpleEnum {
    pub variants: Vec<CEnumVariant>,
    /// The fixed-width C integer type matching the Rust `#[repr(...)]` (e.g. `uint8_t`).
    pub tag_c_type: String,
}

/// A variant of a tagged union, optionally carrying data.
#[derive(Debug, Clone)]
pub struct CTaggedUnionVariant {
    pub name: String,
    /// Lowercased variant name used as the union field name (e.g. `circle`).
    pub field_name: String,
    pub tag: usize,
    /// The type name of the data field, if this variant carries data.
    pub data_type: Option<String>,
}

/// A C11 tagged union: a tag enum + struct with anonymous union.
#[derive(Debug, Clone)]
pub struct CTaggedUnion {
    pub tag_name: String,
    pub variants: Vec<CTaggedUnionVariant>,
    /// The fixed-width C integer type matching the Rust `#[repr(...)]` (e.g. `uint8_t`).
    pub tag_c_type: String,
}

/// A C function pointer typedef.
#[derive(Debug, Clone)]
pub struct CFnPointer {
    pub rval: String,
    pub params: String,
}

/// A callback: function typedef + 3-field struct (fn, data, destructor).
#[derive(Debug, Clone)]
pub struct CCallback {
    pub fn_typedef: String,
    pub rval: String,
    pub params: String,
}

/// A slice struct (`data` pointer + `len`), either const or mutable.
#[derive(Debug, Clone)]
pub struct CSlice {
    pub inner_type: String,
    pub is_const: bool,
}

/// A vec struct (`ptr` + `len` + `capacity`).
#[derive(Debug, Clone)]
pub struct CVec {
    pub inner_type: String,
}

/// An option struct: tag enum + union with a `some` member.
#[derive(Debug, Clone)]
pub struct COption {
    pub tag_name: String,
    pub inner_type: String,
    /// The fixed-width C integer type matching the Rust `#[repr(...)]` (e.g. `uint32_t`).
    pub tag_c_type: String,
    /// Styled variant name for the `Some` tag value (e.g. `OPTION_F32_SOME`).
    pub some_variant: String,
    /// Styled variant name for the `None` tag value (e.g. `OPTION_F32_NONE`).
    pub none_variant: String,
}

/// A result struct: tag enum + union with `ok` / `err` members.
#[derive(Debug, Clone)]
pub struct CResult {
    pub tag_name: String,
    pub ok_type: String,
    pub err_type: String,
    /// The fixed-width C integer type matching the Rust `#[repr(...)]` (e.g. `uint32_t`).
    pub tag_c_type: String,
    /// Styled variant name for the `Ok` tag value (e.g. `RESULT_I32_ERROR_OK`).
    pub ok_variant: String,
    /// Styled variant name for the `Err` tag value (e.g. `RESULT_I32_ERROR_ERR`).
    pub err_variant: String,
}

/// A C pointer (`const T*` or `T*`).
#[derive(Debug, Clone)]
pub struct CPointer {
    pub target_name: String,
    pub is_const: bool,
}

/// An inline fixed-size array (`T name[N]`) inside a struct field.
#[derive(Debug, Clone)]
pub struct CArray {
    pub element_type: String,
    pub len: usize,
}

/// A mapped C function.
#[derive(Debug, Clone)]
pub struct CFunction {
    pub name: String,
    /// The original symbol name exported by the Rust cdylib (used in `dlsym`/`GetProcAddress`).
    pub symbol: String,
    pub rval: String,
    pub params: String,
    pub param_types: String,
    pub is_internal: bool,
}

/// The complete C model produced by model passes.
#[derive(Debug, Clone, Default)]
pub struct CModel {
    /// All types in topological order (dependencies before dependents).
    pub types_ordered: Vec<RustTypeId>,
    /// Type data keyed by Rust `TypeId`.
    pub types: HashMap<RustTypeId, CType>,
    /// Functions sorted by name.
    pub functions: Vec<CFunction>,
}
