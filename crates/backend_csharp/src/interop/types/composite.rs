use crate::converter::{field_name_to_csharp_name, is_blittable, to_typespecifier_in_field};
use crate::interop::functions::write_documentation;
use crate::{Interop, Unsupported};
use interoptopus::lang::c;
use interoptopus::lang::c::{ArrayType, CType, CompositeType, Field, Layout, PrimitiveType};
use interoptopus::patterns::TypePattern;
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, Error};

pub fn write_type_definition_composite(i: &Interop, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
    i.debug(w, "write_type_definition_composite")?;
    write_documentation(w, the_type.meta().documentation())?;
    write_type_definition_composite_annotation(i, w, the_type)?;
    write_type_definition_composite_body(i, w, the_type, WriteFor::Code)?;
    write_type_definition_composite_marshaller(i, w, the_type)
}

pub fn write_type_definition_composite_marshaller(i: &Interop, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
    i.debug(w, "write_type_marshaller")?;

    if i.should_emit_marshaller_for_composite(the_type) {
        w.newline()?;
        indented!(
            w,
            r"[CustomMarshaller(typeof({}), MarshalMode.Default, typeof({}Marshaller))]",
            the_type.rust_name(),
            the_type.rust_name()
        )?;
        indented!(w, r"internal static class {}Marshaller", the_type.rust_name())?;
        indented!(w, r"{{")?;
        w.indent();
        write_type_definition_composite_layout_annotation(w, the_type)?;
        indented!(w, r"public unsafe struct Unmanaged")?;
        indented!(w, r"{{")?;
        w.indent();
        for field in the_type.fields() {
            write_type_definition_composite_unmanaged_body_field(i, w, field, the_type)?;
        }
        w.unindent();
        indented!(w, r"}}")?;
        w.unindent();
        w.newline()?;
        w.indent();
        indented!(w, r"public static Unmanaged ConvertToUnmanaged({} managed)", the_type.rust_name())?;
        indented!(w, r"{{")?;
        w.indent();
        indented!(w, r"var result = new Unmanaged")?;
        indented!(w, r"{{")?;
        w.indent();
        for field in the_type.fields().iter().filter(|t| !matches!(t.the_type(), CType::Array(_))) {
            write_type_definition_composite_to_unmanaged_inline_field(i, w, field)?;
        }
        w.unindent();
        indented!(w, r"}};")?;
        w.newline()?;
        indented!(w, r"unsafe")?;
        indented!(w, r"{{")?;
        w.indent();
        for (x, field) in the_type.fields().iter().filter(|t| matches!(t.the_type(), CType::Array(_))).enumerate() {
            if x > 0 {
                w.newline()?;
            }
            if let CType::Array(a) = field.the_type() {
                write_type_definition_composite_to_unmanaged_marshal_field(i, w, the_type, field, a)?;
            }
        }
        w.unindent();
        indented!(w, r"}}")?;
        w.newline()?;
        indented!(w, r"return result;")?;
        w.unindent();
        indented!(w, r"}}")?;
        w.newline()?;
        indented!(w, r"public static {0} ConvertToManaged(Unmanaged unmanaged)", the_type.rust_name())?;
        indented!(w, r"{{")?;
        w.indent();
        indented!(w, r"var result = new {0}()", the_type.rust_name())?;
        indented!(w, r"{{")?;
        w.indent();
        for field in the_type.fields().iter().filter(|t| !matches!(t.the_type(), CType::Array(_))) {
            write_type_definition_composite_to_managed_inline_field(i, w, field)?;
        }
        w.unindent();
        indented!(w, r"}};")?;
        w.newline()?;
        indented!(w, r"unsafe")?;
        indented!(w, r"{{")?;
        w.indent();
        for (x, field) in the_type.fields().iter().filter(|t| matches!(t.the_type(), CType::Array(_))).enumerate() {
            if x > 0 {
                w.newline()?;
            }
            if let CType::Array(a) = field.the_type() {
                write_type_definition_composite_to_managed_marshal_field(i, w, the_type, field, a)?;
            }
        }
        w.unindent();
        indented!(w, r"}}")?;
        w.newline()?;
        indented!(w, r"return result;")?;
        w.unindent();
        indented!(w, r"}}")?;
        w.unindent();
        indented!(w, r"}}")?;
        w.newline()?;
    }

    Ok(())
}

