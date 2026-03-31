+++
title = "FAQ"
weight = 400
+++

## General

#### Where can I ask questions?

Use [Github Discussions](https://github.com/ralfbiedert/interoptopus/discussions).

## Performance

#### Why does the async overhead appear so high in benchmarks?

It depends heavily on how many tasks are in flight, and how wake-ups are scheduled 
by the respective runtime. For example, when a `.NET` callback completes and needs to
resume a Tokio task, it calls the waker. If no Tokio worker thread is active, the OS must wake one
up — a futex operation that costs 1–4 µs. With other tasks already running, the waker hands off
directly to an active thread and the overhead drops a dramatic 8x despite 64x more work in flight:

| Construct [run on Linux]                 | ns / call |
|------------------------------------------|-----------|
| `plugin.add_one(1).await` [sequential]   | 4779      |
| `plugin.add_one(1).await` [64 in-flight] | 570       |

In practice, workloads with many concurrent async calls will not pay the elevated wake up cost;
and applications with few tasks in flight benefit from sleeping threads.



## Safety, Soundness, Undefined Behavior

#### How do you deal with safety around interop?

Interop by definition combines the safety models of two languages, and most other languages have a safety model worse than Rust's. The general rules we try to follow in that world are:

- **Rust-to-Rust is always sound.** Safe Rust using our types (e.g., `ffi::Slice`) must never cause UB, no matter how it's called. 
- **Generated bindings are never intentionally broken.** Types map correctly; pattern types are correct when
  used per their documentation. However, the documentation might require doing or not doing certain things. For example, C# users must `.Dispose()` objects or they will leak memory, or not arbitrarily use obtained `IntPtr`. 
- **Foreign callers are trusted.** Where Rust soundness depends on the caller (e.g., `CStrPtr` must point to valid ASCII), we trust the generated contract is followed even when called from unsafe languages (e.g., C).

## Licensing

#### What's the license of my generated code?

Interoptopus is MIT licensed, but that only applies to code **in** Interoptopus, not code generated **by** Interoptopus. We do not claim copyright for code produced by backends included in our repo; even if said code was based on a template contained within our repo. 