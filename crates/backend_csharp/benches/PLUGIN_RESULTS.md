# Plugin Call Overheads (Rust → .NET)

Times were determined by running the construct 100k times (warmup + measure), reporting the median ns per call with an empty-loop baseline median subtracted.

| Construct | ns per call |
| --- | --- |
| `primitive_void()` | 50 |
| `primitive_u32(42)` | 50 |
| `svc.call(5)` | 50 |
| `wire_string(Wire::from("{}")).unwire()` | 3287 |
| `wire_hashmap_string(Wire::from(1x{16char,16char})).unwire()` | 621 |
| `wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()` | 5781 |
| `async service_async.add_one(1) [sequential]` | 4698 |
| `async service_async.add_one(1) [64 in-flight]` | 597 |
| `async service_async.wire_1(Wire::from({1char,1char})).await` | 4949 |
| `async async_svc.add_one(1) [sequential]` | 4639 |
| `async async_svc.add_one(1) [64 in-flight]` | 534 |
| `async async_svc.wire_1(Wire::from({1char,1char})).await` | 4909 |
