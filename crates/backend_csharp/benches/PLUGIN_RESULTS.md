# Plugin Call Overheads (Rust → .NET)

Times were determined by running the construct 100k times (warmup + measure), computing ns per call with an empty-loop baseline subtracted.

| Construct | ns per call |
| --- | --- |
| `primitive_void()` | 4 |
| `primitive_u32(42)` | 3 |
| `foo.wire(Wire::from("hello world")).unwire()` | 306 |
| `foo.wire2(Wire::from(16x{16char,16char})).unwire()` | 4901 |
| `foo.big16(Big16 { 16x u32 })` | 5 |
