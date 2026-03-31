    [UnmanagedCallersOnly(EntryPoint = "_trampoline_register")]
    internal static void _trampoline_register(long id, IntPtr fn_ptr) => Trampoline.Register(id, fn_ptr);
