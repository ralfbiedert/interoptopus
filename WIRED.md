#[ffi_type(wired)]
struct T{x: X, y: Y}

X must be wired
Y must be wired
But we don't know!

basically every type inside that is not a builtin we need to add to "emit list"
and then check that all items that are there by name only (no impl) also have an impl
(referring X but not having a (wired)X anywhere is an error)

// is Wire<u32> a thing? Most probably not, useless. Wire wraps a struct.


Interop.cs generated wrapper:

        [LibraryImport(NativeLib, EntryPoint = "start_server")]
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        private static partial WireOfReturn start_server(WireOfSomething server_name);

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static Return StartServer(Something s) {
            Span<byte> bytes = stackalloc byte[s.wire_size()];
            fixed (byte* p = bytes)
            {
                WireOfSomething ws = s.Wire(p);
                var wr_buf = start_server(ws); // returns a WireOfReturn constructed on the rs side?
                // might have to convert wr_buf to Managed
                return WireOfReturn.Unwire(wr_buf);
            }
        }





old generated

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static unsafe Utf8String Empty()
        {
            InteropHelper.interoptopus_string_create(IntPtr.Zero, 0, out var _out);
            return _out.IntoManaged();
        }
