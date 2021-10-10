
    
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
Python: 3.10.0
```
    
## Results

| Construct | ns per call |
| --- | --- |
| `empty`                                            | 51 |
| `primitive_void()`                                 | 276 |
| `primitive_u8(0)`                                  | 379 |
| `primitive_u16(0)`                                 | 395 |
| `primitive_u32(0)`                                 | 386 |
| `primitive_u64(0)`                                 | 408 |
| `many_args_5(0, 0, 0, 0, 0)`                       | 803 |
| `ptr(x)`                                           | 457 |
| `tupled(r.Tupled())`                               | 491 |
| `complex_args_1(r.Vec3f32(), empty)`               | 662 |
| `callback(lambda x: x, 0)`                         | 1,084 |
| `pattern_ffi_slice_delegate(lambda x: x[0])`       | 1,196 |
