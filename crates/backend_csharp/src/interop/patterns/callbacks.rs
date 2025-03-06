use crate::converter::{named_callback_to_typename, to_typespecifier_in_param, to_typespecifier_in_rval};
use crate::interop::types::fnptrs::write_type_definition_fn_pointer_annotation;
use crate::Interop;
use interoptopus::lang::c::{CType, PrimitiveType};
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_type_definition_named_callback(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    i.debug(w, "write_type_definition_named_callback")?;

    let rval = to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
    let name = named_callback_to_typename(the_type);
    let visibility = i.visibility_types.to_access_modifier();

    let mut params = Vec::new();
    let mut params_native = Vec::new();
    let mut param_invokes = Vec::new();
    for param in the_type.fnpointer().signature().params() {
        match param.the_type() {
            CType::Pattern(TypePattern::Slice(_)) => param_invokes.push(format!("{}.Managed()", param.name())),
            CType::Pattern(TypePattern::SliceMut(_)) => param_invokes.push(format!("{}.Managed()", param.name())),
            _ => param_invokes.push(param.name().to_string()),
        }
        params.push(format!("{} {}", to_typespecifier_in_param(param.the_type()), param.name()));
        params_native.push(format!("{} {}", i.to_native_callback_typespecifier(param.the_type()), param.name()));
    }

    params.pop();
    param_invokes.pop();

    write_type_definition_fn_pointer_annotation(w, the_type.fnpointer())?;
    indented!(
        w,
        r"{} delegate {} {}Native({}); // 'True' native callback signature",
        visibility,
        i.to_native_callback_typespecifier(the_type.fnpointer().signature().rval()),
        name,
        params_native.join(", ")
    )?;
    indented!(w, r"{} delegate {} {}Delegate({}); // Our C# signature", visibility, rval, name, params.join(", "))?;
    w.newline()?;

    indented!(w, r"public partial class {}", name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"private {}Delegate _managed; // C# callback", name)?;
    indented!(w, [()], r"private {}Native _native; // Native callback ", name)?;
    indented!(w, [()], r"private IntPtr _ptr; // Raw function pointer of native callback")?;
    indented!(w, [()], r"private Exception _exception; // Set if the callback encountered an Exception")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial class {} : IDisposable", name)?;
    indented!(w, r"{{")?;
    w.newline()?;
    indented!(w, [()], r"public {}() {{ }}", name)?;
    w.newline()?;
    indented!(w, [()], r"public {}({}Delegate managed)", name, name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"_managed = managed;")?;
    indented!(w, [()()], r"_native = Call;")?;
    indented!(w, [()()], r"_ptr = Marshal.GetFunctionPointerForDelegate(_native);")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public {} Call({})", rval, params_native.join(", "))?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"// We ignore the last parameter, a generic callback pointer, as it's not needed in C#.")?;
    indented!(w, [()()], r"try")?;
    indented!(w, [()()], r"{{")?;
    if the_type.fnpointer().signature().rval().is_void() {
        indented!(w, [()()()], r"_managed({});", param_invokes.join(", "))?;
    } else {
        indented!(w, [()()()], r"return _managed({});", param_invokes.join(", "))?;
    }
    indented!(w, [()()], r"}}")?;
    indented!(w, [()()], r"catch (Exception e)")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"_exception = e;")?;
    match the_type.fnpointer().signature().rval() {
        CType::Primitive(PrimitiveType::Void) => indented!(w, [()()()], r"return;")?,
        CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
            indented!(w, [()()()], r"return new {}({}.{});", rval, e.the_enum().rust_name(), e.panic_variant().name())?;
        }
        _ => indented!(w, [()()()], r"return default;")?,
    }
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public void Dispose()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"// This means when the callback was invoked from Rust C# had an exception which")?;
    indented!(w, [()()], r"// we caught (otherwise C# might not re-enter Rust, and we leak memory). Now is")?;
    indented!(w, [()()], r"// the time to rethrow it.")?;
    indented!(w, [()()], r"if (_exception != null) throw _exception;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"[CustomMarshaller(typeof({}), MarshalMode.Default, typeof(Marshaller))]", name)?;
    indented!(w, [()], r"private struct MarshallerMeta {{  }}")?;
    w.newline()?;
    indented!(w, [()], r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, [()], r"public struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"internal IntPtr Callback;")?;
    indented!(w, [()()], r"internal IntPtr Data;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public ref struct Marshaller")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"private {} _managed;", name)?;
    indented!(w, [()()], r"private Unmanaged _unmanaged;")?;
    w.newline()?;
    indented!(w, [()()], r"public Marshaller({} managed) {{ _managed = managed; }}", name)?;
    indented!(w, [()()], r"public Marshaller(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    indented!(w, [()()], r"public void FromManaged({} managed) {{ _managed = managed; }}", name)?;
    indented!(w, [()()], r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    indented!(w, [()()], r"public Unmanaged ToUnmanaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"_unmanaged = new Unmanaged();")?;
    indented!(w, [()()()], r"_unmanaged.Callback = _managed?._ptr ?? IntPtr.Zero;")?;
    indented!(w, [()()()], r"_unmanaged.Data = IntPtr.Zero;")?;
    indented!(w, [()()()], r"return _unmanaged;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"public {} ToManaged()", name)?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"_managed = new {}();", name)?;
    indented!(w, [()()()], r"_managed._ptr = _unmanaged.Callback;")?;
    indented!(w, [()()()], r"return _managed;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"public void Free() {{ }}")?;
    indented!(w, [()], r"}}")?; // Close ref struct Marshaller.
    indented!(w, r"}}")?;
    w.newline()?;

    Ok(())
}
