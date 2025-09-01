//! Wire type generation for serializable FFI types.
//!
//! This module generates C# wire wrappers for types marked with `#[ffi_type(wired)]`.
//! The wire generation process:
//!
//! 1. **Type Discovery**: Uses the macro-generated `wire_info()` method (simulated by
//!    recursive type analysis) to discover all types that need serialization wrappers.
//!
//! 2. **Type Filtering**: Excludes primitive types and collections already handled by
//!    the shared serialization helpers in `wire_helpers.cs`:
//!    - Primitives: `bool`, `int`, `string`, etc.
//!    - Collections: `Vec<T>`, `Option<T>`, `Result<T,E>` where T is primitive
//!    - Built-ins: `String`, `FFIBool`, etc.
//!
//! 3. **Wrapper Generation**: For each remaining type, generates a `WireOf<Type>` struct
//!    with methods for serialization, deserialization, and size calculation.
//!
//! 4. **Recursive Dependencies**: Automatically discovers and generates wrappers for
//!    nested custom types (e.g., if `MyStruct` contains `AnotherCustomType`, both
//!    get wire wrappers).

use crate::converter::{field_name, field_to_type, wire_suffix};
use crate::Interop;
use interoptopus::lang::{Composite, Enum, Type, VariantKind, Visibility, WirePayload};
use interoptopus_backend_utils::{render, Error, IndentWriter};

pub fn write_wire_helpers(_i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    // Add single copy of shared serialization helpers.
    render!(w, "wire/helpers.cs")
}

/// Generate a `WireOfT` definition
pub fn write_type_definitions_wired(_i: &Interop, w: &mut IndentWriter, the_type: &Composite) -> Result<(), Error> {
    let type_name = wire_suffix(the_type);
    render!(w, "wire/wire_of.cs", ("type", type_name))
}

pub fn write_type_definition_wired_enum(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    #[derive(serde::Serialize)]
    struct VariantDesc {
        tag: String,
        value: usize,
    }

    i.debug(w, "write_type_definition_wired_enum")?;

    let name = the_type.rust_name();

    let variants = the_type
        .variants()
        .iter()
        .map(|variant| VariantDesc {
            tag: variant.name().to_string(),
            value: match variant.kind() {
                VariantKind::Unit(x) => *x,
                VariantKind::Typed(x, _) => *x,
            },
        })
        .collect::<Vec<_>>();

    render!(w, "wire/enum.cs", ("name", name), ("variants", &variants))
}

/// Generate a simple POCO inner type for wired types
pub fn write_type_definitions_domain_wired(i: &Interop, w: &mut IndentWriter, wired: &Composite) -> Result<(), Error> {
    #[derive(serde::Serialize)]
    struct FieldDesc {
        visibility: String,
        docs: String,
        type_name: String,
        name: String,
    }

    let type_name = wired.rust_name();
    let visibility = i.visibility_types.to_access_modifier();
    let self_kind = "partial class"; // Use class for wire inner types to allow null references
    let docs = wired.meta().docs().lines();

    let serialization_code = generate_serialization_code(w, wired)?;
    let deserialization_code = generate_deserialization_code(w, wired)?;
    let size_calculation = generate_size_calculation(w, wired)?;

    let fields = wired
        .fields()
        .iter()
        .map(|field| {
            let visibility = String::from(match field.visibility() {
                Visibility::Public => "public ",
                Visibility::Private => "",
            });

            let docs = field.docs().lines().join(" ");
            let type_name = field_to_type(field.the_type());
            let name = field_name(field);

            FieldDesc { visibility, docs, type_name, name }
        })
        .collect::<Vec<FieldDesc>>();

    render!(
        w,
        "wire/domain_type.cs",
        ("type", type_name),
        ("visibility", visibility),
        ("self_kind", self_kind),
        ("docs", &docs),
        ("fields", &fields),
        ("serialization_code", &serialization_code),
        ("deserialization_code", &deserialization_code),
        ("size_calculation", &size_calculation)
    )
}

// Domain type kinds
#[derive(serde::Serialize, Default, Copy, Clone)]
#[serde(rename_all = "snake_case")]
enum Kind {
    String,
    Vec,
    Map,
    Optional,
    Enum,
    Composite,
    #[default]
    Primitive,
}

#[allow(clippy::fallible_impl_from, reason = "It's not user input")]
impl From<&Type> for Kind {
    fn from(value: &Type) -> Self {
        match value {
            Type::Primitive(_) => Self::Primitive,
            Type::Enum(_) => Self::Enum,
            Type::WirePayload(WirePayload::String) => Self::String,
            Type::WirePayload(WirePayload::Vec(_)) => Self::Vec,
            Type::WirePayload(WirePayload::Map(_, _)) => Self::Map,
            Type::WirePayload(WirePayload::Enum(_)) => Self::Enum,
            Type::WirePayload(WirePayload::Option(_)) => Self::Optional,
            Type::WirePayload(WirePayload::Composite(_)) => Self::Composite,
            _ => panic!("Unsupported domain type kind {value:?}"),
        }
    }
}

