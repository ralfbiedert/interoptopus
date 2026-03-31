# Plugin Call Overheads (Rust → .NET)

Times were determined by running the construct 100k times (warmup + measure), reporting the median ns per call with an empty-loop baseline median subtracted.

| Construct | ns per call |
| --- | --- |
| `primitive_void()` | 100 |
| `primitive_u32(42)` | 100 |
| `svc.call(5)` | 100 |
| `wire_string(Wire::from("{}")).unwire()` | 2700 |
| `wire_hashmap_string(Wire::from(1x{16char,16char})).unwire()` | 1000 |
| `wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()` | 5000 |
| `async service_async.add_one(1) [sequential]` | 1300 |
| `async service_async.add_one(1) [64 in-flight]` | 598 |
| `async service_async.wire_1(Wire::from({1char,1char})).await` | 2000 |
| `async async_svc.add_one(1) [sequential]` | 1500 |
| `async async_svc.add_one(1) [64 in-flight]` | 489 |
| `async async_svc.wire_1(Wire::from({1char,1char})).await` | 1800 |
