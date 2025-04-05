use crate::Interop;
use crate::converter::{is_blittable, param_to_type, rval_to_type_sync};
use crate::interop::types::fnptrs::write_type_definition_fn_pointer_annotation;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Primitive, Type};
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::callback::NamedCallback;
use interoptopus::{Error, indented};

pub fn write_type_definition_named_callback(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    i.debug(w, "write_type_definition_named_callback")?;

    let rval_safe = rval_to_type_sync(the_type.fnpointer().signature().rval());
    let rval_unsafe = match the_type.fnpointer().signature().rval() {
        Type::Composite(_) => format!("{rval_safe}.Unmanaged"),
        Type::Enum(_) => format!("{rval_safe}.Unmanaged"),
        Type::Pattern(TypePattern::Result(_)) => format!("{rval_safe}.Unmanaged"),
        _ => rval_safe.clone(),
    };

    let name = the_type.name().to_string();
    let visibility = i.visibility_types.to_access_modifier();

    let mut params = Vec::new();
    let mut params_native = Vec::new();
    let mut params_invoke = Vec::new();
    for param in the_type.fnpointer().signature().params() {
        params.push(format!("{} {}", param_to_type(param.the_type()), param.name()));
        params_native.push(format!("{} {}", i.to_native_callback_typespecifier(param.the_type()), param.name()));

        match param.the_type() {
            Type::Primitive(_) => params_invoke.push(param.name().to_string()),
            Type::Opaque(_) => params_invoke.push(param.name().to_string()),
            Type::ReadPointer(_) => params_invoke.push(param.name().to_string()),
            Type::ReadWritePointer(_) => params_invoke.push(param.name().to_string()),
            x if is_blittable(x) => params_invoke.push(format!("{}.ToManaged()", param.name())),
            _ => params_invoke.push(format!("{}.IntoManaged()", param.name())),
        }
    }

    params.pop();
    params_invoke.pop();

    write_type_definition_fn_pointer_annotation(w, the_type.fnpointer())?;
    let params_unsafe_str = params_native.join(", ");
    let params_str = params.join(", ");
    let params_invoke = params_invoke.join(", ");
    indented!(w, r"{visibility} delegate {rval_unsafe} {name}Native({params_unsafe_str}); // 'True' native callback signature",)?;
    indented!(w, r"{visibility} delegate {rval_safe} {name}Delegate({params_str}); // Our C# signature")?;
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
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public {}({}Delegate managed)", name, name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"_managed = managed;")?;
    indented!(w, [()()], r"_native = CallTrampoline;")?;
    indented!(w, [()()], r"_ptr = Marshal.GetFunctionPointerForDelegate(_native);")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"// Helper to invoke managed code from the native invocation.")?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"private {rval_unsafe} CallTrampoline({params_unsafe_str})")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"// We ignore the last parameter, a generic callback pointer, as it's not needed in C#.")?;
    indented!(w, [()()], r"try")?;
    indented!(w, [()()], r"{{")?;
    match the_type.fnpointer().signature().rval() {
        Type::Primitive(Primitive::Void) => indented!(w, [()()()], r"_managed({params_invoke});")?,
        Type::Primitive(_) => indented!(w, [()()()], r"return _managed({params_invoke});")?,
        t if is_blittable(t) => indented!(w, [()()()], r"return _managed({params_invoke}).ToUnmanaged();")?,
        t => indented!(w, [()()()], r"return _managed({params_invoke}).IntoUnmanaged();")?,
    }
    indented!(w, [()()], r"}}")?;
    indented!(w, [()()], r"catch (Exception e)")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"_exception = e;")?;
    match the_type.fnpointer().signature().rval() {
        Type::Primitive(Primitive::Void) => indented!(w, [()()()], r"return;")?,
        _ => indented!(w, [()()()], r"return default;")?,
    }
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"// Invokes the callback.")?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public {rval_safe} Call({params_str})")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var __target = Marshal.GetDelegateForFunctionPointer<{name}Native>(_ptr);")?;
    indented!(w, [()()], r"// TODO")?;
    if the_type.fnpointer().signature().rval().is_void() {
        indented!(w, [()()], r"// __target({params_invoke});")?;
    } else {
        indented!(w, [()()], r"// return __target({params_invoke});")?;
    }
    match the_type.fnpointer().signature().rval() {
        Type::Primitive(Primitive::Void) => indented!(w, [()()], r"return;")?,
        _ => indented!(w, [()()], r"return default;")?,
    }
    indented!(w, [()], r"}}")?;
    w.newline()?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public void Dispose()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"// This means when the callback was invoked from Rust C# had an exception which")?;
    indented!(w, [()()], r"// we caught (otherwise C# might not re-enter Rust, and we leak memory). Now is")?;
    indented!(w, [()()], r"// the time to rethrow it.")?;
    indented!(w, [()()], r"if (_exception != null) throw _exception;")?;
    indented!(w, [()], r"}}")?;
    indented!(w, r"public Unmanaged ToUnmanaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var marshaller = new Marshaller(this);")?;
    indented!(w, [()], r"try {{ return marshaller.ToUnmanaged(); }}")?;
    indented!(w, [()], r"finally {{ marshaller.Free(); }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, [()], r"[CustomMarshaller(typeof({name}), MarshalMode.Default, typeof(Marshaller))]")?;
    indented!(w, [()], r"private struct MarshallerMeta {{  }}")?;
    w.newline()?;
    indented!(w, [()], r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, [()], r"public struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"internal IntPtr Callback;")?;
    indented!(w, [()()], r"internal IntPtr Data;")?;
    indented!(w, [()()], r"public {name} ToManaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"var marshaller = new Marshaller(this);")?;
    indented!(w, [()()()], r"try {{ return marshaller.ToManaged(); }}")?;
    indented!(w, [()()()], r"finally {{ marshaller.Free(); }}")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;

    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public ref struct Marshaller")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"private {name} _managed;")?;
    indented!(w, [()()], r"private Unmanaged _unmanaged;")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public Marshaller({name} managed) {{ _managed = managed; }}")?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public Marshaller(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public void FromManaged({name} managed) {{ _managed = managed; }}")?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public Unmanaged ToUnmanaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"_unmanaged = new Unmanaged();")?;
    indented!(w, [()()()], r"_unmanaged.Callback = _managed?._ptr ?? IntPtr.Zero;")?;
    indented!(w, [()()()], r"_unmanaged.Data = IntPtr.Zero;")?;
    indented!(w, [()()()], r"return _unmanaged;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public {name} ToManaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"_managed = new {name}();")?;
    indented!(w, [()()()], r"_managed._ptr = _unmanaged.Callback;")?;
    indented!(w, [()()()], r"return _managed;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public void Free() {{ }}")?;
    indented!(w, [()], r"}}")?; // Close ref struct Marshaller.
    indented!(w, r"}}")?;
    w.newline()?;

    Ok(())
}