pub fn write_type_definition_composite_to_managed_marshal_field(
    i: &Interop,
    w: &mut IndentWriter,
    the_type: &CompositeType,
    field: &Field,
    a: &ArrayType,
) -> Result<(), Error> {
    let field_name = field_name_to_csharp_name(field, i.rename_symbols);
    let type_name = to_typespecifier_in_field(a.array_type(), field, the_type);
    if i.unroll_struct_arrays {
        for i in 0..a.len() {
            indented!(w, r"result.{0}{1} = unmanaged.{0}[{1}];", field_name, i)?;
        }
    } else if matches!(a.array_type(), CType::Pattern(TypePattern::CChar)) {
        indented!(w, r"var source_{0} = new ReadOnlySpan<byte>(unmanaged.{0}, {1});", field_name, a.len())?;
        indented!(w, r"var terminatorIndex_{0} = source_{0}.IndexOf<byte>(0);", field_name)?;
        indented!(
            w,
            r"result.{0} = Encoding.UTF8.GetString(source_{0}.Slice(0, terminatorIndex_{0} == -1 ? Math.Min(source_{0}.Length, {1}) : terminatorIndex_{0}));",
            field_name,
            a.len()
        )?;
    } else {
        indented!(w, r"var source_{1} = new Span<{0}>(unmanaged.{1}, {2});", type_name, field_name, a.len())?;
        indented!(w, r"var arr_{} = new {}[{}];", field_name, type_name, a.len())?;
        indented!(w, r"source_{0}.CopyTo(arr_{0}.AsSpan());", field_name)?;
        indented!(w, r"result.{0} = arr_{0};", field_name)?;
    }
    Ok(())
}

pub fn write_type_definition_composite_to_managed_inline_field(i: &Interop, w: &mut IndentWriter, field: &Field) -> Result<(), Error> {
    let field_name = field_name_to_csharp_name(field, i.rename_symbols);
    match field.the_type() {
        CType::Primitive(PrimitiveType::Bool) => {
            indented!(w, r"{0} = Convert.ToBoolean(unmanaged.{0}),", field_name)?;
        }
        CType::Composite(composite) if i.should_emit_marshaller_for_composite(composite) => {
            indented!(w, r"{0} = {1}Marshaller.ConvertToManaged(unmanaged.{0}),", field_name, composite.rust_name())?;
        }
        _ => {
            indented!(w, r"{0} = unmanaged.{0},", field_name)?;
        }
    }
    Ok(())
}

pub fn write_type_definition_composite_to_unmanaged_marshal_field(
    i: &Interop,
    w: &mut IndentWriter,
    the_type: &CompositeType,
    field: &Field,
    a: &ArrayType,
) -> Result<(), Error> {
    let field_name = field_name_to_csharp_name(field, i.rename_symbols);
    let type_name = to_typespecifier_in_field(a.array_type(), field, the_type);
    if i.unroll_struct_arrays {
        for i in 0..a.len() {
            indented!(w, r"result.{0}[{1}] = managed.{0}{1};", field_name, i)?;
        }
    } else {
        indented!(w, r"if(managed.{} != null)", field_name)?;
        indented!(w, r"{{")?;
        w.indent();
        if matches!(a.array_type(), CType::Pattern(TypePattern::CChar)) {
            indented!(w, "fixed(char* s = managed.{})", field_name)?;
            indented!(w, "{{")?;
            w.indent();
            indented!(
                w,
                r"if(Encoding.UTF8.GetByteCount(managed.{0}, 0, managed.{0}.Length) + 1 > {1})",
                field_name,
                a.len()
            )?;
            indented!(w, r"{{")?;
            w.indent();
            indented!(
                w,
                r#"throw new InvalidOperationException($"The managed string field '{{nameof({0}.{1})}}' cannot be encoded to fit the fixed size array of {2}.");"#,
                the_type.rust_name(),
                field_name,
                a.len()
            )?;
            w.unindent();
            indented!(w, r"}}")?;
            indented!(
                w,
                r"var written = Encoding.UTF8.GetBytes(s, managed.{0}.Length, result.{0}, {1});",
                field_name,
                a.len() - 1
            )?;
            indented!(w, r"result.{}[written] = 0;", field_name)?;
            w.unindent();
            indented!(w, r"}}")?;
        } else {
            indented!(w, r"if(managed.{}.Length > {})", field_name, a.len())?;
            indented!(w, r"{{")?;
            w.indent();
            indented!(
                w,
                r#"throw new InvalidOperationException($"The managed array field '{{nameof({0}.{1})}}' has {{managed.{1}.Length}} elements, exceeding the fixed size array of {2}.");"#,
                the_type.rust_name(),
                field_name,
                a.len()
            )?;
            w.unindent();
            indented!(w, r"}}")?;
            indented!(
                w,
                r"var source_{1} = new ReadOnlySpan<{0}>(managed.{1}, 0, managed.{1}.Length);",
                type_name,
                field_name
            )?;
            indented!(w, r"var dest = new Span<{0}>(result.{1}, {2});", type_name, field_name, a.len())?;
            indented!(w, r"source_{}.CopyTo(dest);", field_name)?;
        }
        w.unindent();
        indented!(w, r"}}")?;
    }
    Ok(())
}

