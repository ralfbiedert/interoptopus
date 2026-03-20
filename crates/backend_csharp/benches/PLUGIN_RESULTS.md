# Plugin Call Overheads (Rust → .NET)

Times were determined by running the construct 100k times (warmup + measure), computing ns per call with an empty-loop baseline subtracted.

| Construct | ns per call |
| --- | --- |
| `primitive_void()` | 3 |
| `primitive_u32(42)` | 3 |
| `foo.wire(Wire::from("hello world")).unwire()` | 287 |
