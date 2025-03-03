// Automatically generated by Interoptopus.

// Debug - write_imports 
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
using My.Company.Common;
#pragma warning restore 0105

// Debug - write_namespace_context 
namespace My.Company.Common
{

    // Debug - write_type_definition_composite 
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Vec
    {
        public double x;
        public double z;
    }

    // Debug - write_type_definition_composite_marshaller 
    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct Vec
    {
        [StructLayout(LayoutKind.Sequential)]
        public unsafe struct Unmanaged
        {
            // Debug - write_type_definition_composite_unmanaged_body_field 
            public double x;
            // Debug - write_type_definition_composite_unmanaged_body_field 
            public double z;
        }

        [CustomMarshaller(typeof(Vec), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        public ref struct Marshaller
        {
            private Vec _managed; // Used when converting managed -> unmanaged
            private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

            public Marshaller(Vec managed) { _managed = managed; }
            public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public void FromManaged(Vec managed) { _managed = managed; }
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public unsafe Unmanaged ToUnmanaged()
            {;
                _unmanaged = new Unmanaged();

                // Debug - write_type_definition_composite_marshaller_unmanaged_invoke 
                _unmanaged.x = _managed.x;
                // Debug - write_type_definition_composite_marshaller_unmanaged_invoke 
                _unmanaged.z = _managed.z;

                return _unmanaged;
            }

            public unsafe Vec ToManaged()
            {
                _managed = new Vec();

                // Debug - write_type_definition_composite_marshaller_field_from_unmanaged 
                _managed.x = _unmanaged.x;
                // Debug - write_type_definition_composite_marshaller_field_from_unmanaged 
                _managed.z = _unmanaged.z;

                return _managed;
            }
            public void Free() { }
        }
    }

    // Debug - write_type_definition_fn_pointer 
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate byte InteropDelegate_fn_u8_rval_u8(byte x0);

    // Debug - write_type_definition_fn_pointer 
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void InteropDelegate_fn_CharArray(CharArray x0);
    public delegate void InteropDelegate_fn_CharArray_native(CharArray x0);

    // Debug - write_pattern_slice 
    public partial struct SliceBool
    {
        Bool[] _managed;
        IntPtr _data;
        ulong _len;
        bool _wePinned;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct SliceBool : IEnumerable<Bool>, IDisposable
    {
        public int Count => _managed?.Length ?? (int)_len;

        public unsafe ReadOnlySpan<Bool> ReadOnlySpan
        {
            get
            {
                if (_managed is not null)
                {
                    return new ReadOnlySpan<Bool>(_managed);
                }
                return new ReadOnlySpan<Bool>(_data.ToPointer(), (int)_len);
            }
        }

        public unsafe Bool this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                if (_managed is not null)
                {
                    return _managed[i];
                }
                return Unsafe.Read<Bool>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<Bool>()));
            }
        }

        public SliceBool(GCHandle handle, ulong count)
        {
            _data = handle.AddrOfPinnedObject();
            _len = count;
        }

        public SliceBool(IntPtr handle, ulong count)
        {
            _data = handle;
            _len = count;
        }

        public SliceBool(Bool[] managed)
        {
            _managed = managed;
            _data = GCHandle.Alloc(managed, GCHandleType.Pinned).AddrOfPinnedObject();
            _len = (ulong) managed.Length;
            _wePinned = true;
        }

        public IEnumerator<Bool> GetEnumerator()
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

        [CustomMarshaller(typeof(SliceBool), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            public IntPtr Data;
            public ulong Len;

            public SliceBool Managed()
            {
                return new SliceBool(Data, Len);
            }
        }

        public ref struct Marshaller
        {
            private SliceBool managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;
            private SliceBool marshalled;

            public void FromManaged(SliceBool managed) { this.managed = managed; }
            public Unmanaged ToUnmanaged() => new Unmanaged { Data = managed._data, Len = managed._len };
            public void FromUnmanaged(Unmanaged unmanaged) { sourceNative = unmanaged; }
            public unsafe SliceBool ToManaged() => new SliceBool(sourceNative.Data, sourceNative.Len);
            public void Free() { }
        }
    }

