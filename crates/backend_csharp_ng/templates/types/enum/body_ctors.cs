// Ctors
public static EnumPayload A => new() { _variant = 0 };
public static EnumPayload B(Vec3f32 value) => new() { _variant = 1, _B = value };
public static EnumPayload C(uint value) => new() { _variant = 2, _C = value };

// checks
public bool IsA => _variant == 0;
public bool IsB => _variant == 1;
public bool IsC => _variant == 2;

// conversions
public void AsA() { if (_variant != 0) throw ExceptionForVariant(); }
public Vec3f32 AsB() { if (_variant != 1) { throw ExceptionForVariant(); } else { return _B; } }
public uint AsC() { if (_variant != 2) { throw ExceptionForVariant(); } else { return _C; } }
