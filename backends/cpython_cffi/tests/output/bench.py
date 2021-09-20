import ctypes
import reference_project as r
from reference_project import api
import time

N = 1_000_000

r.init_api("../../../../target/debug/interoptopus_reference_project.dll")

def bench(file, name, f, reference=0):
    start = time.perf_counter()

    for i in range(N):
        f()

    end = time.perf_counter()
    nanos = (end - start) * 1000 * 1000 * 1000
    nanos_per_single = (nanos / N) - reference

    name = f"`{name}`"
    print("|", name.ljust(50), "| {:,.0f} |".format(nanos_per_single), file=file)
    return nanos_per_single


# ptr = (ctypes.c_int64 * 100)(100, 2, 3)
# empty = r.Empty.c_array(1)


with open("BENCHMARK_RESULTS.md", 'w') as f:
    print("""

# FFI Call Overheads

The numbers below are to help FFI design decisions by giving order-of-magnitude estimates how
expensive certain constructs are.

Times were determined by running the given construct 1M times, taking the elapsed time in ticks,
and computing the cost per 1k invocations.


## System

The following system was used:    

```
System: i9-9900K, 32 GB RAM; Windows 10
rustc: stable (i.e., 1.53 or later)
profile: --release
Python: 3.10
```

## Results

| Construct | ns per call |
| --- | --- |""", file=f)

    reference = bench(f, "empty", lambda: 0)
    bench(f, "primitive_void()", lambda: r.api.primitive_void(), reference)
    bench(f, "primitive_u8(0)", lambda: r.api.primitive_u8(0), reference)
    bench(f, "primitive_u16(0)", lambda: r.api.primitive_u16(0), reference)
    bench(f, "primitive_u32(0)", lambda: r.api.primitive_u32(0), reference)
    bench(f, "primitive_u64(0)", lambda: r.api.primitive_u64(0), reference)
    bench(f, "many_args_5(0, 0, 0, 0, 0)", lambda: r.api.many_args_5(0, 0, 0, 0, 0), reference)
    # bench(f, "ptr(x)", lambda: r.api.ptr(ptr), reference)
    bench(f, "tupled(r.Tupled())", lambda: r.api.tupled(r.Tupled()), reference)
    # bench(f, "complex_args_1(r.Vec3f32(), empty)", lambda: r.api.complex_args_1(r.Vec3f32(), empty), reference)
    bench(f, "callback(lambda x: x, 0)", lambda: r.api.callback(lambda x: x, 0), reference)
    # bench(f, "pattern_ffi_slice_delegate(lambda x: x[0])", lambda: r.api.pattern_ffi_slice_delegate(lambda x: x[0]),
    #       reference)
    # bench("pattern_ffi_slice_delegate_huge(lambda x: x[0])", lambda: r.pattern_ffi_slice_delegate_huge(lambda x: x[0]), reference) # ??



