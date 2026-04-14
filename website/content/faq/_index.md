+++
title = "FAQ"
weight = 400
+++

## General

#### Where can I ask questions?

Use [GitHub Discussions](https://github.com/ralfbiedert/interoptopus/discussions).


#### How do you pronounce it?

Inter‧op‧topus, just as one word. 

Once we support eight backend languages, we might rename it to Interoctopus.

## Technical 

#### What is a {service, plugin, extension}?

In Interoptopus these are three unrelated concepts each with a well-defined meaning.

**Services** are constructs you can define in Rust and expose over an FFI boundary. Each service consists of an (opaque) type and a number of methods operating on that type:

```rust
#[ffi(service)]
pub struct ServiceBasic {}

#[ffi]
impl ServiceBasic {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }
}
```

They need explicitly defined constructors, and come with destructors under the hood. As such they have a well-defined lifecycle. In practical terms a Rust service becomes a C# class and methods, whereas a plugin-based service becomes a droppable Rust struct with methods. The term 'service' was chosen (over instance or similar) because services have restrictions where and how they can be used and composed. For example, services can't be put in fields.


**Plugins** are a way to extend a Rust application through 'reverse interop'. Based on the `plugin!` macro found in the core `interoptopus` crate, they allow you to define APIs that can be fulfilled by other languages, e.g., C#:

```rust
plugin!(MyPlugin {
    fn foo(vec: Vec3f32) -> Vec3f32;
    fn bar(x: u32);
});
```

Once you defined your plugin you use a backend crate (e.g., `interoptopus_csharp`) to emit a plugin stub for the respective language (e.g., `interoptopus_csharp::DotnetLibrary`). You then implement and compile the plugin (e.g., via `dotnet build`) and eventually load it through the backends's provided plugin runtime (e.g., `interoptopus_csharp::rt::dynamic`). 

**Extensions** on the other hand are a feature of some codegen backends like `backend_csharp`. In essence, you can register extensions with a codegen pipeline, and these extensions will then be able to modify or inspect the emitted code through APIs:

```rust
impl RustCodegenExtension for MyExtension {
    fn init(&mut self, _: &mut RustInventory) {}
    fn post_model_cycle(&mut self, _: &RustInventory, _: PostModelPass) -> ModelResult {}
    fn post_model_all(&mut self, _: &RustInventory, _: PostModelPass) -> Result<(), Error> {}
    fn post_output(&mut self, _: &mut Multibuf, _: PostOutputPass) -> OutputResult {}
}

RustLibrary::builder(inventory)
  .with_extension(MyExtension::new())
  .build()
  .process()?;
```


## Performance

#### Why does the async overhead appear so high in benchmarks?

Async benchmarks can be highly misleading, both in what they measure, and what it means for your application. For example, the measured time depends heavily on how many tasks are in flight, and how wake-ups are scheduled by the respective runtime. 


| Construct [run on Linux]                 | ns / call |
|------------------------------------------|-----------|
| `plugin.add_one(1).await` [sequential]   | 4779      |
| `plugin.add_one(1).await` [64 in-flight] | 570       |


Here, when a `.NET` callback completes and needs to resume a Tokio task, it calls the waker. If no Tokio worker thread is active, the OS must wake one up — a futex operation that costs 1–4 µs. 

<div style="font-family: system-ui, sans-serif; font-size: 14px; max-width: 600px;">
  <div style="margin-bottom: 12px;">
    <div style="font-weight: 600; margin-bottom: 4px;">Sequential Benchmarks (Worker sleeping, ~4800 ns)</div>
    <div style="display: flex; border-radius: 6px; overflow: hidden; height: 32px; line-height: 32px; text-align: center; color: #fff; font-size: 12px;">
      <div style="width: 90px; background: #5b9bd5;">.NET done</div>
      <div style="flex: 1; background: #e06060; font-weight: 600;">OS wakes thread (1–4 µs)</div>
      <div style="width: 80px; background: #50b87a;">Rust resume</div>
    </div>
  </div>
</div>

With other tasks already running, the waker hands off
directly to an active thread and the overhead drops by a dramatic 8x despite 64x more work in flight:

<div style="font-family: system-ui, sans-serif; font-size: 14px; max-width: 600px;">
    <div>
    <div style="font-weight: 600; margin-bottom: 4px;">Concurrent Operation (Worker active, ~570 ns)</div>
    <div style="display: flex; border-radius: 6px; overflow: hidden; height: 32px; line-height: 32px; text-align: center; color: #fff; font-size: 12px;">
      <div style="width: 90px; background: #5b9bd5;">.NET done</div>
      <div style="width: 80px; background: #50b87a;">Rust resume</div>
    </div>
  </div>
</div>

Part of that wakeup can be CPU cost; part of it is just waiting for a time slice. In practice, workloads with many concurrent async calls will not pay the elevated wake-up cost;
and applications with few tasks in flight doing actual `async` work will benefit from sleeping threads.



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