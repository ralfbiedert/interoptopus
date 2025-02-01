use crate::converter::{
    field_name_to_csharp_name, fnpointer_to_typename, function_parameter_to_csharp_typename, is_blittable, named_callback_to_typename, to_typespecifier_in_field,
    to_typespecifier_in_param, to_typespecifier_in_rval,
};
use crate::interop::functions::write_documentation;
use crate::interop::patterns::options::write_pattern_option;
use crate::interop::patterns::slices::{write_pattern_slice, write_pattern_slice_mut};
use crate::{Interop, Unsupported};
use interoptopus::lang::c;
use interoptopus::lang::c::{ArrayType, CType, CompositeType, EnumType, Field, FnPointerType, Layout, PrimitiveType, Variant};
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, Error};

pub fn write_type_definitions(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for the_type in i.inventory.ctypes() {
        write_type_definition(i, w, the_type)?;
    }

    Ok(())
}

pub fn write_type_definition(i: &Interop, w: &mut IndentWriter, the_type: &CType) -> Result<(), Error> {
    if !i.should_emit_by_type(the_type) {
        return Ok(());
    }

    match the_type {
        CType::Primitive(_) => {}
        CType::Array(_) => {}
        CType::Enum(e) => {
            write_type_definition_enum(i, w, e, WriteFor::Code)?;
            w.newline()?;
        }
        CType::Opaque(_) => {}
        CType::Composite(c) => {
            write_type_definition_composite(i, w, c)?;
            w.newline()?;
        }
        CType::FnPointer(f) => {
            write_type_definition_fn_pointer(i, w, f)?;
            w.newline()?;
        }
        CType::ReadPointer(_) => {}
        CType::ReadWritePointer(_) => {}
        CType::Pattern(x) => match x {
            TypePattern::CStrPointer => {}
            TypePattern::FFIErrorEnum(e) => {
                write_type_definition_enum(i, w, e.the_enum(), WriteFor::Code)?;
                w.newline()?;
            }
            TypePattern::Slice(x) => {
                write_type_definition_composite(i, w, x)?;
                w.newline()?;
                write_pattern_slice(i, w, x)?;
                w.newline()?;
            }
            TypePattern::SliceMut(x) => {
                write_type_definition_composite(i, w, x)?;
                w.newline()?;
                write_pattern_slice_mut(i, w, x)?;
                w.newline()?;
            }
            TypePattern::Option(x) => {
                write_type_definition_composite(i, w, x)?;
                w.newline()?;
                write_pattern_option(i, w, x)?;
                w.newline()?;
            }
            TypePattern::NamedCallback(x) => {
                // Handle this better way
                write_type_definition_named_callback(i, w, x)?;
                w.newline()?;
            }
            TypePattern::Bool => {
                write_type_definition_ffibool(i, w)?;
                w.newline()?;
            }

            TypePattern::CChar => {}
            TypePattern::APIVersion => {}
            _ => panic!("Pattern not explicitly handled"),
        },
    }
    Ok(())
}

pub fn write_type_definition_ffibool(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_type_definition_ffibool")?;

    let type_name = to_typespecifier_in_param(&CType::Pattern(TypePattern::Bool));

    indented!(w, r"[Serializable]")?;
    indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), type_name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"byte value;")?;
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), type_name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public static readonly {} True = new Bool {{ value =  1 }};", type_name)?;
    indented!(w, [()], r"public static readonly {} False = new Bool {{ value =  0 }};", type_name)?;
    indented!(w, [()], r"public Bool(bool b)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"value = (byte) (b ? 1 : 0);")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"public bool Is => value == 1;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}

pub fn write_type_definition_fn_pointer(i: &Interop, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
    i.debug(w, "write_type_definition_fn_pointer")?;
    write_type_definition_fn_pointer_annotation(w, the_type)?;
    write_type_definition_fn_pointer_body(i, w, the_type)?;
    Ok(())
}

pub fn write_type_definition_named_callback(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    i.debug(w, "write_type_definition_named_callback")?;
    write_type_definition_fn_pointer_annotation(w, the_type.fnpointer())?;
    write_type_definition_named_callback_body(i, w, the_type)?;
    write_callback_overload(i, w, the_type)?;
    Ok(())
}

