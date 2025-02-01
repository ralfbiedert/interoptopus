use crate::converter::{is_blittable, to_typespecifier_in_rval};
use crate::Interop;
use interoptopus::lang::c::CompositeType;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_pattern_slice(i: &Interop, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
    i.debug(w, "write_pattern_slice")?;

    let context_type_name = slice.rust_name();
    let data_type = slice
        .fields()
        .iter()
        .find(|x| x.name().contains("data"))
        .expect("Slice must contain field called 'data'.")
        .the_type()
        .try_deref_pointer()
        .expect("data must be a pointer type");

    let type_string = to_typespecifier_in_rval(data_type);
    let is_blittable = is_blittable(data_type);

    indented!(
        w,
        r"{} partial struct {} : IEnumerable<{}>",
        i.visibility_types.to_access_modifier(),
        context_type_name,
        type_string
    )?;
    indented!(w, r"{{")?;

    // Ctor
    indented!(w, [()], r"public {}(GCHandle handle, ulong count)", context_type_name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"this.data = handle.AddrOfPinnedObject();")?;
    indented!(w, [()()], r"this.len = count;")?;
    indented!(w, [()], r"}}")?;

    // Ctor
    indented!(w, [()], r"public {}(IntPtr handle, ulong count)", context_type_name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"this.data = handle;")?;
    indented!(w, [()()], r"this.len = count;")?;
    indented!(w, [()], r"}}")?;

    write_pattern_slice_overload(w, context_type_name, &type_string)?;

    // Getter
    indented!(w, [()], r"public unsafe {} this[int i]", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"get")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;

    if is_blittable {
        indented!(w, [()()()], r"var d = ({}*) data.ToPointer();", type_string)?;
        indented!(w, [()()()], r"return d[i];")?;
    } else {
        indented!(w, [()()()], r"var size = Marshal.SizeOf(typeof({}));", type_string)?;
        indented!(w, [()()()], r"var ptr = new IntPtr(data.ToInt64() + i * size);")?;
        indented!(w, [()()()], r"return Marshal.PtrToStructure<{}>(ptr);", type_string)?;
    }

    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;

    // Copied
    indented!(w, [()], r"public unsafe {}[] Copied", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"get")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"var rval = new {}[len];", type_string)?;

    if is_blittable {
        indented!(w, [()()()], r"fixed (void* dst = rval)")?;
        indented!(w, [()()()], r"{{")?;
        indented!(
            w,
            [()()()()],
            r"Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof({}));",
            type_string
        )?;
        indented!(w, [()()()()], r"for (var i = 0; i < (int) len; i++) {{")?;
        indented!(w, [()()()()()], r"rval[i] = this[i];")?;
        indented!(w, [()()()()], r"}}")?;
        indented!(w, [()()()], r"}}")?;
    } else {
        indented!(w, [()()()], r"for (var i = 0; i < (int) len; i++) {{")?;
        indented!(w, [()()()()], r"rval[i] = this[i];")?;
        indented!(w, [()()()], r"}}")?;
    }
    indented!(w, [()()()], r"return rval;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;

    // Count
    indented!(w, [()], r"public int Count => (int) len;")?;

    // GetEnumerator
    indented!(w, [()], r"public IEnumerator<{}> GetEnumerator()", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"for (var i = 0; i < (int)len; ++i)")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"yield return this[i];")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;

    // The other GetEnumerator
    indented!(w, [()], r"IEnumerator IEnumerable.GetEnumerator()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"return this.GetEnumerator();")?;
    indented!(w, [()], r"}}")?;

    indented!(w, r"}}")?;
    w.newline()?;

    Ok(())
}

pub fn write_pattern_slice_overload(w: &mut IndentWriter, _context_type_name: &str, type_string: &str) -> Result<(), Error> {
    indented!(w, [()], r"public unsafe ReadOnlySpan<{}> ReadOnlySpan", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"get")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"unsafe")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"return new ReadOnlySpan<{}>(this.data.ToPointer(), (int) this.len);", type_string)?;
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    Ok(())
}

pub fn write_pattern_slice_mut_overload(w: &mut IndentWriter, _context_type_name: &str, type_string: &str) -> Result<(), Error> {
    indented!(w, [()], r"public unsafe Span<{}> Span", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"get")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"unsafe")?;
    indented!(w, [()()()], r"{{")?;
    indented!(w, [()()()()], r"return new Span<{}>(this.data.ToPointer(), (int) this.len);", type_string)?;
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    Ok(())
}

pub fn write_pattern_slice_mut(i: &Interop, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
    i.debug(w, "write_pattern_slice_mut")?;
    let context_type_name = slice.rust_name();
    let data_type = slice
        .fields()
        .iter()
        .find(|x| x.name().contains("data"))
        .expect("Slice must contain field called 'data'.")
        .the_type()
        .try_deref_pointer()
        .expect("data must be a pointer type");

    let type_string = to_typespecifier_in_rval(data_type);

    indented!(
        w,
        r"{} partial struct {} : IEnumerable<{}>",
        i.visibility_types.to_access_modifier(),
        context_type_name,
        type_string
    )?;
    indented!(w, r"{{")?;

    // Ctor
    indented!(w, [()], r"public {}(GCHandle handle, ulong count)", context_type_name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"this.data = handle.AddrOfPinnedObject();")?;
    indented!(w, [()()], r"this.len = count;")?;
    indented!(w, [()], r"}}")?;

    // Ctor
    indented!(w, [()], r"public {}(IntPtr handle, ulong count)", context_type_name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"this.data = handle;")?;
    indented!(w, [()()], r"this.len = count;")?;
    indented!(w, [()], r"}}")?;

    write_pattern_slice_overload(w, context_type_name, &type_string)?;
    write_pattern_slice_mut_overload(w, context_type_name, &type_string)?;

    // Getter
    indented!(w, [()], r"public unsafe {} this[int i]", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"get")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()()()], r"var d = ({}*) data.ToPointer();", type_string)?;
    indented!(w, [()()()], r"return d[i];")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()()], r"set")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()()()], r"var d = ({}*) data.ToPointer();", type_string)?;
    indented!(w, [()()()], r"d[i] = value;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;

    // Copied
    indented!(w, [()], r"public unsafe {}[] Copied", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"get")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"var rval = new {}[len];", type_string)?;
    indented!(w, [()()()], r"fixed (void* dst = rval)")?;
    indented!(w, [()()()], r"{{")?;
    indented!(
        w,
        [()()()()],
        r"Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof({}));",
        type_string
    )?;
    indented!(w, [()()()()], r"for (var i = 0; i < (int) len; i++) {{")?;
    indented!(w, [()()()()()], r"rval[i] = this[i];")?;
    indented!(w, [()()()()], r"}}")?;
    indented!(w, [()()()], r"}}")?;
    indented!(w, [()()()], r"return rval;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;

    // Count
    indented!(w, [()], r"public int Count => (int) len;")?;

    // GetEnumerator
    indented!(w, [()], r"public IEnumerator<{}> GetEnumerator()", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"for (var i = 0; i < (int)len; ++i)")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"yield return this[i];")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;

    // The other GetEnumerator
    indented!(w, [()], r"IEnumerator IEnumerable.GetEnumerator()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"return this.GetEnumerator();")?;
    indented!(w, [()], r"}}")?;

    indented!(w, r"}}")?;
    w.newline()?;

    Ok(())
}
