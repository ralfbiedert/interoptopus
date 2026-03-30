    [UnmanagedCallersOnly(EntryPoint = "register_trampoline")]
    internal static void register_trampoline(long id, IntPtr fn_ptr) => Trampoline.Register(id, fn_ptr);
