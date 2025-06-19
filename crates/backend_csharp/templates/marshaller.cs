public ref struct Marshaller
{
    private {{managed}} _managed;
    private Unmanaged _unmanaged;

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public Marshaller({{managed}} managed) { _managed = managed; }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void FromManaged({{managed}} managed) { _managed = managed; }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public Unmanaged ToUnmanaged() { return _managed.{{in_to}}Unmanaged(); }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public {{managed}} ToManaged() { return _unmanaged.{{in_to}}Managed(); }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Free() {}
}
