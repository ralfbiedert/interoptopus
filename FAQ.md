# FAQ


## General

- **Why should I use this and not X?**

  As a rule of thumb, if you only need to support a single language (e.g., Python) then 
  `X` might be better; and you'll probably write FFI bindings tailored to that specific runtime 
  anyway.
  However, once you target more than one language everything you do needs to have
  a proper C representation and this crate aims to give you the best of both worlds, being both
  universally C-level _and_ reasonably idiomatic in each backend; including Rust.  


- **Who's the target audience for this?**

  Anyone writing 'system libraries' that need to be consumable from multiple
  environments at the same time. Our usage profile is:

  - several libraries with 100+ functions (think Vulkan, OpenXR),
  - callable from Unity, Unreal, Python,
  - high "API uncertainty" in the beginning, requiring lots of iterations to get it right,
  - predictable and "industry-grade" APIs in the end (final bindings ship to customers).


- **Where can I ask questions?**

  Use [Github Discussions](https://github.com/ralfbiedert/interoptopus/discussions).



## Rust Usage

- **Why do I get `error[E0658]: macro attributes in #[derive] output are unstable`?**

  This happens when `#[ffi_type]` appears after `#derive[...]`. Just switch their order.


- **Why do I get `use of undeclared crate or module` in `inventory!()`?**

  You probably forgot to declare one of your items `#[ffi_function]` or `#[ffi_const]`.


- **How can I add a new pattern?**

  Adding support for new patterns is best done via a PR. Patterns mimicking common Rust features
  and improvements to existing patterns are welcome. As a rule of thumb they
  should
  - be idiomatic in Rust (e.g., options, slices),
  - have a safe and sound Rust implementation and 'reasonable' usability in other languages,
  - be useful in more than one backend and come with a reference implementation,
  - _must_ fallback to C primitives (e.g., 'class methods' are functions with receivers).

  As an alternative, and discouraged for public backends, you might be able to get away using "tags".


- **How should I design my APIs?**

  This is a broad question and depends on your use case. As a rule of thumb we recommend being
  slightly conservative with your signatures and always
  _think C_. Other languages do not track lifetimes well, and it is easy to accidentally pass an
  outlived pointer, or doubly alias a `&mut X` on reentrant functions.





## Existing Backends

### C#

- **Why do you pin objects in the C# bindings and pass GCHandles to slice constructors?**

  This question relates to bindings generated like this:

  ```csharp
  public static uint pattern_ffi_slice_1(uint[] ffi_slice) {
      var ffi_slice_pinned = GCHandle.Alloc(ffi_slice, GCHandleType.Pinned);
      var ffi_slice_slice = new Sliceu32(ffi_slice_pinned, (ulong) ffi_slice.Length);
      try
      {
          return pattern_ffi_slice_1(ffi_slice_slice);
      }
      finally
      {
          ffi_slice_pinned.Free();
      }
  }
  ```
  
  - Without pinning the .NET runtime could relocate the memory while the FFI call is running. In other words, when you enter `pattern_ffi_slice_1` `ffi_slice` might reside at `0x1234`, but during FFI execution the CLR GC can move the whole array to `0x1000` if it wants to optimize memory layout. Since this could happen while Rust is still accessing the old location UB or an access violation would ensue. Pinning prevents that.
  
  - The reason `Sliceu32` in turn only accepts a `GCHandle` and not the `uint[]` array itself is that once an object is pinned, somebody needs to remember its proper lifetime and to unpin it, but `Sliceu32` has no reserved field for that, being a low-level primitive. In most cases the method overload handling pinning is the right place, as lifetimes are guaranteed to be correct, but if you need a "long-lived" FFI slice (which, I'd argue, is playing with fire from an interop perspective) you'll also need to handle proper pinning / unpinning (aka _lifetime semantics_) and race prevention elsewhere.


- **How can I get more performance with slices?**

  As mentioned above, the C# backend will pin slices by default. On our test machine this incurs a 
  performance overhead of about 30-40ns per pinned slice, but uses only safe C#:

  | Construct | ns per call |
  | --- | --- |
  | `pattern_ffi_slice_delegate(x => x[0])` | 195 |
  | `pattern_ffi_slice_delegate(x => x.Copied[0])` | 1307 |
  | `pattern_ffi_slice_delegate_huge(x => x[0])` | 190 |
  | `pattern_ffi_slice_delegate_huge(x => x.Copied[0])` | 11844317 |
  | `pattern_ffi_slice_2(short_vec, 0)` | 64 
  | `pattern_ffi_slice_2(long_vec, 0)` | 61 |
- 
  For a dramatic 2x - 150x (!) performance increase you can enable `use_unsafe` in the C# backend which will use 
  a `fixed` slice instead. 

  | Construct | ns per call |
  | --- | --- |
  | `pattern_ffi_slice_delegate(x => x[0])` | 52 |
  | `pattern_ffi_slice_delegate(x => x.Copied[0])` | 87 |
  | `pattern_ffi_slice_delegate_huge(x => x[0])` | 61 |
  | `pattern_ffi_slice_delegate_huge(x => x.Copied[0])` | 79741 |
  | `pattern_ffi_slice_2(short_vec, 0)` | 28 |
  | `pattern_ffi_slice_2(long_vec, 0)` | 24 |
  | `pattern_ffi_slice_4(short_byte, short_byte)` | 28 |

  This gives more performance when working with slices, but requires `<AllowUnsafeBlocks>true</AllowUnsafeBlocks>`
  being enabled in the C# project setting. In Unity it will force the entire game
  project to be ticked `Unsafe` and might not be _nice_ if you ship bindings to customers.
  However, if you only consume your own bindings and don't give them to 3rd parties this is a non-issue.


## New Backends

**Quickstart**

1) start a crate
1) copy code of whatever backend comes closest (e.g, C)
1) from `Interop::write_to` produce some output, fix errors as they appear
1) create UI test against `interoptopus_reference_project` to ensure quality

