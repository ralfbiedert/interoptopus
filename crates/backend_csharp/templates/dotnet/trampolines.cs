/// Stores runtime trampolines registered by the Rust host at plugin load time.
public static unsafe class Trampolines
{
    private static delegate* unmanaged[Cdecl]<int, int*, int*, IntPtr> _wire_create;
    private static delegate* unmanaged[Cdecl]<IntPtr, int, int, void> _wire_destroy;
    private static delegate* unmanaged[Cdecl]<nint, byte*, int, void> _uncaught_exception;
    private static nint _uncaught_exception_ctx;

    private const long WIRE_CREATE              = 0x4952_4F50_5743_0001;
    private const long WIRE_DESTROY             = 0x4952_4F50_5743_0002;
    private const long UNCAUGHT_EXCEPTION       = 0x4952_4F50_5743_0003;
    private const long UNCAUGHT_EXCEPTION_CTX   = 0x4952_4F50_5743_0004;

    public static void Register(long id, IntPtr fn_ptr)
    {
        if (id == WIRE_CREATE)            _wire_create            = (delegate* unmanaged[Cdecl]<int, int*, int*, IntPtr>)fn_ptr;
        if (id == WIRE_DESTROY)           _wire_destroy           = (delegate* unmanaged[Cdecl]<IntPtr, int, int, void>)fn_ptr;
        if (id == UNCAUGHT_EXCEPTION)     _uncaught_exception     = (delegate* unmanaged[Cdecl]<nint, byte*, int, void>)fn_ptr;
        if (id == UNCAUGHT_EXCEPTION_CTX) _uncaught_exception_ctx = fn_ptr;
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

    /// Notifies the Rust host that an unhandled exception escaped a trampoline method.
    /// Safe to call even if no handler has been registered.
    public static void UncaughtException(string message)
    {
        if (_uncaught_exception == null) return;
        var bytes = System.Text.Encoding.UTF8.GetBytes(message);
        fixed (byte* ptr = bytes)
        {
            _uncaught_exception(_uncaught_exception_ctx, ptr, bytes.Length);
        }
    }
}
