# Plugin Call Overheads (Rust → .NET)

Times were determined by running the construct 100k times (warmup + measure), computing ns per call with an empty-loop baseline subtracted.

| Construct | ns per call |
| --- | --- |
| `primitive_void()` | 7 |
| `primitive_u32(42)` | 7 |
| `svc.call(5)` | 10 |
| `wire_string(Wire::from("{}")).unwire()` | 3527 |
| `wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()` | 1240 |
| `wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()` | 6465 |
| `async service_async.add_one(1)` | 1418 |
| `async service_async.wire_1(Wire::from({1char,1char})).await` | 2581 |
| `async async_svc.add_one(1)` | 1507 |
| `async async_svc.wire_1(Wire::from({1char,1char})).await` | 2192 |
