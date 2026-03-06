[MethodImpl(MethodImplOptions.AggressiveOptimization)]
internal Unmanaged ToUnmanaged()
{
    var _unmanaged = new Unmanaged();
    _unmanaged._variant = _variant;
    if (_variant == 1) _unmanaged._B._B = _B.ToUnmanaged();
    if (_variant == 2) _unmanaged._C._C = _C;
    return _unmanaged;
}