**Some Tips**

Once you understand how Interoptopus abstracts APIs writing a backend is quite simple: 

- The `Library` is _the input_ to any backend, as in, it fully describes what you should generate. 
  It mainly consists of these elements:
  - Types
  - Functions
  - Constants
  - Patterns
- Any backend more or less just converts each of these things one-by-one. It usually writes all constants,
  then all (composite) types and enums, then all functions, then (optionally) all patterns. 
  
- Writing and converting types is usually the most tricky part, and might require that you sort types by 
  dependencies (e.g., for C) or handle types differently depending on where they appear (e.g., in C# an 
  `IntPtr` in a field might become a `ref T` in a function).

- Patterns are fully optional. You can always just implement their "fallback type" 
  (e.g, an AsciiPointer is just a `*const u8`) and call it a day. However, when exporting larger APIs 
  (like 100+ functions) producing _idiomatic_ pattern bindings will be a good investment. 

**How long will it take?**

Judging from creating the existing backends, and assuming you've done some FFI
calls from that language to a C library, I'd say:

- **1h** - browsing an existing backend and understanding how CTypes work
- **2h** - producing MVP output that can call a single `hello_world()`
- **4h** - generate bindings for arbitrary functions with primitive parameters
- **1d** - also produce `structs` and `enums`
- **2d** - support the entire C API surface
- **3-5d** - have clean, idiomatic wrappers for all patterns and run automated reference tests



## Safety, Soundness, Undefined Behavior

This library naturally does "unsafe" things and any journey into FFI-land is a little adventure.
That said, here are some assumptions and quality standards this project is based on:

- Safe Rust calling safe Rust code must always be sound, with soundness boundaries
on the module level, although smaller scopes are preferred. For example, creating a `FFISlice`
from Rust and directly using it from Rust must never cause UB.

- We must never willingly generate broken bindings. For _low level types_ we must never
generate bindings which "cannot be used correctly" (e.g., map a `u8` to a `float`), for
_patterns_ we must generate bindings that are "correct if used according to specification".

- There are situations where the (Rust) soundness of a binding invocation depends on conditions outside
our control. In these cases we trust foreign code will invoke the generated functions
correctly. For example, if a function is called with an `AsciiPointer` type we consider it _safe and sound_
to obtain a `str` from this pointer as `AsciiPointer`'s contract specifies it must point to
ASCII data.

- Related to the previous point we generally assume functions and types on both sides are used _appropriately_ w.r.t.
Rust's FFI requirements and we trust you do that, e.g., you must declare types `#[repr(C)]` (or similar)
and functions `extern "C"`.

- Any `unsafe` code in any abstraction we provide should be "well contained", properly documented
and reasonably be auditable.

- If unsound Rust types or bindings were ever needed (e.g., because of a lack of Rust specification,
like 'safely' mapping a trait's vtable) such bindings should be gated behind a feature flag
(e.g., `unsound`) and only enabled via an explicit opt-in. Right now there are none, but this is
to set expectations around discussions.


**tl;dr**: if it's fishy we probably want to fix it, but we rely on external code calling in 'according to documentation'. 



## Errors vs. Panics

Any Rust code ...

- ... likely used in applications **must not** panic, unless a panic is clearly user-requested 
  (e.g., having an `unwrap()` on an `FFIOption`). If a function can fail it must return a 
  `Result`. Allocations & co. are currently exempt if they lack a good `Result`-API; 
  although the plan is to replace them eventually.
  
- ... reasonably only used during compilation or unit tests (e.g., proc macro code or code generation 
  helpers) **should panic**, if panicking can lead to a better developer experience (e.g., 
  clearer error messages). 


## Licensing

A clarification how the license is meant to be applied:

- Our license only applies to code **in** this repository, not code generated **by** this repository.
- We do not claim copyright for code produced by backends included here; even if said code was based on a template in this repository.
- For the avoidance of doubt, anything produced by `Interop::write_to` or any item emitted by a proc macro is considered “generated by”.