pub fn write_type_definition_composite_to_unmanaged_inline_field(i: &Interop, w: &mut IndentWriter, field: &Field) -> Result<(), Error> {
    let field_name = field_name_to_csharp_name(field, i.rename_symbols);
    match field.the_type() {
        CType::Primitive(PrimitiveType::Bool) => {
            indented!(w, r"{0} = Convert.ToSByte(managed.{0}),", field_name)?;
        }
        CType::Composite(composite) if i.should_emit_marshaller_for_composite(composite) => {
            indented!(w, r"{0} = {1}Marshaller.ConvertToUnmanaged(managed.{0}),", field_name, composite.rust_name())?;
        }
        _ => {
            indented!(w, r"{0} = managed.{0},", field_name)?;
        }
    }
    Ok(())
}

pub fn write_type_definition_composite_unmanaged_body_field(i: &Interop, w: &mut IndentWriter, field: &Field, the_type: &CompositeType) -> Result<(), Error> {
    let field_name = field_name_to_csharp_name(field, i.rename_symbols);
    match field.the_type() {
        CType::Array(a) => {
            let type_name = to_typespecifier_in_field(a.array_type(), field, the_type);
            let size = a.len();
            if matches!(a.array_type(), CType::Pattern(TypePattern::CChar)) {
                indented!(w, r"public fixed byte {}[{}];", field_name, size)?;
            } else {
                indented!(w, r"public fixed {} {}[{}];", type_name, field_name, size)?;
            }
        }
        CType::Primitive(PrimitiveType::Bool) => {
            indented!(w, r"public sbyte {};", field_name)?;
        }
        CType::Composite(composite) => {
            if i.should_emit_marshaller_for_composite(composite) {
                indented!(w, r"public {}Marshaller.Unmanaged {};", composite.rust_name(), field_name)?;
            } else {
                indented!(w, r"public {} {};", composite.rust_name(), field_name)?;
            }
        }
        _ => {
            let type_name = to_typespecifier_in_field(field.the_type(), field, the_type);
            indented!(w, r"public {} {};", type_name, field_name)?;
        }
    }
    Ok(())
}

pub fn write_type_definition_composite_annotation(i: &Interop, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
    indented!(w, r"[Serializable]")?;

    if the_type.repr().alignment().is_some() {
        let comment = r"// THIS STRUCT IS BROKEN - C# does not support alignment of entire Rust types that do #[repr(align(...))]";
        match i.unsupported {
            Unsupported::Panic => panic!("{}", comment),
            Unsupported::Comment => indented!(w, "{}", comment)?,
        }
    }

    if i.should_emit_marshaller_for_composite(the_type) {
        indented!(w, r"[NativeMarshalling(typeof({}Marshaller))]", the_type.rust_name())?;
    } else {
        write_type_definition_composite_layout_annotation(w, the_type)?;
    }

    Ok(())
}

#[allow(clippy::unused_self)]
pub fn write_type_definition_composite_layout_annotation(w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
    match the_type.repr().layout() {
        Layout::C | Layout::Transparent | Layout::Opaque => indented!(w, r"[StructLayout(LayoutKind.Sequential)]"),
        Layout::Packed => indented!(w, r"[StructLayout(LayoutKind.Sequential, Pack = 1)]"),
        Layout::Primitive(_) => panic!("Primitive layout not supported for structs."),
    }
}

pub fn write_type_definition_composite_body(i: &Interop, w: &mut IndentWriter, the_type: &CompositeType, write_for: WriteFor) -> Result<(), Error> {
    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), the_type.rust_name())?;
    indented!(w, r"{{")?;
    w.indent();

    for field in the_type.fields() {
        if write_for == WriteFor::Code {
            write_documentation(w, field.documentation())?;
        }

        write_type_definition_composite_body_field(i, w, field, the_type)?;
    }

    w.unindent();
    indented!(w, r"}}")
}

pub fn write_type_definition_composite_body_field(i: &Interop, w: &mut IndentWriter, field: &Field, the_type: &CompositeType) -> Result<(), Error> {
    let field_name = field_name_to_csharp_name(field, i.rename_symbols);
    let visibility = match field.visibility() {
        c::Visibility::Public => "public ",
        c::Visibility::Private if i.should_emit_marshaller_for_composite(the_type) => "internal ",
        c::Visibility::Private => "",
    };

    if let CType::Array(a) = field.the_type() {
        if i.unroll_struct_arrays {
            let type_name = to_typespecifier_in_field(a.array_type(), field, the_type);
            for i in 0..a.len() {
                indented!(w, r"{}{} {}{};", visibility, type_name, field_name, i)?;
            }
        } else {
            assert!(is_blittable(a.array_type()), "Array type is not blittable: {:?}", a.array_type());

            let type_name = if matches!(a.array_type(), CType::Pattern(TypePattern::CChar)) {
                "string".to_string()
            } else {
                format!("{}[]", to_typespecifier_in_field(a.array_type(), field, the_type))
            };

            indented!(w, r"{}{} {};", visibility, type_name, field_name)?;
        }

        Ok(())
    } else {
        let type_name = to_typespecifier_in_field(field.the_type(), field, the_type);
        indented!(w, r"{}{} {};", visibility, type_name, field_name)
    }
}
