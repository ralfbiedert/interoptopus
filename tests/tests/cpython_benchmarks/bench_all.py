import ctypes
import common
import reference_project as r


r.init_lib(common.DLL)
template = open("RESULTS.template", 'r').read()

ptr = (ctypes.c_int64 * 100)(100, 2, 3)

with open("RESULTS.md", 'w') as f:
    print(template, file=f)

    reference = common.bench(f, "Baseline (already subtracted)", lambda: 0)
    common.bench(f, "primitive_void()", lambda: r.primitive_void(), reference)
    common.bench(f, "primitive_u8(0)", lambda: r.primitive_u8(0), reference)
    common.bench(f, "primitive_u16(0)", lambda: r.primitive_u16(0), reference)
    common.bench(f, "primitive_u32(0)", lambda: r.primitive_u32(0), reference)
    common.bench(f, "primitive_u64(0)", lambda: r.primitive_u64(0), reference)
    common.bench(f, "many_args_5(0, 0, 0, 0, 0)", lambda: r.many_args_5(0, 0, 0, 0, 0), reference)
    common.bench(f, "ptr(x)", lambda: r.ptr(ptr), reference)
    common.bench(f, "tupled(r.Tupled())", lambda: r.call_tupled(r.Tupled()), reference)
    common.bench(f, "complex_args_1(r.Vec3f32(), empty)", lambda: r.complex_args_1(r.Vec3f32(), r.Tupled()), reference)
    common.bench(f, "callback(lambda x: x, 0)", lambda: r.callback(lambda x: x, 0), reference)
    common.bench(f, "pattern_ffi_slice_delegate(lambda x: x[0])", lambda: r.pattern_ffi_slice_delegate(lambda x: x[0]), reference)
    # bench("pattern_ffi_slice_delegate_huge(lambda x: x[0])", lambda: r.pattern_ffi_slice_delegate_huge(lambda x: x[0]), reference) # ??



