# Plugin Call Overheads (Rust → .NET)

Times were determined by running the construct 100k times (warmup + measure), computing ns per call with an empty-loop baseline subtracted.

| Construct | ns per call |
| --- | --- |
| `primitive_void()` | 47 |
| `primitive_u32(42)` | 47 |
| `svc.call(5)` | 51 |
| `wire_string(Wire::from("{}")).unwire()` | 3215 |
| `wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()` | 5545 |
| `async call_void()` | 4934 |
| `async add_one(1)` | 4737 |
| `async wire_1(16x{16char,16char})` | 8336 |
| `async_svc.call_void()` | 4743 |
| `async_svc.add_one(1)` | 4715 |
| `async_svc.wire_1(16x{16char,16char})` | 8357 |
