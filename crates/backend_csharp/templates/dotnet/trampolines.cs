/// Stores runtime trampolines registered by the Rust host at plugin load time.
public static unsafe class Trampolines
{
    private static delegate* unmanaged[Cdecl]<int, int*, int*, IntPtr> _wire_create;
    private static delegate* unmanaged[Cdecl]<IntPtr, int, int, void> _wire_destroy;

    private const long WIRE_CREATE  = 0x4952_4F50_5743_0001;
    private const long WIRE_DESTROY = 0x4952_4F50_5743_0002;

    [UnmanagedCallersOnly]
    public static void register_trampoline(long id, IntPtr fn_ptr)
    {
        if (id == WIRE_CREATE)  _wire_create  = (delegate* unmanaged[Cdecl]<int, int*, int*, IntPtr>)fn_ptr;
        if (id == WIRE_DESTROY) _wire_destroy = (delegate* unmanaged[Cdecl]<IntPtr, int, int, void>)fn_ptr;
    }

    public static IntPtr WireCreate(int size, out int len, out int capacity)
    {
        fixed (int* pLen = &len, pCap = &capacity)
        {
            return _wire_create(size, pLen, pCap);
        }
    }

    public static void WireDestroy(IntPtr data, int len, int capacity)
    {
        _wire_destroy(data, len, capacity);
    }
}
