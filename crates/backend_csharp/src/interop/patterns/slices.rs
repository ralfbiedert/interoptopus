use crate::Interop;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_pattern_read_only_span_marshaller(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_pattern_read_only_span_marshaller")?;

    write_pattern_generic_slice(i, w, true)?;
    w.newline()?;
    write_pattern_generic_slice_marshaller(i, w, true)
}

pub fn write_pattern_span_marshaller(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_pattern_read_only_span_marshaller")?;

    write_pattern_generic_slice(i, w, false)?;
    w.newline()?;
    write_pattern_generic_slice_marshaller(i, w, false)
}

fn write_pattern_generic_slice(i: &Interop, w: &mut IndentWriter, read_only: bool) -> Result<(), Error> {
    i.debug(w, "write_pattern_generic_slice")?;

    let struct_name = if read_only { "Slice" } else { "SliceMut" };

    indented!(w, r"[NativeMarshalling(typeof({}Marshaller<>))]", struct_name)?;
    indented!(w, r"public readonly partial struct {}<T> : IEnumerable<T> where T : struct", struct_name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"internal readonly T[]? Managed;")?;
    indented!(w, [()], r"internal readonly IntPtr Data;")?;
    indented!(w, [()], r"internal readonly ulong Len;")?;
    w.newline()?;
    indented!(w, [()], r"public int Count => Managed?.Length ?? (int)Len;")?;
    w.newline()?;
    indented!(w, [()], r"public unsafe ReadOnlySpan<T> ReadOnlySpan")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"get")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"if (Managed is not null)")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"return new ReadOnlySpan<T>(Managed);")?;
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()()], r"return new ReadOnlySpan<T>(Data.ToPointer(), (int)Len);")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    if !read_only {
        indented!(w, [()], r"public unsafe Span<T> Span")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"get")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"if (Managed is not null)")?;
        indented!(w, [()()()], r"{{")?;
        indented!(w, [()()()()], r"return new Span<T>(Managed);")?;
        indented!(w, [()()()], r"}}")?;
        indented!(w, [()()()], r"return new Span<T>(Data.ToPointer(), (int)Len);")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;
    }
    indented!(w, [()], r"public unsafe T this[int i]")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"get")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()()()], r"if (Managed is not null)")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"return Managed[i];")?;
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()()], r"return Unsafe.Read<T>((void*)IntPtr.Add(Data, i * Unsafe.SizeOf<T>()));")?;
    indented!(w, [()()], r"}}")?;
    if !read_only {
        indented!(w, [()()], r"set")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
        indented!(w, [()()()], r"if (Managed is not null)")?;
        indented!(w, [()()()], r"{{")?;
        indented!(w, [()()()()], r"Managed[i] = value;")?;
        indented!(w, [()()()], r"}}")?;
        indented!(w, [()()()], r"else")?;
        indented!(w, [()()()], r"{{")?;
        indented!(w, [()()()()], r"Unsafe.Write((void*)IntPtr.Add(Data, i * Unsafe.SizeOf<T>()), value);")?;
        indented!(w, [()()()], r"}}")?;
        indented!(w, [()()], r"}}")?;
    }
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public {}(GCHandle handle, ulong count)", struct_name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"this.Data = handle.AddrOfPinnedObject();")?;
    indented!(w, [()()], r"this.Len = count;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public {}(IntPtr handle, ulong count)", struct_name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"this.Data = handle;")?;
    indented!(w, [()()], r"this.Len = count;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public {}(T[] managed)", struct_name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"this.Managed = managed;")?;
    indented!(w, [()()], r"this.Data = IntPtr.Zero;")?;
    indented!(w, [()()], r"this.Len = 0;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public IEnumerator<T> GetEnumerator()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"for (var i = 0; i < Count; ++i)")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"yield return this[i];")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"IEnumerator IEnumerable.GetEnumerator()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"return GetEnumerator();")?;
    indented!(w, [()], r"}}")?;
    indented!(w, r"}}")?;

    Ok(())
}

