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

use crate::Interop;
use crate::converter::{field_name, field_to_type};
use crate::interop::FfiTransType;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Composite, DomainType, Enum, Type, VariantKind, Visibility};
use interoptopus::pattern::TypePattern;
use interoptopus::{Error, render};

pub fn write_wire_helpers(_i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    // Add single copy of shared serialization helpers.
    render!(w, "wire/helpers.cs")
}

/// Generate a WireOfT definition
pub fn write_type_definitions_wired(_i: &Interop, w: &mut IndentWriter, wired: &Composite) -> Result<(), Error> {
    let type_name = wired.trans_type_name();

    render!(w, "wire/wire_of.cs", ("type", type_name))
}

pub fn write_type_definition_wired_enum(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_wired_enum")?;
    let name = the_type.rust_name();

    #[derive(serde::Serialize)]
    struct VariantDesc {
        tag: String,
        value: usize,
    }

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
    let type_name = wired.rust_name();
    let visibility = i.visibility_types.to_access_modifier();
    let self_kind = "partial class"; // Use class for wire inner types to allow null references
    let docs = wired.meta().docs().lines();

    let serialization_code = generate_serialization_code(w, wired)?;
    let deserialization_code = generate_deserialization_code(w, wired)?;
    let size_calculation = generate_size_calculation(w, wired)?;

    #[derive(serde::Serialize)]
    struct FieldDesc {
        visibility: String,
        docs: String,
        type_name: String,
        name: String,
    }

    let fields = wired
        .fields()
        .iter()
        .map(|field| {
            let field_visibility = match field.visibility() {
                Visibility::Public => "public ",
                Visibility::Private => "",
            };

            let field_docs = field.docs().lines().join(" ");
            let field_type_name = field_to_type(field.the_type());
            let field_name_str = field_name(field);

            eprintln!("{}: generating type {} for {:?}", field_name_str, field_type_name, field.the_type());

            FieldDesc { docs: field_docs, visibility: field_visibility.to_string(), type_name: field_type_name, name: field_name_str.to_string() }
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

impl From<&Type> for Kind {
    fn from(value: &Type) -> Self {
        match value {
            Type::Primitive(_) => Kind::Primitive,
            Type::Enum(_) => Kind::Enum,
            Type::Domain(DomainType::String) => Kind::String,
            Type::Domain(DomainType::Vec(_)) => Kind::Vec,
            Type::Domain(DomainType::Map(_, _)) => Kind::Map,
            Type::Domain(DomainType::Enum(_)) => Kind::Enum,
            Type::Domain(DomainType::Composite(_)) => Kind::Composite,
            Type::Pattern(TypePattern::Option(_)) => Kind::Optional,
            _ => panic!("Unsupported domain type kind {value:?}"),
        }
    }
}

#[derive(serde::Serialize, Default)]
struct FieldDesc {
    kind: Kind,
    name: String,
    deser_type: String,    // ser/deser - rename to csharp_type
    inner_type: String,    // calc_size
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
            let csharp_type = field_to_type(field_type);

            FieldDesc { kind: field_type.into(), name: field_name.to_string(), deser_type: csharp_type, ..Default::default() }
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
            let field_name = field.name();
            let field_type = field.the_type();
            let csharp_type = field_to_type(field_type);
            let kind: Kind = field_type.into();

            FieldDesc {
                kind: kind,
                name: field_name.to_string(),
                deser_type: match kind {
                    Kind::Vec => extract_vec_inner_type(&field_type).to_string(),
                    Kind::Optional => extract_option_inner_type(&field_type).to_string(),
                    Kind::Primitive => csharp_type,
                    Kind::Map => extract_map_keyvalue_types(&field_type).to_string(),
                    _ => csharp_type,
                },
                ..Default::default()
            }
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

            FieldDesc {
                kind: field_type.into(),
                name: field_name.to_string(),
                inner_type: if is_vec_type(field_type) {
                    extract_vec_inner_type(&field_type).to_string()
                } else if is_optional_type(field_type) {
                    extract_option_inner_type(&field_type).to_string()
                } else {
                    "".into()
                },
                primitive_size: if matches!(field_type, Type::Primitive(_)) {
                    get_primitive_size(&csharp_type)
                } else {
                    0
                },
                ..Default::default()
            }
        })
        .collect::<Vec<_>>();

    render!(writer, "wire/size_calculation.cs", ("fields", &fields))?;
    Ok(String::from_utf8(buf)?)
}

fn is_vec_type(t: &Type) -> bool {
    matches!(t, Type::Domain(DomainType::Vec(_)))
}

fn is_optional_type(t: &Type) -> bool {
    matches!(t, Type::Pattern(TypePattern::Option(_)))
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

fn extract_vec_inner_type(vec_type: &Type) -> String {
    match vec_type {
        Type::Domain(DomainType::Vec(t)) => field_to_type(t),
        _ => "object".to_string(),
    }
}

fn extract_option_inner_type(option_type: &Type) -> String {
    match option_type {
        Type::Pattern(TypePattern::Option(dom)) => field_to_type(dom.t()),
        _ => "object".to_string(),
    }
}

fn extract_map_keyvalue_types(the_type: &Type) -> String {
    match the_type {
        Type::Domain(dom) => match dom {
            DomainType::Map(u, v) => format!("{}, {}", field_to_type(u), field_to_type(v)),
            _ => "".into(),
        },
        _ => "".into(),
    }
}
