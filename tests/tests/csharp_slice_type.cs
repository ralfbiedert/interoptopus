// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Text;
using System.Threading.Tasks;
using System.Reflection;
using System.Linq.Expressions;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.Marshalling;
using System.Runtime.CompilerServices;
using My.Company;
#pragma warning restore 0105

namespace My.Company
{
    public static partial class Interop
    {
        public const string NativeLib = "library";

        static Interop()
        {
        }


        [LibraryImport(NativeLib, EntryPoint = "sample_function")]
        public static partial void sample_function(SliceU8 ignored);

        public static unsafe void sample_function(ReadOnlySpan<byte> ignored)
        {
            fixed (void* ptr_ignored = ignored)
            {
                var ignored_slice = new SliceU8(new IntPtr(ptr_ignored), (ulong) ignored.Length);
                try
                {
                    sample_function(ignored_slice);
                }
                finally
                {
                }
            }
        }

    }

    public partial struct SliceU8
    {
        byte[] _managed;
        IntPtr _data;
        ulong _len;
        bool _wePinned;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct SliceU8 : IEnumerable<byte>, IDisposable
    {
        public int Count => _managed?.Length ?? (int)_len;

        public unsafe ReadOnlySpan<byte> ReadOnlySpan
        {
            get
            {
                if (_managed is not null)
                {
                    return new ReadOnlySpan<byte>(_managed);
                }
                return new ReadOnlySpan<byte>(_data.ToPointer(), (int)_len);
            }
        }

        public unsafe byte this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                if (_managed is not null)
                {
                    return _managed[i];
                }
                return Unsafe.Read<byte>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<byte>()));
            }
        }

        public SliceU8(GCHandle handle, ulong count)
        {
            _data = handle.AddrOfPinnedObject();
            _len = count;
        }

        public SliceU8(IntPtr handle, ulong count)
        {
            _data = handle;
            _len = count;
        }

        public SliceU8(byte[] managed)
        {
            _managed = managed;
            _data = GCHandle.Alloc(managed, GCHandleType.Pinned).AddrOfPinnedObject();
            _len = (ulong) managed.Length;
            _wePinned = true;
        }

        public IEnumerator<byte> GetEnumerator()
        {
            for (var i = 0; i < Count; ++i)
            {
                yield return this[i];
            }
        }

        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

        public void Dispose()
        {
            if (_wePinned && _data != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(_data);
                _data = IntPtr.Zero;
            }
            _managed = null;
        }

        [CustomMarshaller(typeof(SliceU8), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            public IntPtr Data;
            public ulong Len;

            public SliceU8 Managed()
            {
                return new SliceU8(Data, Len);
            }
        }

        public ref struct Marshaller
        {
            private SliceU8 managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;
            private SliceU8 marshalled;

            public void FromManaged(SliceU8 managed) { this.managed = managed; }
            public Unmanaged ToUnmanaged() => new Unmanaged { Data = managed._data, Len = managed._len };
            public void FromUnmanaged(Unmanaged unmanaged) { sourceNative = unmanaged; }
            public unsafe SliceU8 ToManaged() => new SliceU8(sourceNative.Data, sourceNative.Len);
            public void Free() { }
        }
    }



    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void AsyncHelperNative(IntPtr data, IntPtr callback_data);
    public delegate void AsyncHelperDelegate(IntPtr data);

    public partial struct AsyncHelper
    {
        private AsyncHelperDelegate _managed;
        private AsyncHelperNative _native;
        private IntPtr _ptr;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct AsyncHelper : IDisposable
    {
        public AsyncHelper() { }

        public AsyncHelper(AsyncHelperDelegate managed)
        {
            _managed = managed;
            _native = Call;
            _ptr = Marshal.GetFunctionPointerForDelegate(_native);
        }

        void Call(IntPtr data, IntPtr _)
        {
            _managed(data);
        }

        public void Dispose()
        {
            if (_ptr == IntPtr.Zero) return;
            Marshal.FreeHGlobal(_ptr);
            _ptr = IntPtr.Zero;
        }

        [CustomMarshaller(typeof(AsyncHelper), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            internal IntPtr Callback;
            internal IntPtr Data;
        }

        public ref struct Marshaller
        {
            private AsyncHelper _managed;
            private Unmanaged _unmanaged;

            public void FromManaged(AsyncHelper managed) { _managed = managed; }
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public Unmanaged ToUnmanaged()
            {
                _unmanaged = new Unmanaged();
                _unmanaged.Callback = _managed._ptr;
                _unmanaged.Data = IntPtr.Zero;
                return _unmanaged;
            }

            public AsyncHelper ToManaged()
            {
                _managed = new AsyncHelper();
                _managed._ptr = _unmanaged.Callback;
                return _managed;
            }

            public void Free() { }
        }
    }
    public partial struct Utf8String
    {
        string _s;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct Utf8String: IDisposable
    {
        public Utf8String(string s) { _s = s; }

        public string String => _s;

        public void Dispose() { }

        /// A highly dangerous 'use once type' that has ownership semantics!
        /// Once passed over an FFI boundary 'the other side' is meant to own
        /// (and free) it. Rust handles that fine, but if in C# you put this
        /// in a struct and then call Rust multiple times with that struct
        /// you'll free the same pointer multiple times, and get UB!
        [StructLayout(LayoutKind.Sequential)]
        public unsafe struct Unmanaged
        {
            public IntPtr ptr;
            public ulong len;
            public ulong capacity;
        }

        public partial class InteropHelper
        {
            [LibraryImport(Interop.NativeLib, EntryPoint = "interoptopus_string_create")]
            public static partial long interoptopus_string_create(IntPtr utf8, ulong len, out Unmanaged rval);

            [LibraryImport(Interop.NativeLib, EntryPoint = "interoptopus_string_destroy")]
            public static partial long interoptopus_string_destroy(Unmanaged utf8);
        }

        [CustomMarshaller(typeof(Utf8String), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        public ref struct Marshaller
        {
            private Utf8String _managed; // Used when converting managed -> unmanaged
            private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

            public Marshaller(Utf8String managed) { _managed = managed; }
            public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public void FromManaged(Utf8String managed) { _managed = managed; }
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public unsafe Unmanaged ToUnmanaged()
            {
                var utf8Bytes = Encoding.UTF8.GetBytes(_managed._s);
                var len = utf8Bytes.Length;

                fixed (byte* p = utf8Bytes)
                {
                    InteropHelper.interoptopus_string_create((IntPtr)p, (ulong)len, out var rval);
                    _unmanaged = rval;
                }

                return _unmanaged;
            }

            public unsafe Utf8String ToManaged()
            {
                var span = new ReadOnlySpan<byte>((byte*)_unmanaged.ptr, (int)_unmanaged.len);

                _managed = new Utf8String();
                _managed._s = Encoding.UTF8.GetString(span);

                InteropHelper.interoptopus_string_destroy(_unmanaged);

                return _managed;
            }

            public void Free() { }
        }
    }

}