#[allow(clippy::too_many_lines)]
fn write_pattern_generic_slice_marshaller(i: &Interop, w: &mut IndentWriter, read_only: bool) -> Result<(), Error> {
    i.debug(w, "write_pattern_generic_slice_marshaller")?;

    if read_only {
        indented!(w, r"[CustomMarshaller(typeof(Slice<>), MarshalMode.Default, typeof(SliceMarshaller<>.Marshaller))]")?;
        indented!(w, r"internal static class SliceMarshaller<T> where T: struct")?;
    } else {
        indented!(
            w,
            r"[CustomMarshaller(typeof(SliceMut<>), MarshalMode.Default, typeof(SliceMutMarshaller<>.Marshaller))]"
        )?;
        indented!(w, r"internal static class SliceMutMarshaller<T> where T: struct")?;
    }
    indented!(w, r"{{")?;
    indented!(w, [()], r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, [()], r"public unsafe struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"public IntPtr Data;")?;
    indented!(w, [()()], r"public ulong Len;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public ref struct Marshaller")?;
    indented!(w, [()], r"{{")?;
    if read_only {
        indented!(w, [()()], r"private Slice<T> managed;")?;
    } else {
        indented!(w, [()()], r"private SliceMut<T> managed;")?;
    }
    indented!(w, [()()], r"private Unmanaged native;")?;
    indented!(w, [()()], r"private Unmanaged sourceNative;")?;
    indented!(w, [()()], r"private GCHandle? pinned;")?;
    indented!(w, [()()], r"private T[] marshalled;")?;
    w.newline()?;
    if read_only {
        indented!(w, [()()], r"public void FromManaged(Slice<T> managed)")?;
    } else {
        indented!(w, [()()], r"public void FromManaged(SliceMut<T> managed)")?;
    }
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"this.managed = managed;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"public unsafe Unmanaged ToUnmanaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"if(managed.Count == 0)")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"return default;")?;
    indented!(w, [()()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()()], r"if (CustomMarshallerHelper<T>.HasCustomMarshaller)")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"var count = managed.Count;")?;
    indented!(w, [()()()()], r"var size = CustomMarshallerHelper<T>.UnmanagedSize;")?;
    indented!(w, [()()()()], r"native.Len = (ulong)count;")?;
    indented!(w, [()()()()], r"native.Data = Marshal.AllocHGlobal(count * size);")?;
    indented!(w, [()()()()], r"for (var i = 0; i < count; i++)")?;
    indented!(w, [()()()()], r"{{")?;
    indented!(
        w,
        [()()()()()],
        r"CustomMarshallerHelper<T>.ToUnmanagedFunc!( managed[i], IntPtr.Add(native.Data, i * size));"
    )?;
    indented!(w, [()()()()], r"}}")?;
    indented!(w, [()()()()], r"return native;")?;
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()()], r"else if(managed.Managed is not null)")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"pinned = GCHandle.Alloc(managed.Managed, GCHandleType.Pinned);")?;
    indented!(w, [()()()()], r"return new Unmanaged")?;
    indented!(w, [()()()()], r"{{")?;
    indented!(w, [()()()()()], r"Data = pinned.Value.AddrOfPinnedObject(),")?;
    indented!(w, [()()()()()], r"Len = (ulong)managed.Count")?;
    indented!(w, [()()()()], r"}};")?;
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()()], r"else")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"return new Unmanaged")?;
    indented!(w, [()()()()], r"{{")?;
    indented!(w, [()()()()()], r"Data = (IntPtr)managed.Data,")?;
    indented!(w, [()()()()()], r"Len = (ulong)managed.Len")?;
    indented!(w, [()()()()], r"}};")?;
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"public void FromUnmanaged(Unmanaged unmanaged)")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"sourceNative = unmanaged;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    if read_only {
        indented!(w, [()()], r"public unsafe Slice<T> ToManaged()")?;
    } else {
        indented!(w, [()()], r"public unsafe SliceMut<T> ToManaged()")?;
    }
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"if (CustomMarshallerHelper<T>.HasCustomMarshaller)")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"var count = (int)sourceNative.Len;")?;
    indented!(w, [()()()()], r"var size = CustomMarshallerHelper<T>.UnmanagedSize;")?;
    indented!(w, [()()()()], r"marshalled = new T[count];")?;
    indented!(w, [()()()()], r"for (var i = 0; i < count; i++)")?;
    indented!(w, [()()()()], r"{{")?;
    indented!(
        w,
        [()()()()()],
        r"marshalled[i] = CustomMarshallerHelper<T>.ToManagedFunc!(IntPtr.Add(sourceNative.Data, i * size));"
    )?;
    indented!(w, [()()()()], r"}}")?;
    if read_only {
        indented!(w, [()()()()], r"return new Slice<T>(marshalled);")?;
    } else {
        indented!(w, [()()()()], r"return new SliceMut<T>(marshalled);")?;
    }
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()()], r"else")?;
    indented!(w, [()()()], r"{{")?;
    if read_only {
        indented!(w, [()()()()], r"return new Slice<T>(sourceNative.Data, sourceNative.Len);")?;
    } else {
        indented!(w, [()()()()], r"return new SliceMut<T>(sourceNative.Data, sourceNative.Len);")?;
    }
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"public void Free()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"if (native.Data != IntPtr.Zero)")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"Marshal.FreeHGlobal(native.Data);")?;
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()()], r"pinned?.Free();")?;
    indented!(w, [()()], r"}}")?;
    if !read_only {
        w.newline()?;
        indented!(w, [()()], r"public void OnInvoked()")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"if (CustomMarshallerHelper<T>.HasCustomMarshaller)")?;
        indented!(w, [()()()], r"{{")?;
        indented!(w, [()()()()], r"if (marshalled is not null)")?;
        indented!(w, [()()()()], r"{{")?;
        indented!(w, [()()()()()], r"var count = marshalled.Length;")?;
        indented!(w, [()()()()()], r"var size = CustomMarshallerHelper<T>.UnmanagedSize;")?;
        indented!(w, [()()()()()], r"for (var i = 0; i < count; i++)")?;
        indented!(w, [()()()()()], r"{{")?;
        indented!(
            w,
            [()()()()()()],
            r"CustomMarshallerHelper<T>.ToUnmanagedFunc!(marshalled[i], IntPtr.Add(sourceNative.Data, i * size));"
        )?;
        indented!(w, [()()()()()], r"}}")?;
        indented!(w, [()()()()], r"}}")?;
        indented!(w, [()()()()], r"else if (native.Data != IntPtr.Zero)")?;
        indented!(w, [()()()()], r"{{")?;
        indented!(w, [()()()()()], r"var count = managed.Count;")?;
        indented!(w, [()()()()()], r"var size = CustomMarshallerHelper<T>.UnmanagedSize;")?;
        indented!(w, [()()()()()], r"for (var i = 0; i < count; i++)")?;
        indented!(w, [()()()()()], r"{{")?;
        indented!(
            w,
            [()()()()()()],
            r"managed[i] = (T)CustomMarshallerHelper<T>.ToManagedFunc!(IntPtr.Add(native.Data, i * size));"
        )?;
        indented!(w, [()()()()()], r"}}")?;
        indented!(w, [()()()()], r"}}")?;
        indented!(w, [()()()], r"}}")?;
        indented!(w, [()()], r"}}")?;
    }
    indented!(w, [()], r"}}")?;
    indented!(w, r"}}")?;

    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn write_pattern_generic_slice_helper(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_pattern_generic_slice_helper")?;

    indented!(
        w,
        r"// This is a helper for the marshallers for Slice<T> and SliceMut<T> of Ts that require custom marshalling."
    )?;
    indented!(w, r"// It is used to precompile the conversion logic for the custom marshaller.")?;
    indented!(w, r"internal static class CustomMarshallerHelper<T> where T : struct")?;
    indented!(w, r"{{")?;
    w.indent();

    indented!(w, r"// Delegate to convert a managed T to its unmanaged representation at the given pointer.")?;
    indented!(w, r"// Precompiling these conversions minimizes overhead during runtime marshalling.")?;
    indented!(w, r"public static readonly Action<T, IntPtr> ToUnmanagedFunc;")?;
    indented!(w, r"// Delegate that converts unmanaged data at a specified pointer back to a managed instance of T.")?;
    indented!(w, r"public static readonly Func<IntPtr, T> ToManagedFunc;")?;
    w.newline()?;
    indented!(w, r"// Indicates whether type T is decorated with a NativeMarshallingAttribute.")?;
    indented!(w, r"public static readonly bool HasCustomMarshaller;")?;
    indented!(w, r"// Size of the unmanaged type in bytes. This is used for memory allocation.")?;
    indented!(w, r"public static readonly int UnmanagedSize;")?;
    indented!(w, r"// The unmanaged type that corresponds to T as defined by the custom marshaller.")?;
    indented!(w, r"// This assumes that the custom marshaller has a nested type named 'Unmanaged'.")?;
    indented!(w, r"public static readonly Type UnmanagedType;")?;
    w.newline()?;
    indented!(w, r"// This runs once per type T, ensuring that the conversion logic is set up only once.")?;
    indented!(w, r"static CustomMarshallerHelper()")?;
    indented!(w, r"{{")?;
    w.indent();

    indented!(w, r"var nativeMarshalling = typeof(T).GetCustomAttribute<NativeMarshallingAttribute>();")?;
    indented!(w, r"if (nativeMarshalling != null)")?;
    indented!(w, r"{{")?;
    w.indent();

    indented!(w, r"var marshallerType = nativeMarshalling.NativeType;")?;
    indented!(
        w,
        r#"var convertToUnmanaged = marshallerType.GetMethod("ConvertToUnmanaged", BindingFlags.Public | BindingFlags.Static);"#
    )?;
    indented!(
        w,
        r#"var convertToManaged = marshallerType.GetMethod("ConvertToManaged", BindingFlags.Public | BindingFlags.Static);"#
    )?;
    indented!(w, r#"UnmanagedType = marshallerType.GetNestedType("Unmanaged")!;"#)?;
    indented!(w, r"UnmanagedSize = Marshal.SizeOf(UnmanagedType);")?;
    w.newline()?;
    indented!(
        w,
        r"// If the stateless custom marshaller shape is not available we currently do not support marshalling T in a slice."
    )?;
    indented!(w, r"if (convertToUnmanaged == null || convertToManaged == null)")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(
        w,
        r"ToUnmanagedFunc = Expression.Lambda<Action<T, IntPtr>>(Expression.Throw(Expression.New(typeof(NotSupportedException))), Expression.Parameter(typeof(T)), Expression.Parameter(typeof(IntPtr))).Compile();"
    )?;
    indented!(
        w,
        r"ToManagedFunc = Expression.Lambda<Func<IntPtr, T>>(Expression.Throw(Expression.New(typeof(NotSupportedException)), typeof(T)), Expression.Parameter(typeof(IntPtr))).Compile();"
    )?;
    w.unindent();
    indented!(w, r"}}")?;
    indented!(w, r"else")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(
        w,
        r"var unsafeRead = typeof(CustomMarshallerHelper<T>).GetMethod(nameof(ReadPointer), BindingFlags.NonPublic | BindingFlags.Static)!.MakeGenericMethod(UnmanagedType)!;"
    )?;
    indented!(w, r"var parameter = Expression.Parameter(typeof(IntPtr));")?;
    indented!(w, r"var unsafeCall = Expression.Call(unsafeRead, parameter);")?;
    indented!(w, r"var callExpression = Expression.Call(convertToManaged, unsafeCall);")?;
    indented!(w, r"ToManagedFunc = Expression.Lambda<Func<IntPtr, T>>(callExpression, parameter).Compile();")?;
    w.newline()?;
    indented!(
        w,
        r" var unsafeWrite = typeof(CustomMarshallerHelper<T>).GetMethod(nameof(WritePointer), BindingFlags.NonPublic | BindingFlags.Static)!.MakeGenericMethod(UnmanagedType)!;"
    )?;
    indented!(w, r"var managedParameter = Expression.Parameter(typeof(T));")?;
    indented!(w, r"var destParameter = Expression.Parameter(typeof(IntPtr));")?;
    indented!(w, r"var toUnmanagedCall = Expression.Call(convertToUnmanaged, managedParameter);")?;
    indented!(w, r"var unsafeWriteCall = Expression.Call(unsafeWrite, toUnmanagedCall, destParameter);")?;
    indented!(
        w,
        r"ToUnmanagedFunc = Expression.Lambda<Action<T, IntPtr>>(unsafeWriteCall, managedParameter, destParameter).Compile();"
    )?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"HasCustomMarshaller = true;")?;
    w.unindent();
    indented!(w, r"}}")?;
    indented!(w, r"else")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"UnmanagedType = typeof(T);")?;
    indented!(
        w,
        r"ToUnmanagedFunc = Expression.Lambda<Action<T, IntPtr>>(Expression.Throw(Expression.New(typeof(InvalidOperationException))), Expression.Parameter(typeof(T)), Expression.Parameter(typeof(IntPtr))).Compile();"
    )?;
    indented!(
        w,
        r"ToManagedFunc = Expression.Lambda<Func<IntPtr, T>>(Expression.Throw(Expression.New(typeof(InvalidOperationException)), typeof(T)), Expression.Parameter(typeof(IntPtr))).Compile();"
    )?;
    indented!(w, r"HasCustomMarshaller = false;")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, r"// This exists to simplify the creation of the expression tree.")?;
    indented!(w, r"private static void WritePointer<TUnmanaged>(TUnmanaged unmanaged, IntPtr dest)")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"unsafe {{ Unsafe.Write((void*)dest, unmanaged); }}")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, r"// This exists to simplify the creation of the expression tree.")?;
    indented!(w, r"private static TUnmanaged ReadPointer<TUnmanaged>(IntPtr ptr)")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r" unsafe {{ return Unsafe.Read<TUnmanaged>((void*)ptr); }}")?;
    w.unindent();
    indented!(w, r"}}")?;

    w.unindent();
    indented!(w, r"}}")?;
    Ok(())
}
