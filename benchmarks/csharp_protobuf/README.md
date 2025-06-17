Benchmark comparing two approaches:
- ser/de from C# to Rust and back
  - using Google Protobuf
  - using Interoptopus FFI types
  - using Interoptopus Wire<T>

The difference should arise from Interoptopus knowing the exact types and layout and
being able to serialize in-memory without paying attention to external format limitations.

Additional improvements could include using zerocopy.