    // Debug - write_pattern_slice 
    public partial struct SliceI32
    {
        int[] _managed;
        IntPtr _data;
        ulong _len;
        bool _wePinned;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct SliceI32 : IEnumerable<int>, IDisposable
    {
        public int Count => _managed?.Length ?? (int)_len;

        public unsafe ReadOnlySpan<int> ReadOnlySpan
        {
            get
            {
                if (_managed is not null)
                {
                    return new ReadOnlySpan<int>(_managed);
                }
                return new ReadOnlySpan<int>(_data.ToPointer(), (int)_len);
            }
        }

        public unsafe int this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                if (_managed is not null)
                {
                    return _managed[i];
                }
                return Unsafe.Read<int>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<int>()));
            }
        }

        public SliceI32(GCHandle handle, ulong count)
        {
            _data = handle.AddrOfPinnedObject();
            _len = count;
        }

        public SliceI32(IntPtr handle, ulong count)
        {
            _data = handle;
            _len = count;
        }

        public SliceI32(int[] managed)
        {
            _managed = managed;
            _data = GCHandle.Alloc(managed, GCHandleType.Pinned).AddrOfPinnedObject();
            _len = (ulong) managed.Length;
            _wePinned = true;
        }

        public IEnumerator<int> GetEnumerator()
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

        [CustomMarshaller(typeof(SliceI32), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            public IntPtr Data;
            public ulong Len;

            public SliceI32 Managed()
            {
                return new SliceI32(Data, Len);
            }
        }

        public ref struct Marshaller
        {
            private SliceI32 managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;
            private SliceI32 marshalled;

            public void FromManaged(SliceI32 managed) { this.managed = managed; }
            public Unmanaged ToUnmanaged() => new Unmanaged { Data = managed._data, Len = managed._len };
            public void FromUnmanaged(Unmanaged unmanaged) { sourceNative = unmanaged; }
            public unsafe SliceI32 ToManaged() => new SliceI32(sourceNative.Data, sourceNative.Len);
            public void Free() { }
        }
    }

    // Debug - write_pattern_slice 
    public partial struct SliceU32
    {
        uint[] _managed;
        IntPtr _data;
        ulong _len;
        bool _wePinned;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct SliceU32 : IEnumerable<uint>, IDisposable
    {
        public int Count => _managed?.Length ?? (int)_len;

        public unsafe ReadOnlySpan<uint> ReadOnlySpan
        {
            get
            {
                if (_managed is not null)
                {
                    return new ReadOnlySpan<uint>(_managed);
                }
                return new ReadOnlySpan<uint>(_data.ToPointer(), (int)_len);
            }
        }

        public unsafe uint this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                if (_managed is not null)
                {
                    return _managed[i];
                }
                return Unsafe.Read<uint>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<uint>()));
            }
        }

        public SliceU32(GCHandle handle, ulong count)
        {
            _data = handle.AddrOfPinnedObject();
            _len = count;
        }

        public SliceU32(IntPtr handle, ulong count)
        {
            _data = handle;
            _len = count;
        }

        public SliceU32(uint[] managed)
        {
            _managed = managed;
            _data = GCHandle.Alloc(managed, GCHandleType.Pinned).AddrOfPinnedObject();
            _len = (ulong) managed.Length;
            _wePinned = true;
        }

        public IEnumerator<uint> GetEnumerator()
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

        [CustomMarshaller(typeof(SliceU32), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            public IntPtr Data;
            public ulong Len;

            public SliceU32 Managed()
            {
                return new SliceU32(Data, Len);
            }
        }

        public ref struct Marshaller
        {
            private SliceU32 managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;
            private SliceU32 marshalled;

            public void FromManaged(SliceU32 managed) { this.managed = managed; }
            public Unmanaged ToUnmanaged() => new Unmanaged { Data = managed._data, Len = managed._len };
            public void FromUnmanaged(Unmanaged unmanaged) { sourceNative = unmanaged; }
            public unsafe SliceU32 ToManaged() => new SliceU32(sourceNative.Data, sourceNative.Len);
            public void Free() { }
        }
    }

    // Debug - write_pattern_slice 
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

    // Debug - write_pattern_slice 
    public partial struct SliceVec
    {
        Vec[] _managed;
        IntPtr _data;
        ulong _len;
        bool _wePinned;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct SliceVec : IEnumerable<Vec>, IDisposable
    {
        public int Count => _managed?.Length ?? (int)_len;

        public unsafe ReadOnlySpan<Vec> ReadOnlySpan
        {
            get
            {
                if (_managed is not null)
                {
                    return new ReadOnlySpan<Vec>(_managed);
                }
                return new ReadOnlySpan<Vec>(_data.ToPointer(), (int)_len);
            }
        }

        public unsafe Vec this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                if (_managed is not null)
                {
                    return _managed[i];
                }
                return Unsafe.Read<Vec>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<Vec>()));
            }
        }

        public SliceVec(GCHandle handle, ulong count)
        {
            _data = handle.AddrOfPinnedObject();
            _len = count;
        }

        public SliceVec(IntPtr handle, ulong count)
        {
            _data = handle;
            _len = count;
        }

        public SliceVec(Vec[] managed)
        {
            _managed = managed;
            _data = GCHandle.Alloc(managed, GCHandleType.Pinned).AddrOfPinnedObject();
            _len = (ulong) managed.Length;
            _wePinned = true;
        }

        public IEnumerator<Vec> GetEnumerator()
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

        [CustomMarshaller(typeof(SliceVec), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            public IntPtr Data;
            public ulong Len;

            public SliceVec Managed()
            {
                return new SliceVec(Data, Len);
            }
        }

        public ref struct Marshaller
        {
            private SliceVec managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;
            private SliceVec marshalled;

            public void FromManaged(SliceVec managed) { this.managed = managed; }
            public Unmanaged ToUnmanaged() => new Unmanaged { Data = managed._data, Len = managed._len };
            public void FromUnmanaged(Unmanaged unmanaged) { sourceNative = unmanaged; }
            public unsafe SliceVec ToManaged() => new SliceVec(sourceNative.Data, sourceNative.Len);
            public void Free() { }
        }
    }

    // Debug - write_pattern_slice_mut 
    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct SliceMutU32 : IEnumerable<uint>, IDisposable
    {
        uint[] _managed;
        IntPtr _data;
        ulong _len;
        bool _wePinned;

        public int Count => _managed?.Length ?? (int)_len;

        public unsafe ReadOnlySpan<uint> ReadOnlySpan
        {
            get
            {
                if (_managed is not null)
                {
                    return new ReadOnlySpan<uint>(_managed);
                }
                return new ReadOnlySpan<uint>(_data.ToPointer(), (int)_len);
            }
        }

        public unsafe uint this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                if (_managed is not null)
                {
                    return _managed[i];
                }
                return Unsafe.Read<uint>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<uint>()));
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var d = (uint*) _data.ToPointer();
                d[i] = value;
            }
        }

        public SliceMutU32(GCHandle handle, ulong count)
        {
            _data = handle.AddrOfPinnedObject();
            _len = count;
        }

        public SliceMutU32(IntPtr handle, ulong count)
        {
            _data = handle;
            _len = count;
        }

        public SliceMutU32(uint[] managed)
        {
            _managed = managed;
            _data = GCHandle.Alloc(managed, GCHandleType.Pinned).AddrOfPinnedObject();
            _len = (ulong) managed.Length;
            _wePinned = true;
        }

        public IEnumerator<uint> GetEnumerator()
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

        [CustomMarshaller(typeof(SliceMutU32), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            public IntPtr Data;
            public ulong Len;

            public SliceMutU32 Managed()
            {
                return new SliceMutU32(Data, Len);
            }
        }

        public ref struct Marshaller
        {
            private SliceMutU32 managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;
            private SliceMutU32 marshalled;

            public void FromManaged(SliceMutU32 managed) { this.managed = managed; }
            public Unmanaged ToUnmanaged() => new Unmanaged { Data = managed._data, Len = managed._len };
            public void FromUnmanaged(Unmanaged unmanaged) { sourceNative = unmanaged; }
            public unsafe SliceMutU32 ToManaged() => new SliceMutU32(sourceNative.Data, sourceNative.Len);
            public void Free() { }
        }
    }

    // Debug - write_pattern_slice_mut 
    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct SliceMutU8 : IEnumerable<byte>, IDisposable
    {
        byte[] _managed;
        IntPtr _data;
        ulong _len;
        bool _wePinned;

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
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var d = (byte*) _data.ToPointer();
                d[i] = value;
            }
        }

        public SliceMutU8(GCHandle handle, ulong count)
        {
            _data = handle.AddrOfPinnedObject();
            _len = count;
        }

        public SliceMutU8(IntPtr handle, ulong count)
        {
            _data = handle;
            _len = count;
        }

        public SliceMutU8(byte[] managed)
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

        [CustomMarshaller(typeof(SliceMutU8), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            public IntPtr Data;
            public ulong Len;

            public SliceMutU8 Managed()
            {
                return new SliceMutU8(Data, Len);
            }
        }

        public ref struct Marshaller
        {
            private SliceMutU8 managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;
            private SliceMutU8 marshalled;

            public void FromManaged(SliceMutU8 managed) { this.managed = managed; }
            public Unmanaged ToUnmanaged() => new Unmanaged { Data = managed._data, Len = managed._len };
            public void FromUnmanaged(Unmanaged unmanaged) { sourceNative = unmanaged; }
            public unsafe SliceMutU8 ToManaged() => new SliceMutU8(sourceNative.Data, sourceNative.Len);
            public void Free() { }
        }
    }

    // Debug - write_pattern_slice_mut 
    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct SliceMutVec : IEnumerable<Vec>, IDisposable
    {
        Vec[] _managed;
        IntPtr _data;
        ulong _len;
        bool _wePinned;

        public int Count => _managed?.Length ?? (int)_len;

        public unsafe ReadOnlySpan<Vec> ReadOnlySpan
        {
            get
            {
                if (_managed is not null)
                {
                    return new ReadOnlySpan<Vec>(_managed);
                }
                return new ReadOnlySpan<Vec>(_data.ToPointer(), (int)_len);
            }
        }

        public unsafe Vec this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                if (_managed is not null)
                {
                    return _managed[i];
                }
                return Unsafe.Read<Vec>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<Vec>()));
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var d = (Vec*) _data.ToPointer();
                d[i] = value;
            }
        }

        public SliceMutVec(GCHandle handle, ulong count)
        {
            _data = handle.AddrOfPinnedObject();
            _len = count;
        }

        public SliceMutVec(IntPtr handle, ulong count)
        {
            _data = handle;
            _len = count;
        }

        public SliceMutVec(Vec[] managed)
        {
            _managed = managed;
            _data = GCHandle.Alloc(managed, GCHandleType.Pinned).AddrOfPinnedObject();
            _len = (ulong) managed.Length;
            _wePinned = true;
        }

        public IEnumerator<Vec> GetEnumerator()
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

        [CustomMarshaller(typeof(SliceMutVec), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            public IntPtr Data;
            public ulong Len;

            public SliceMutVec Managed()
            {
                return new SliceMutVec(Data, Len);
            }
        }

        public ref struct Marshaller
        {
            private SliceMutVec managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;
            private SliceMutVec marshalled;

            public void FromManaged(SliceMutVec managed) { this.managed = managed; }
            public Unmanaged ToUnmanaged() => new Unmanaged { Data = managed._data, Len = managed._len };
            public void FromUnmanaged(Unmanaged unmanaged) { sourceNative = unmanaged; }
            public unsafe SliceMutVec ToManaged() => new SliceMutVec(sourceNative.Data, sourceNative.Len);
            public void Free() { }
        }
    }

    // Debug - write_type_definition_composite 
    ///Option type containing boolean flag and maybe valid data.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct OptionVec
    {
        ///Element that is maybe valid.
        internal Vec t;
        ///Byte where `1` means element `t` is valid.
        internal byte is_some;
    }

    // Debug - write_type_definition_composite_marshaller 
    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct OptionVec
    {
        [StructLayout(LayoutKind.Sequential)]
        public unsafe struct Unmanaged
        {
            // Debug - write_type_definition_composite_unmanaged_body_field 
            public Vec.Unmanaged t;
            // Debug - write_type_definition_composite_unmanaged_body_field 
            public byte is_some;
        }

        [CustomMarshaller(typeof(OptionVec), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta { }

        public ref struct Marshaller
        {
            private OptionVec _managed; // Used when converting managed -> unmanaged
            private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

            public Marshaller(OptionVec managed) { _managed = managed; }
            public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public void FromManaged(OptionVec managed) { _managed = managed; }
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public unsafe Unmanaged ToUnmanaged()
            {;
                _unmanaged = new Unmanaged();

                // Debug - write_type_definition_composite_marshaller_unmanaged_invoke 
                var _t = new Vec.Marshaller(_managed.t);
                _unmanaged.t = _t.ToUnmanaged();
                // Debug - write_type_definition_composite_marshaller_unmanaged_invoke 
                _unmanaged.is_some = _managed.is_some;

                return _unmanaged;
            }

            public unsafe OptionVec ToManaged()
            {
                _managed = new OptionVec();

                // Debug - write_type_definition_composite_marshaller_field_from_unmanaged 
                var _t = new Vec.Marshaller(_unmanaged.t);
                _managed.t = _t.ToManaged();
                // Debug - write_type_definition_composite_marshaller_field_from_unmanaged 
                _managed.is_some = _unmanaged.is_some;

                return _managed;
            }
            public void Free() { }
        }
    }

    // Debug - write_pattern_option 
    public partial struct OptionVec
    {
        public static OptionVec FromNullable(Vec? nullable)
        {
            var result = new OptionVec();
            if (nullable.HasValue)
            {
                result.is_some = 1;
                result.t = nullable.Value;
            }

            return result;
        }

        public Vec? ToNullable()
        {
            return this.is_some == 1 ? this.t : (Vec?)null;
        }
    }


    // Debug - write_type_definition_ffibool 
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Bool
    {
        byte value;
    }

    public partial struct Bool
    {
        public static readonly Bool True = new Bool { value =  1 };
        public static readonly Bool False = new Bool { value =  0 };
        public Bool(bool b)
        {
            value = (byte) (b ? 1 : 0);
        }
        public bool Is => value == 1;
    }


    // Debug - write_type_definition_named_callback 
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate uint MyCallbackNamespacedNative(uint value, IntPtr callback_data);
    public delegate uint MyCallbackNamespacedDelegate(uint value);

    public partial struct MyCallbackNamespaced : IDisposable
    {
        private MyCallbackNamespacedDelegate _callbackUser;
        private IntPtr _callbackNative;
    }

    [NativeMarshalling(typeof(MarshallerMeta))]
    public partial struct MyCallbackNamespaced : IDisposable
    {

        public MyCallbackNamespaced() { }

        public MyCallbackNamespaced(MyCallbackNamespacedDelegate callbackUser)
        {
            _callbackUser = callbackUser;
            _callbackNative = Marshal.GetFunctionPointerForDelegate(new MyCallbackNamespacedNative(Call));
        }

        public uint Call(uint value, IntPtr callback_data)
        {
            return _callbackUser(value);
        }

        public void Dispose()
        {
            if (_callbackNative == IntPtr.Zero) return;
            Marshal.FreeHGlobal(_callbackNative);
            _callbackNative = IntPtr.Zero;
        }


        [CustomMarshaller(typeof(MyCallbackNamespaced), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta {  }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            internal IntPtr Callback;
            internal IntPtr Data;
        }


        public ref struct Marshaller
        {
            private MyCallbackNamespaced _managed;
            private Unmanaged _unmanaged;

            public Marshaller(MyCallbackNamespaced managed) { _managed = managed; }
            public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public void FromManaged(MyCallbackNamespaced managed) { _managed = managed; }
            public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

            public Unmanaged ToUnmanaged()
            {
                return new Unmanaged
                {
                    Callback = _managed._callbackNative,
                    Data = IntPtr.Zero
                };
            }

            public MyCallbackNamespaced ToManaged()
            {
                return new MyCallbackNamespaced
                {
                    _callbackNative = _unmanaged.Callback,
                };
            }

            public void Free() { }
        }
    }




    public class InteropException<T> : Exception
    {
        public T Error { get; private set; }

        public InteropException(T error): base($"Something went wrong: {error}")
        {
            Error = error;
        }
    }

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void AsyncHelperNative(IntPtr data, IntPtr callback_data);
    public delegate void AsyncHelperDelegate(IntPtr data);

    [NativeMarshalling(typeof(MarshallerMeta))]
    public struct AsyncHelper : IDisposable
    {
        private AsyncHelperDelegate _callbackUser;
        private IntPtr _callbackNative;

        public AsyncHelper() { }

        public AsyncHelper(AsyncHelperDelegate callbackUser)
        {
            _callbackUser = callbackUser;
            _callbackNative = Marshal.GetFunctionPointerForDelegate(new AsyncHelperNative(Call));
        }

        public void Call(IntPtr data, IntPtr callback_data)
        {
            _callbackUser(data);
        }

        public void Dispose()
        {
            if (_callbackNative == IntPtr.Zero) return;
            Marshal.FreeHGlobal(_callbackNative);
            _callbackNative = IntPtr.Zero;
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
            private AsyncHelper managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;

            public void FromManaged(AsyncHelper managed)
            {
                this.managed = managed;
            }

            public Unmanaged ToUnmanaged()
            {
                return new Unmanaged
                {
                    Callback = managed._callbackNative,
                    Data = IntPtr.Zero
                };
            }

            public void FromUnmanaged(Unmanaged unmanaged)
            {
                sourceNative = unmanaged;
            }

            public AsyncHelper ToManaged()
            {
                return new AsyncHelper
                {
                    _callbackNative = sourceNative.Callback,
                };
            }

            public void Free() { }
        }
    }
}