pub fn write_callback_overload(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    if !i.work_around_exception_in_callback_no_reentry {
        return Ok(());
    }

    let CType::Pattern(TypePattern::FFIErrorEnum(ffi_error)) = the_type.fnpointer().signature().rval() else {
        return Ok(());
    };

    let name = format!("{}ExceptionSafe", the_type.name());
    let rval = to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
    let mut function_signature = Vec::new();
    let mut function_param_names = Vec::new();

    for p in the_type.fnpointer().signature().params() {
        let name = p.name();
        let the_type = function_parameter_to_csharp_typename(p);

        let x = format!("{the_type} {name}");
        function_signature.push(x);
        function_param_names.push(name);
    }

    w.newline()?;
    indented!(w, "// Internal helper that works around an issue where exceptions in callbacks don't reenter Rust.")?;
    indented!(w, "{} class {} {{", i.visibility_types.to_access_modifier(), name)?;
    indented!(w, [()], "private Exception failure = null;")?;
    indented!(w, [()], "private readonly {} _callback;", the_type.name())?;
    w.newline()?;
    indented!(w, [()], "public {}({} original)", name, the_type.name())?;
    indented!(w, [()], "{{")?;
    indented!(w, [()()], "_callback = original;")?;
    indented!(w, [()], "}}")?;
    w.newline()?;
    indented!(w, [()], "public {} Call({})", rval, function_signature.join(", "))?;
    indented!(w, [()], "{{")?;
    indented!(w, [()()], "try")?;
    indented!(w, [()()], "{{")?;
    indented!(w, [()()()], "return _callback({});", function_param_names.join(", "))?;
    indented!(w, [()()], "}}")?;
    indented!(w, [()()], "catch (Exception e)")?;
    indented!(w, [()()], "{{")?;
    indented!(w, [()()()], "failure = e;")?;
    indented!(w, [()()()], "return {}.{};", rval, ffi_error.panic_variant().name())?;
    indented!(w, [()()], "}}")?;
    indented!(w, [()], "}}")?;
    w.newline()?;
    indented!(w, [()], "public void Rethrow()")?;
    indented!(w, [()], "{{")?;
    indented!(w, [()()], "if (this.failure != null)")?;
    indented!(w, [()()], "{{")?;
    indented!(w, [()()()], "throw this.failure;")?;
    indented!(w, [()()], "}}")?;
    indented!(w, [()], "}}")?;
    indented!(w, "}}")?;

    Ok(())
}

pub fn write_type_definition_named_callback_body(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    let rval = to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
    let name = named_callback_to_typename(the_type);
    let visibility = i.visibility_types.to_access_modifier();

    let mut params = Vec::new();
    for param in the_type.fnpointer().signature().params() {
        params.push(format!("{} {}", to_typespecifier_in_param(param.the_type()), param.name()));
    }

    indented!(w, r"{} delegate {} {}({});", visibility, rval, name, params.join(", "))
}

pub fn write_type_definition_fn_pointer_annotation(w: &mut IndentWriter, _the_type: &FnPointerType) -> Result<(), Error> {
    indented!(w, r"[UnmanagedFunctionPointer(CallingConvention.Cdecl)]")
}

pub fn write_type_definition_fn_pointer_body(i: &Interop, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
    let rval = to_typespecifier_in_rval(the_type.signature().rval());
    let name = fnpointer_to_typename(the_type);
    let visibility = i.visibility_types.to_access_modifier();

    let mut params = Vec::new();
    for (i, param) in the_type.signature().params().iter().enumerate() {
        params.push(format!("{} x{}", to_typespecifier_in_param(param.the_type()), i));
    }

    indented!(w, r"{} delegate {} {}({});", visibility, rval, name, params.join(", "))
}

pub fn write_type_definition_enum(i: &Interop, w: &mut IndentWriter, the_type: &EnumType, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum")?;
    if write_for == WriteFor::Code {
        write_documentation(w, the_type.meta().documentation())?;
    }
    indented!(w, r"public enum {}", the_type.rust_name())?;
    indented!(w, r"{{")?;
    w.indent();

    for variant in the_type.variants() {
        write_type_definition_enum_variant(i, w, variant, the_type, write_for)?;
    }

    w.unindent();
    indented!(w, r"}}")
}

pub fn write_type_definition_enum_variant(_i: &Interop, w: &mut IndentWriter, variant: &Variant, _the_type: &EnumType, write_for: WriteFor) -> Result<(), Error> {
    let variant_name = variant.name();
    let variant_value = variant.value();
    if write_for == WriteFor::Code {
        write_documentation(w, variant.documentation())?;
    }
    indented!(w, r"{} = {},", variant_name, variant_value)
}

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
        indented!(w, r"var source = new ReadOnlySpan<byte>(unmanaged.{}, {});", field_name, a.len())?;
        indented!(w, r"var terminatorIndex = source.IndexOf<byte>(0);")?;
        indented!(
            w,
            r"result.{} = Encoding.UTF8.GetString(source.Slice(0, terminatorIndex == -1 ? Math.Min(source.Length, {}) : terminatorIndex));",
            field_name,
            a.len()
        )?;
    } else {
        indented!(w, r"var source = new Span<{}>(unmanaged.{}, {});", type_name, field_name, a.len())?;
        indented!(w, r"var arr_{} = new {}[{}];", field_name, type_name, a.len())?;
        indented!(w, r"source.CopyTo(arr_{}.AsSpan());", field_name)?;
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
            indented!(w, r"var source = new ReadOnlySpan<{0}>(managed.{1}, 0, managed.{1}.Length);", type_name, field_name)?;
            indented!(w, r"var dest = new Span<{0}>(result.{1}, {2});", type_name, field_name, a.len())?;
            indented!(w, r"source.CopyTo(dest);")?;
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

    match field.the_type() {
        CType::Array(a) => {
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
        }
        CType::Primitive(PrimitiveType::Bool) => {
            let type_name = to_typespecifier_in_field(field.the_type(), field, the_type);
            if !i.should_emit_marshaller_for_composite(the_type) {
                indented!(w, r"[MarshalAs(UnmanagedType.I1)]")?;
            }
            indented!(w, r"{}{} {};", visibility, type_name, field_name)
        }
        _ => {
            let type_name = to_typespecifier_in_field(field.the_type(), field, the_type);
            indented!(w, r"{}{} {};", visibility, type_name, field_name)
        }
    }
}