impl From<&Box<Type>> for Kind {
    fn from(value: &Box<Type>) -> Self {
        Self::from(&**value)
    }
}

#[derive(serde::Serialize, Default)]
struct FieldDesc {
    kind: Kind,
    name: String,
    inner_kind: Kind,
    inner_type: String,
    primitive_size: usize, // calc_size
}

fn generate_serialization_code(w: &mut IndentWriter, composite: &Composite) -> Result<String, Error> {
    let mut buf = Vec::new();
    let mut writer = IndentWriter::with_same_indent_as(w, &mut buf);
    writer.indent();

    let fields = composite
        .fields()
        .iter()
        .map(|field| {
            let field_name = field.name();
            let field_type = field.the_type();
            // let csharp_type = field_to_type(field_type);
            // ^^  that's uints and ulongs and other such shit?
            let (inner_kind, inner_type) = extract_inner_type(field_type);

            FieldDesc { kind: field_type.into(), name: field_name.to_string(), inner_kind, inner_type, ..Default::default() }
        })
        .collect::<Vec<_>>();

    render!(writer, "wire/serializer.cs", ("fields", &fields))?;
    Ok(String::from_utf8(buf)?)
}

fn generate_deserialization_code(w: &mut IndentWriter, composite: &Composite) -> Result<String, Error> {
    let mut buf = Vec::new();
    let mut writer = IndentWriter::with_same_indent_as(w, &mut buf);
    writer.indent();

    let type_name = composite.rust_name();
    let fields = composite
        .fields()
        .iter()
        .map(|field| {
            let name = field.name().to_string();
            let field_type = field.the_type();
            let csharp_type = field_to_type(field_type);
            let kind: Kind = field_type.into();
            let (inner_kind, inner_type) = match kind {
                Kind::Vec => extract_inner_type(field_type),
                Kind::Optional => extract_inner_type(field_type),
                Kind::Primitive => (Kind::Primitive, csharp_type), // placeholder kind
                Kind::Map => extract_inner_type(field_type),
                _ => (Kind::Primitive, csharp_type), // we do need a placeholder kind...
            };

            FieldDesc { kind, name, inner_kind, inner_type, ..Default::default() }
        })
        .collect::<Vec<_>>();

    render!(writer, "wire/deserializer.cs", ("type", &type_name), ("fields", &fields))?;
    Ok(String::from_utf8(buf)?)
}

fn generate_size_calculation(w: &mut IndentWriter, composite: &Composite) -> Result<String, Error> {
    let mut buf = Vec::new();
    let mut writer = IndentWriter::with_same_indent_as(w, &mut buf);
    writer.indent();

    let fields = composite
        .fields()
        .iter()
        .map(|field| {
            let field_name = field.name();
            let field_type = field.the_type();
            let csharp_type = field_to_type(field_type);
            let (inner_kind, inner_type) = extract_inner_type(field_type);

            FieldDesc {
                kind: field_type.into(),
                name: field_name.to_string(),
                inner_kind,
                inner_type,
                primitive_size: if matches!(field_type, Type::Primitive(_)) {
                    get_primitive_size(&csharp_type)
                } else {
                    0
                },
            }
        })
        .collect::<Vec<_>>();

    render!(writer, "wire/calculate_size_body.cs", ("fields", &fields))?;
    Ok(String::from_utf8(buf)?)
}

fn get_primitive_size(csharp_type: &str) -> usize {
    match csharp_type {
        "bool" | "sbyte" | "byte" => 1,
        "short" | "ushort" => 2,
        "int" | "uint" | "float" => 4,
        "long" | "ulong" | "double" => 8,
        _ => 1,
    }
}

fn extract_inner_type(a_type: &Type) -> (Kind, String) {
    match a_type {
        Type::WirePayload(dom) => match dom {
            WirePayload::Vec(t) => (t.into(), field_to_type(t)),
            WirePayload::Option(o) => (o.into(), field_to_type(o)),
            WirePayload::Map(k, v) => (Kind::Map, format!("{}, {}", field_to_type(k), field_to_type(v))), // must be Kind::MapPair?
            WirePayload::Composite(_c) => (Kind::Composite, "?ask-me-how-we-got-here?".into()),
            WirePayload::String => (Kind::String, String::new()),
            WirePayload::Enum(_e) => (Kind::Enum, "!ask-me-how-we-got-here!".into()),
        },
        _ => (Kind::Primitive, "object".to_string()), // ??? do we need a placeholder kind?
    }
}
