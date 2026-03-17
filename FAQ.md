# FAQ

## General

- **Where can I ask questions?**

  Use [Github Discussions](https://github.com/ralfbiedert/interoptopus/discussions).


## Rust Usage


- **How should I design my APIs?**

  This is a broad question and depends on your use case. As a rule of thumb we recommend being
  slightly conservative with your signatures and always
  _think C_. Other languages do not track lifetimes well, and it is easy to accidentally pass an
  outlived pointer, or doubly alias a `&mut X` on reentrant functions.


- **I have a `Vec<T>` in Rust, how can I move it to C#, Python, ...?**

  Moving a `Vec<T>` as-is cannot work as the type would be deallocted on passing the FFI boundary. Creating a
  new `FFIVec<T>` pattern _could_ be implemented, but is currently unsupported. The main design issue is that
  we would have to create helper methods on the user's behalf and manage ownership and (de)allocation on both side of
  the boundary.

  That said, if you want to pass arbitrarily long data from a Rust function `f` to FFI you have 3 options:

    - Accept a callback `f(c: MyCallback)`. This allows you to create data ad-hoc within `f` and invoke `callback` with
      a `FFISlice`.
    - Return a slice `f() -> FFISlice<T>`. For your users this is a bit nicer to call, but requires you to hold the
      `Vec<T>` somewhere else. Usually `f` would be a method of some service pattern. You also run the risk of UB if
      callers hold on to your slice for too long.
    - Accept a mutable slice `f(slice: FFISliceMut<T>)` and write into it. This is a bit more verbose for your caller
      but usually the most flexible and performant option.

  We recommend to accept callbacks if you have _a few but unknown-many_ elements and mutable slices if users can query
  the length by other means.


## Existing Backends

### General

- **I'm trying to compose pattern X with Y but get errors.**

  While on the Rust side patterns compose easily, backends usually have some trouble creating
  working code for things like `FFISlice<FFIOption<CStrPointer>>`. For example C# does
  not support generics on FFI types, thus a new `FFISliceT` type has to be generated for every use of `FFISlice<T>` in
  Rust. We
  generally do not recommend to nest type patterns.



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

    - Without pinning the .NET runtime could relocate the memory while the FFI call is running. In other words, when you
      enter `pattern_ffi_slice_1` `ffi_slice` might reside at `0x1234`, but during FFI execution the CLR GC can move the
      whole array to `0x1000` if it wants to optimize memory layout. Since this could happen while Rust is still
      accessing the old location UB or an access violation would ensue. Pinning prevents that.

    - The reason `Sliceu32` in turn only accepts a `GCHandle` and not the `uint[]` array itself is that once an object
      is pinned, somebody needs to remember its proper lifetime and to unpin it, but `Sliceu32` has no reserved field
      for that, being a low-level primitive. In most cases the method overload handling pinning is the right place, as
      lifetimes are guaranteed to be correct, but if you need a "long-lived" FFI slice (which, I'd argue, is playing
      with fire from an interop perspective) you'll also need to handle proper pinning / unpinning (aka _lifetime
      semantics_) and race prevention elsewhere.



## Safety, Soundness, Undefined Behavior

This library naturally does "unsafe" things and any journey into FFI-land is a little adventure. That said, here are
some assumptions and quality standards this project is based on:

### General Safety Considerations

- Safe Rust calling safe Rust code must always be sound, with soundness boundaries
  on the module level, although smaller scopes are preferred. For example, creating a `FFISlice`
  from Rust and directly using it from Rust must never cause UB.

- We must never willingly generate broken bindings. For _low level types_ we must never
  generate bindings which "cannot be used correctly" (e.g., map a `u8` to a `float`), for
  _patterns_ we must generate bindings that are "correct if used according to specification".

- There are situations where the (Rust) soundness of a binding invocation depends on conditions outside
  our control. In these cases we trust foreign code will invoke the generated functions
  correctly. For example, if a function is called with an `CStrPtr` type we consider it _safe and sound_
  to obtain a `str` from this pointer as `CStrPtr`'s contract specifies it must point to
  ASCII data.

- Related to the previous point we generally assume functions and types on both sides are used _appropriately_ w.r.t.
  Rust's FFI requirements and we trust you do that.

- Any `unsafe` code in any abstraction we provide should be "well contained", properly documented
  and reasonably be auditable.

- If unsound Rust types or bindings were ever needed (e.g., because of a lack of Rust specification,
  like 'safely' mapping a trait's vtable) such bindings should be gated behind a feature flag
  (e.g., `unsound`) and only enabled via an explicit opt-in. Right now there are none, but this is
  to set expectations around discussions.

**tl;dr**: if it's fishy we probably want to fix it, but we rely on external code calling in 'according to
documentation'.

### Specific Constructs

- **Around Rust '24 `#[no_mangle]` became unsafe, and you add it automatically, isn't that an issue?**

  Theoretically yes, practically no:
    - It is true that with `#[no_mangle]` you could cause UB, for example by accidentally writing a
      `#[no_mangle] fn malloc() -> usize {...}`. Around Rust '24 the
      attribute was therefore made unsafe. The vast majority (probably all) of safe Rust projects should
      simply not use the attribute because of that.
    - In FFI crates though, you _must_ use the attribute to get normal C names, and there is practically no way of
      knowing
      which names you are not supposed to use. In other words, even if we made you type
      `#[ffi_function] #[unsafe(no_mangle)] fn _ZN2io5stdio6_print20h94cd0587c9a534faX3gE() {...}`
      (compare [this Rust issue](https://github.com/rust-lang/rust/issues/28179)), and even if you tried to be diligent,
      you still wouldn't have any way of knowing whether what you just typed might cause UB (without using low-level
      symbol table analyzers after the fact).
    - By that same logic, there are quite a few other 'safe' things you are not supposed
      to do from FFI crates, e.g., messing up calling conventions, panicking, or _not_ specifying `#[no_mangle]`
      some of which can be impossible to guard against.
    - With all that said, us automatically handling these attributes does not create additional issues, but allows
      us to prevent some, and makes the library nicer to use.


## Licensing

A clarification how the license is meant to be applied:

- Our license only applies to code **in** this repository, not code generated **by** this repository.
- We do not claim copyright for code produced by backends included here; even if said code was based on a template in
  this repository.
- For the avoidance of doubt, anything produced by `Interop::write_to` or any item emitted by a proc macro is considered
  “generated by”.