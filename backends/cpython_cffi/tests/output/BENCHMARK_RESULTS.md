

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
| `empty`                                            | 51 |
| `primitive_void()`                                 | 275 |
| `primitive_u8(0)`                                  | 348 |
| `primitive_u16(0)`                                 | 353 |
| `primitive_u32(0)`                                 | 357 |
| `primitive_u64(0)`                                 | 351 |
| `many_args_5(0, 0, 0, 0, 0)`                       | 574 |
| `tupled(r.Tupled())`                               | 1,194 |
| `callback(lambda x: x, 0)`                         | 1,723 |
