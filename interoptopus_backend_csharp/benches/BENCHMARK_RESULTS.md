
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

## Timings
 
| Construct | µs per 1k calls |
| --- | --- |
| `primitive_void()` | 8.232 |
| `primitive_u8(0)` | 8.481 |
| `primitive_u16(0)` | 8.8 |
| `primitive_u32(0)` | 8.496 |
| `primitive_u64(0)` | 8.395 |
| `pattern_ascii_pointer_1('hello world')` | 45.162 |
