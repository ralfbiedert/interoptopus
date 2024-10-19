# FFI Call Overheads

The numbers below are to help FFI design decisions by giving order-of-magnitude estimates how
expensive certain constructs are.

Times were determined by running the given construct 1M times, taking the elapsed time in ticks,
and computing the cost per 1k invocations.


## System

The following system was used:    
    
```
System: AMD Ryzen 9 7950X3D, 64 GB RAM; Windows 11
rustc: stable (i.e., 1.82 or later)
profile: --release
Python: 3.11.5
```
    
## Results

| Construct | ns per call |
| --- | --- |
| `primitive_void()`                                 | 138 |
| `primitive_u8(0)`                                  | 224 |
| `primitive_u16(0)`                                 | 228 |
| `primitive_u32(0)`                                 | 235 |
| `primitive_u64(0)`                                 | 229 |
| `many_args_5(0, 0, 0, 0, 0)`                       | 484 |
| `ptr(x)`                                           | 283 |
| `tupled(r.Tupled())`                               | 282 |
| `complex_args_1(r.Vec3f32(), empty)`               | 495 |
| `callback(lambda x: x, 0)`                         | 733 |
| `pattern_ffi_slice_delegate(lambda x: x[0])`       | 871 |
