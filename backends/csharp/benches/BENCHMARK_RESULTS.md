
# FFI Call Overheads

The numbers below are to help FFI design decisions by giving order-of-magnitude estimates how
expensive certain constructs are.

## Notes

- Times were determined by running the given construct N times, taking the elapsed time in ticks,
and computing the cost per 1k invocations.

- The time of the called function is included.

- However, the reference project was written so that each function is _minimal_, i.e., any similar
function you wrote, would have to at least as expensive operations if it were to do anything sensible with
the given type.

- The list is ad-hoc, PRs adding more tests to `Benchmark.cs` are welcome.


## System

The following system was used:

```
System: i9-9900K, 32 GB RAM; Windows 10
rustc: stable (i.e., 1.53 or later)
profile: --release
.NET: v3.1 (netcoreapp3.1)
```

## Results

| Construct | ns per call |
| --- | --- |
| `primitive_void()` | 7 |
| `primitive_u8(0)` | 8 |
| `primitive_u16(0)` | 8 |
| `primitive_u32(0)` | 8 |
| `primitive_u64(0)` | 8 |
| `many_args_5(0, 0, 0, 0, 0)` | 11 |
| `many_args_10(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)` | 14 |
| `ptr(x)` | 9 |
| `ptr_mut(x)` | 10 |
| `ref_simple(x)` | 8 |
| `ref_option(x)` | 11 |
| `tupled(new Tupled())` | 9 |
| `complex_args_1(new Vec3f32(), ref e)` | 12 |
| `callback(x => x, 0)` | 48 |
| `dynamic_api.tupled(new Tupled())` | 15 |
| `pattern_ffi_option_1(new FFIOptionInner())` | 9 |
| `pattern_ffi_slice_delegate(x => x[0])` | 195 |
| `pattern_ffi_slice_delegate(x => x.Copied[0])` | 1307 |
| `pattern_ffi_slice_delegate_huge(x => x[0])` | 190 |
| `pattern_ffi_slice_delegate_huge(x => x.Copied[0])` | 11844317 |
| `pattern_ffi_slice_2(short_vec, 0)` | 64 |
| `pattern_ffi_slice_2(long_vec, 0)` | 61 |
| `pattern_ascii_pointer_1('hello world')` | 44 |
