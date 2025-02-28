use crate::converter::{function_parameter_to_csharp_typename, named_callback_to_typename, to_typespecifier_in_param, to_typespecifier_in_rval};
use crate::interop::types::fnptrs::write_type_definition_fn_pointer_annotation;
use crate::Interop;
use interoptopus::lang::c::CType;
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_type_definition_named_callback(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    i.debug(w, "write_type_definition_named_callback")?;

    let rval = to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
    let name = named_callback_to_typename(the_type);
    let visibility = i.visibility_types.to_access_modifier();
    let needs_wrapper = i.has_custom_marshalled_types(the_type.fnpointer().signature());

    let mut params = Vec::new();
    let mut params_native = Vec::new();
    let mut param_names = Vec::new();
    let mut param_invokes = Vec::new();
    for param in the_type.fnpointer().signature().params() {
        match param.the_type() {
            CType::Pattern(TypePattern::Slice(_)) => param_invokes.push(format!("{}.Managed()", param.name())),
            CType::Pattern(TypePattern::SliceMut(_)) => param_invokes.push(format!("{}.Managed()", param.name())),
            _ => param_invokes.push(param.name().to_string()),
        }
        param_names.push(param.name());
        params.push(format!("{} {}", to_typespecifier_in_param(param.the_type()), param.name()));
        params_native.push(format!("{} {}", i.to_native_callback_typespecifier(param.the_type()), param.name()));
    }

    params.pop();
    param_names.pop();
    param_invokes.pop();

    write_type_definition_fn_pointer_annotation(w, the_type.fnpointer())?;
    indented!(
        w,
        r"{} delegate {} {}Native({});",
        visibility,
        i.to_native_callback_typespecifier(the_type.fnpointer().signature().rval()),
        name,
        params_native.join(", ")
    )?;
    indented!(w, r"{} delegate {} {}Delegate({});", visibility, rval, name, params.join(", "))?;
    w.newline()?;

    // indented!(w, r"[NativeMarshalling(typeof(CallbackStructMarshaller<{}Native>))]", name)?;
    // indented!(w, r"public class {}: CallbackStruct<{}Native>", name, name)?;
    // indented!(w, r"{{")?;
    // w.indent();
    // indented!(w, r"internal readonly {}Delegate _userCallback;", name)?;
    // w.newline()?;
    // indented!(w, r"public {}({}Delegate userCallback)", name, name)?;
    // indented!(w, r"{{")?;
    // indented!(w, [()], r"_userCallback = userCallback;")?;
    // indented!(w, [()], r"Init(Call);")?;
    // indented!(w, r"}}")?;
    // w.newline()?;
    // indented!(w, r"public {} Call({}, IntPtr _)", rval, params.join(", "))?;
    // indented!(w, r"{{")?;
    // if the_type.fnpointer().signature().rval().is_void() {
    //     indented!(w, [()], r"_userCallback({});", param_names.join(", "))?;
    // } else {
    //     indented!(w, [()], r"return _userCallback({});", param_names.join(", "))?;
    // }
    // indented!(w, r"}}")?;
    // w.unindent();
    // indented!(w, r"}}")?;
    //
    // Write the attribute and struct declaration without manual spaces.

    indented!(w, r"public partial struct {} : IDisposable", name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"private {}Delegate _callbackUser;", name)?;
    indented!(w, [()], r"private IntPtr _callbackNative;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial struct {} : IDisposable", name)?;
    indented!(w, r"{{")?;
    w.newline()?;
    indented!(w, [()], r"public {}() {{ }}", name)?;
    w.newline()?;
    indented!(w, [()], r"public {}({}Delegate callbackUser)", name, name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"_callbackUser = callbackUser;")?;
    indented!(w, [()()], r"_callbackNative = Marshal.GetFunctionPointerForDelegate(new {}Native(Call));", name)?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public {} Call({})", rval, params_native.join(", "))?;
    indented!(w, [()], r"{{")?;
    if the_type.fnpointer().signature().rval().is_void() {
        indented!(w, [()()], r"_callbackUser({});", param_invokes.join(", "))?;
    } else {
        indented!(w, [()()], r"return _callbackUser({});", param_invokes.join(", "))?;
    }
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public void Dispose()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"if (_callbackNative == IntPtr.Zero) return;")?;
    indented!(w, [()()], r"Marshal.FreeHGlobal(_callbackNative);")?;
    indented!(w, [()()], r"_callbackNative = IntPtr.Zero;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
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
    w.newline()?;
    indented!(w, [()], r"public ref struct Marshaller")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"private {} _managed;", name)?;
    indented!(w, [()()], r"private Unmanaged _unmanaged;")?;
    w.newline()?;
    indented!(w, [()()], r"public Marshaller({} managed) {{ _managed = managed; }}", name)?;
    w.newline()?;
    indented!(w, [()()], r"public void FromManaged({} managed) {{ _managed = managed; }}", name)?;
    indented!(w, [()()], r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    indented!(w, [()()], r"public Unmanaged ToUnmanaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"return new Unmanaged")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"Callback = _managed._callbackNative,")?;
    indented!(w, [()()()()], r"Data = IntPtr.Zero")?;
    indented!(w, [()()()], r"}};")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"public {} ToManaged()", name)?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"return new {}", name)?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"_callbackNative = _unmanaged.Callback,")?;
    indented!(w, [()()()], r"}};")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"public void Free() {{ }}")?;
    indented!(w, [()], r"}}")?; // Close ref struct Marshaller.
    indented!(w, r"}}")?;
    w.newline()?;

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

    // Handled by the wrapper
    function_param_names.pop();

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
