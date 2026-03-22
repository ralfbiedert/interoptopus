# Plugin Call Overheads (Rust → .NET)

Times were determined by running the construct 100k times (warmup + measure), computing ns per call with an empty-loop baseline subtracted.

| Construct | ns per call |
| --- | --- |
| `primitive_void()` | 7 |
| `primitive_u32(42)` | 5 |
| `svc.call(5)` | 10 |
| `wire_string(Wire::from("{}")).unwire()` | 2862 |
| `wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()` | 790 |
| `wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()` | 5421 |
| `async service_async.add_one(1)` | 1060 |
| `async service_async.wire_1(Wire::from({1char,1char})).await` | 1984 |
| `async async_svc.add_one(1)` | 1283 |
| `async async_svc.wire_1(Wire::from({1char,1char})).await` | 1699 |
