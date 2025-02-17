// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Text;
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
        public static partial void sample_function(Slice<byte> ignored);

        public static unsafe void sample_function(System.ReadOnlySpan<byte> ignored)
        {
            fixed (void* ptr_ignored = ignored)
            {
                var ignored_slice = new Slice<byte>(new IntPtr(ptr_ignored), (ulong) ignored.Length);
                sample_function(ignored_slice);;
            }
        }

    }

    // This is a helper for the marshallers for Slice<T> and SliceMut<T> of Ts that require custom marshalling.
    // It is used to precompile the conversion logic for the custom marshaller.
    internal static class CustomMarshallerHelper<T> where T : struct
    {
        // Delegate to convert a managed T to its unmanaged representation at the given pointer.
        // Precompiling these conversions minimizes overhead during runtime marshalling.
        public static readonly Action<T, IntPtr> ToUnmanagedFunc;
        // Delegate that converts unmanaged data at a specified pointer back to a managed instance of T.
        public static readonly Func<IntPtr, T> ToManagedFunc;

        // Indicates whether type T is decorated with a NativeMarshallingAttribute.
        public static readonly bool HasCustomMarshaller;
        // Size of the unmanaged type in bytes. This is used for memory allocation.
        public static readonly int UnmanagedSize;
        // The unmanaged type that corresponds to T as defined by the custom marshaller.
        // This assumes that the custom marshaller has a nested type named 'Unmanaged'.
        public static readonly Type UnmanagedType;

        // This runs once per type T, ensuring that the conversion logic is set up only once.
        static CustomMarshallerHelper()
        {
            var nativeMarshalling = typeof(T).GetCustomAttribute<NativeMarshallingAttribute>();
            if (nativeMarshalling != null)
            {
                var marshallerType = nativeMarshalling.NativeType;
                var convertToUnmanaged = marshallerType.GetMethod("ConvertToUnmanaged", BindingFlags.Public | BindingFlags.Static);
                var convertToManaged = marshallerType.GetMethod("ConvertToManaged", BindingFlags.Public | BindingFlags.Static);
                UnmanagedType = marshallerType.GetNestedType("Unmanaged")!;
                UnmanagedSize = Marshal.SizeOf(UnmanagedType);

                // If the stateless custom marshaller shape is not available we currently do not support marshalling T in a slice.
                if (convertToUnmanaged == null || convertToManaged == null)
                {
                    ToUnmanagedFunc = Expression.Lambda<Action<T, IntPtr>>(Expression.Throw(Expression.New(typeof(NotSupportedException))), Expression.Parameter(typeof(T)), Expression.Parameter(typeof(IntPtr))).Compile();
                    ToManagedFunc = Expression.Lambda<Func<IntPtr, T>>(Expression.Throw(Expression.New(typeof(NotSupportedException)), typeof(T)), Expression.Parameter(typeof(IntPtr))).Compile();
                }
                else
                {
                    var unsafeRead = typeof(CustomMarshallerHelper<T>).GetMethod(nameof(ReadPointer), BindingFlags.NonPublic | BindingFlags.Static)!.MakeGenericMethod(UnmanagedType)!;
                    var parameter = Expression.Parameter(typeof(IntPtr));
                    var unsafeCall = Expression.Call(unsafeRead, parameter);
                    var callExpression = Expression.Call(convertToManaged, unsafeCall);
                    ToManagedFunc = Expression.Lambda<Func<IntPtr, T>>(callExpression, parameter).Compile();

                     var unsafeWrite = typeof(CustomMarshallerHelper<T>).GetMethod(nameof(WritePointer), BindingFlags.NonPublic | BindingFlags.Static)!.MakeGenericMethod(UnmanagedType)!;
                    var managedParameter = Expression.Parameter(typeof(T));
                    var destParameter = Expression.Parameter(typeof(IntPtr));
                    var toUnmanagedCall = Expression.Call(convertToUnmanaged, managedParameter);
                    var unsafeWriteCall = Expression.Call(unsafeWrite, toUnmanagedCall, destParameter);
                    ToUnmanagedFunc = Expression.Lambda<Action<T, IntPtr>>(unsafeWriteCall, managedParameter, destParameter).Compile();
                }

                HasCustomMarshaller = true;
            }
            else
            {
                UnmanagedType = typeof(T);
                ToUnmanagedFunc = Expression.Lambda<Action<T, IntPtr>>(Expression.Throw(Expression.New(typeof(InvalidOperationException))), Expression.Parameter(typeof(T)), Expression.Parameter(typeof(IntPtr))).Compile();
                ToManagedFunc = Expression.Lambda<Func<IntPtr, T>>(Expression.Throw(Expression.New(typeof(InvalidOperationException)), typeof(T)), Expression.Parameter(typeof(IntPtr))).Compile();
                HasCustomMarshaller = false;
            }
        }

        // This exists to simplify the creation of the expression tree.
        private static void WritePointer<TUnmanaged>(TUnmanaged unmanaged, IntPtr dest)
        {
            unsafe { Unsafe.Write((void*)dest, unmanaged); }
        }

        // This exists to simplify the creation of the expression tree.
        private static TUnmanaged ReadPointer<TUnmanaged>(IntPtr ptr)
        {
             unsafe { return Unsafe.Read<TUnmanaged>((void*)ptr); }
        }
    }

    [NativeMarshalling(typeof(SliceMarshaller<>))]
    public readonly partial struct Slice<T> : IEnumerable<T> where T : struct
    {
        internal readonly T[]? Managed;
        internal readonly IntPtr Data;
        internal readonly ulong Len;

        public int Count => Managed?.Length ?? (int)Len;

        public unsafe ReadOnlySpan<T> ReadOnlySpan
        {
            get
            {
                if (Managed is not null)
                {
                    return new ReadOnlySpan<T>(Managed);
                }
                return new ReadOnlySpan<T>(Data.ToPointer(), (int)Len);
            }
        }

        public unsafe T this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                if (Managed is not null)
                {
                    return Managed[i];
                }
                return Unsafe.Read<T>((void*)IntPtr.Add(Data, i * Unsafe.SizeOf<T>()));
            }
        }

        public Slice(GCHandle handle, ulong count)
        {
            this.Data = handle.AddrOfPinnedObject();
            this.Len = count;
        }

        public Slice(IntPtr handle, ulong count)
        {
            this.Data = handle;
            this.Len = count;
        }

        public Slice(T[] managed)
        {
            this.Managed = managed;
            this.Data = IntPtr.Zero;
            this.Len = 0;
        }

        public IEnumerator<T> GetEnumerator()
        {
            for (var i = 0; i < Count; ++i)
            {
                yield return this[i];
            }
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }
    }

    [CustomMarshaller(typeof(Slice<>), MarshalMode.Default, typeof(SliceMarshaller<>.Marshaller))]
    internal static class SliceMarshaller<T> where T: struct
    {
        [StructLayout(LayoutKind.Sequential)]
        public unsafe struct Unmanaged
        {
            public IntPtr Data;
            public ulong Len;
        }

        public ref struct Marshaller
        {
            private Slice<T> managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;
            private T[] marshalled;

            public void FromManaged(Slice<T> managed)
            {
                this.managed = managed;
            }

            public unsafe Unmanaged ToUnmanaged()
            {
                if(managed.Count == 0)
                {
                    return default;
                }

                if (CustomMarshallerHelper<T>.HasCustomMarshaller)
                {
                    var count = managed.Count;
                    var size = CustomMarshallerHelper<T>.UnmanagedSize;
                    native.Len = (ulong)count;
                    native.Data = Marshal.AllocHGlobal(count * size);
                    for (var i = 0; i < count; i++)
                    {
                        CustomMarshallerHelper<T>.ToUnmanagedFunc!( managed[i], IntPtr.Add(native.Data, i * size));
                    }
                    return native;
                }
                else if(managed.Managed is not null)
                {
                    pinned = GCHandle.Alloc(managed.Managed, GCHandleType.Pinned);
                    return new Unmanaged
                    {
                        Data = pinned.Value.AddrOfPinnedObject(),
                        Len = (ulong)managed.Count
                    };
                }
                else
                {
                    return new Unmanaged
                    {
                        Data = (IntPtr)managed.Data,
                        Len = (ulong)managed.Len
                    };
                }
            }

            public void FromUnmanaged(Unmanaged unmanaged)
            {
                sourceNative = unmanaged;
            }

            public unsafe Slice<T> ToManaged()
            {
                if (CustomMarshallerHelper<T>.HasCustomMarshaller)
                {
                    var count = (int)sourceNative.Len;
                    var size = CustomMarshallerHelper<T>.UnmanagedSize;
                    marshalled = new T[count];
                    for (var i = 0; i < count; i++)
                    {
                        marshalled[i] = CustomMarshallerHelper<T>.ToManagedFunc!(IntPtr.Add(sourceNative.Data, i * size));
                    }
                    return new Slice<T>(marshalled);
                }
                else
                {
                    return new Slice<T>(sourceNative.Data, sourceNative.Len);
                }
            }

            public void Free()
            {
                if (native.Data != IntPtr.Zero)
                {
                    Marshal.FreeHGlobal(native.Data);
                }
                pinned?.Free();
            }
        }
    }



}
