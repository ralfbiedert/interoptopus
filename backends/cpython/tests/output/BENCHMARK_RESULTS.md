
    
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
| --- | --- |
| `empty`                                            | 52 |
| `primitive_void()`                                 | 267 |
| `primitive_u8(0)`                                  | 373 |
| `primitive_u16(0)`                                 | 409 |
| `primitive_u32(0)`                                 | 386 |
| `primitive_u64(0)`                                 | 387 |
| `many_args_5(0, 0, 0, 0, 0)`                       | 779 |
| `ptr(x)`                                           | 444 |
| `tupled(r.Tupled())`                               | 475 |
| `complex_args_1(r.Vec3f32(), empty)`               | 679 |
| `callback(lambda x: x, 0)`                         | 713 |
| `pattern_ffi_slice_delegate(lambda x: x[0])`       | 1,216 |
