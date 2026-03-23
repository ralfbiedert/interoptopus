    [UnmanagedCallersOnly]
    public static void register_trampoline(long id, IntPtr fn_ptr) => Trampoline.Register(id, fn_ptr);
