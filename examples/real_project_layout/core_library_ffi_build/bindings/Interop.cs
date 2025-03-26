// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Text;
using System.Threading.Tasks;
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
        public const string NativeLib = "core_library";

        static Interop()
        {
        }


        [LibraryImport(NativeLib, EntryPoint = "start_server")]
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static partial void start_server([MarshalAs(UnmanagedType.LPStr)] string server_name);


        /// Destroys the given instance.
        ///
        /// # Safety
        ///
        /// The passed parameter MUST have been created with the corresponding init function;
        /// passing any other value results in undefined behavior.
        [LibraryImport(NativeLib, EntryPoint = "game_engine_destroy")]
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static partial ResultConstPtrGameEngineError game_engine_destroy(IntPtr _context);


        [LibraryImport(NativeLib, EntryPoint = "game_engine_new")]
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static partial ResultConstPtrGameEngineError game_engine_new();


        [LibraryImport(NativeLib, EntryPoint = "game_engine_place_object")]
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static partial ResultError game_engine_place_object(IntPtr _context, [MarshalAs(UnmanagedType.LPStr)] string name, Vec2 position);


        [LibraryImport(NativeLib, EntryPoint = "game_engine_num_objects")]
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static partial uint game_engine_num_objects(IntPtr _context);


    }

    public partial struct Error
    {
        uint _variant;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct Error
    {

        [StructLayout(LayoutKind.Explicit)]
        public unsafe struct Unmanaged
        {
            [FieldOffset(0)]
            internal uint _variant;

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Error ToManaged()
            {
                var marshaller = new Marshaller(this);
                try { return marshaller.ToManaged(); }
                finally { marshaller.Free(); }
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged()
        {
            var marshaller = new Marshaller(this);
            try { return marshaller.ToUnmanaged(); }
            finally { marshaller.Free(); }
        }

        [CustomMarshaller(typeof(Error), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        public static Error Fail => new() { _variant = 0 };

        public bool IsFail => _variant == 0;

        public void AsFail() { if (_variant != 0) throw new InteropException(); }

        public ref struct Marshaller
        {
            private Error _managed; // Used when converting managed -> unmanaged
            private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Marshaller(Error managed) { _managed = managed; }
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromManaged(Error managed) { _managed = managed; }
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public unsafe Unmanaged ToUnmanaged()
            {;
                _unmanaged = new Unmanaged();
                _unmanaged._variant = _managed._variant;
                return _unmanaged;
            }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public unsafe Error ToManaged()
            {
                _managed = new Error();
                _managed._variant = _unmanaged._variant;
                return _managed;
            }
            public void Free() { }
        }
    }

    public partial struct Vec2
    {
        public float x;
        public float y;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct Vec2
    {
        public Vec2(Vec2 other)
        {
            x = other.x;
            y = other.y;
        }

        public Unmanaged ToUnmanaged()
        {
            var marshaller = new Marshaller(this);
            try { return marshaller.ToUnmanaged(); }
            finally { marshaller.Free(); }
        }

        [StructLayout(LayoutKind.Sequential)]
        public unsafe struct Unmanaged
        {
            public float x;
            public float y;

            public Vec2 ToManaged()
            {
                var marshaller = new Marshaller(this);
                try { return marshaller.ToManaged(); }
                finally { marshaller.Free(); }
            }
        }

        [CustomMarshaller(typeof(Vec2), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        public ref struct Marshaller
        {
            private Vec2 _managed; // Used when converting managed -> unmanaged
            private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

            public Marshaller(Vec2 managed) { _managed = managed; }
            public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public void FromManaged(Vec2 managed) { _managed = managed; }
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public unsafe Unmanaged ToUnmanaged()
            {;
                _unmanaged = new Unmanaged();

                _unmanaged.x = _managed.x;
                _unmanaged.y = _managed.y;

                return _unmanaged;
            }

            public unsafe Vec2 ToManaged()
            {
                _managed = new Vec2();

                _managed.x = _unmanaged.x;
                _managed.y = _unmanaged.y;

                return _managed;
            }
            public void Free() { }
        }
    }

    ///Result that contains value or an error.
    public partial struct ResultConstPtrGameEngineError
    {
        uint _variant;
        IntPtr _Ok;
        Error _Err;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct ResultConstPtrGameEngineError
    {
        [StructLayout(LayoutKind.Sequential)]
        internal unsafe struct UnmanagedOk
        {
            internal uint _variant;
            internal IntPtr _Ok;
        }

        [StructLayout(LayoutKind.Sequential)]
        internal unsafe struct UnmanagedErr
        {
            internal uint _variant;
            internal Error.Unmanaged _Err;
        }



        [StructLayout(LayoutKind.Explicit)]
        public unsafe struct Unmanaged
        {
            [FieldOffset(0)]
            internal uint _variant;

            [FieldOffset(0)]
            internal UnmanagedOk _Ok;

            [FieldOffset(0)]
            internal UnmanagedErr _Err;

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public ResultConstPtrGameEngineError ToManaged()
            {
                var marshaller = new Marshaller(this);
                try { return marshaller.ToManaged(); }
                finally { marshaller.Free(); }
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged()
        {
            var marshaller = new Marshaller(this);
            try { return marshaller.ToUnmanaged(); }
            finally { marshaller.Free(); }
        }

        [CustomMarshaller(typeof(ResultConstPtrGameEngineError), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        public static ResultConstPtrGameEngineError Ok(IntPtr value) => new() { _variant = 0, _Ok = value };
        public static ResultConstPtrGameEngineError Err(Error value) => new() { _variant = 1, _Err = value };
        public static ResultConstPtrGameEngineError Panic => new() { _variant = 2 };
        public static ResultConstPtrGameEngineError Null => new() { _variant = 3 };

        public bool IsOk => _variant == 0;
        public bool IsErr => _variant == 1;
        public bool IsPanic => _variant == 2;
        public bool IsNull => _variant == 3;

        public IntPtr AsOk() { if (_variant != 0) { throw new InteropException(); } else { return _Ok; } }
        public Error AsErr() { if (_variant != 1) { throw new InteropException(); } else { return _Err; } }
        public void AsPanic() { if (_variant != 2) throw new InteropException(); }
        public void AsNull() { if (_variant != 3) throw new InteropException(); }

        public ref struct Marshaller
        {
            private ResultConstPtrGameEngineError _managed; // Used when converting managed -> unmanaged
            private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Marshaller(ResultConstPtrGameEngineError managed) { _managed = managed; }
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromManaged(ResultConstPtrGameEngineError managed) { _managed = managed; }
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public unsafe Unmanaged ToUnmanaged()
            {;
                _unmanaged = new Unmanaged();
                _unmanaged._variant = _managed._variant;
                if (_unmanaged._variant == 0) _unmanaged._Ok._Ok = _managed._Ok;
                if (_unmanaged._variant == 1) _unmanaged._Err._Err = _managed._Err.ToUnmanaged();
                return _unmanaged;
            }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public unsafe ResultConstPtrGameEngineError ToManaged()
            {
                _managed = new ResultConstPtrGameEngineError();
                _managed._variant = _unmanaged._variant;
                if (_managed._variant == 0) _managed._Ok = _unmanaged._Ok._Ok;
                if (_managed._variant == 1) _managed._Err = _unmanaged._Err._Err.ToManaged();
                return _managed;
            }
            public void Free() { }
        }
    }

    ///Result that contains value or an error.
    public partial struct ResultError
    {
        uint _variant;
        Error _Err;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct ResultError
    {

        [StructLayout(LayoutKind.Sequential)]
        internal unsafe struct UnmanagedErr
        {
            internal uint _variant;
            internal Error.Unmanaged _Err;
        }



        [StructLayout(LayoutKind.Explicit)]
        public unsafe struct Unmanaged
        {
            [FieldOffset(0)]
            internal uint _variant;

            [FieldOffset(0)]
            internal UnmanagedErr _Err;

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public ResultError ToManaged()
            {
                var marshaller = new Marshaller(this);
                try { return marshaller.ToManaged(); }
                finally { marshaller.Free(); }
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged()
        {
            var marshaller = new Marshaller(this);
            try { return marshaller.ToUnmanaged(); }
            finally { marshaller.Free(); }
        }

        [CustomMarshaller(typeof(ResultError), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        public static ResultError Ok => new() { _variant = 0 };
        public static ResultError Err(Error value) => new() { _variant = 1, _Err = value };
        public static ResultError Panic => new() { _variant = 2 };
        public static ResultError Null => new() { _variant = 3 };

        public bool IsOk => _variant == 0;
        public bool IsErr => _variant == 1;
        public bool IsPanic => _variant == 2;
        public bool IsNull => _variant == 3;

        public void AsOk() { if (_variant != 0) throw new InteropException(); }
        public Error AsErr() { if (_variant != 1) { throw new InteropException(); } else { return _Err; } }
        public void AsPanic() { if (_variant != 2) throw new InteropException(); }
        public void AsNull() { if (_variant != 3) throw new InteropException(); }

        public ref struct Marshaller
        {
            private ResultError _managed; // Used when converting managed -> unmanaged
            private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Marshaller(ResultError managed) { _managed = managed; }
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromManaged(ResultError managed) { _managed = managed; }
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public unsafe Unmanaged ToUnmanaged()
            {;
                _unmanaged = new Unmanaged();
                _unmanaged._variant = _managed._variant;
                if (_unmanaged._variant == 1) _unmanaged._Err._Err = _managed._Err.ToUnmanaged();
                return _unmanaged;
            }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public unsafe ResultError ToManaged()
            {
                _managed = new ResultError();
                _managed._variant = _unmanaged._variant;
                if (_managed._variant == 1) _managed._Err = _unmanaged._Err._Err.ToManaged();
                return _managed;
            }
            public void Free() { }
        }
    }


    public partial class GameEngine : IDisposable
    {
        private IntPtr _context;

        private GameEngine() {}

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static GameEngine New()
        {
            var self = new GameEngine();
            self._context = Interop.game_engine_new().AsOk();
            return self;
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Dispose()
        {
            Interop.game_engine_destroy(_context).AsOk();
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public ResultError PlaceObject([MarshalAs(UnmanagedType.LPStr)] string name, Vec2 position)
        {
            return Interop.game_engine_place_object(_context, name, position);
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public uint NumObjects()
        {
            return Interop.game_engine_num_objects(_context);
        }

        public IntPtr Context => _context;
    }



    public class InteropException: Exception
    {

        public InteropException(): base()
        {
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

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public AsyncHelper(AsyncHelperDelegate managed)
        {
            _managed = managed;
            _native = Call;
            _ptr = Marshal.GetFunctionPointerForDelegate(_native);
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        void Call(IntPtr data, IntPtr _)
        {
            _managed(data);
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromManaged(AsyncHelper managed) { _managed = managed; }
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Unmanaged ToUnmanaged()
            {
                _unmanaged = new Unmanaged();
                _unmanaged.Callback = _managed._ptr;
                _unmanaged.Data = IntPtr.Zero;
                return _unmanaged;
            }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public AsyncHelper ToManaged()
            {
                _managed = new AsyncHelper();
                _managed._ptr = _unmanaged.Callback;
                return _managed;
            }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Utf8String(string s) { _s = s; }

        public string String => _s;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Dispose() { }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged()
        {
            var marshaller = new Marshaller(this);
            try { return marshaller.ToUnmanaged(); }
            finally { marshaller.Free(); }
        }

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

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public string ToManaged()
            {
                var marshaller = new Marshaller(this);
                try { return marshaller.ToManaged().String; }
                finally { marshaller.Free(); }
            }

        }

        public partial class InteropHelper
        {
            [LibraryImport(Interop.NativeLib, EntryPoint = "interoptopus_string_create")]
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public static partial long interoptopus_string_create(IntPtr utf8, ulong len, out Unmanaged rval);

            [LibraryImport(Interop.NativeLib, EntryPoint = "interoptopus_string_destroy")]
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public static partial long interoptopus_string_destroy(Unmanaged utf8);
        }

        [CustomMarshaller(typeof(Utf8String), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        public ref struct Marshaller
        {
            private Utf8String _managed; // Used when converting managed -> unmanaged
            private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Marshaller(Utf8String managed) { _managed = managed; }
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromManaged(Utf8String managed) { _managed = managed; }
            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public unsafe Unmanaged ToUnmanaged()
            {
                var source = _managed._s.AsSpan();
                Span<byte> utf8Bytes = stackalloc byte[Encoding.UTF8.GetByteCount(source)];
                var len = Encoding.UTF8.GetBytes(source, utf8Bytes);

                fixed (byte* p = utf8Bytes)
                {
                    InteropHelper.interoptopus_string_create((IntPtr)p, (ulong)len, out var rval);
                    _unmanaged = rval;
                }

                return _unmanaged;
            }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public unsafe Utf8String ToManaged()
            {
                var span = new ReadOnlySpan<byte>((byte*)_unmanaged.ptr, (int)_unmanaged.len);

                _managed = new Utf8String();
                _managed._s = Encoding.UTF8.GetString(span);

                InteropHelper.interoptopus_string_destroy(_unmanaged);

                return _managed;
            }

            [MethodImpl(MethodImplOptions.AggressiveOptimization)]
            public void Free() { }
        }
    }

        public static class StringExtensions
        {
            public static Utf8String Utf8(this string s) { return new Utf8String(s); }
        }
}
