+++
title = "Changelog"
weight = 300
+++


## 0.16

Having been in the works for over a year, this is essentially a total project rewrite. High-level things that changed:

- Interoptopus is entirely 'model-based' now. While earlier versions had the concept of an `Inventory` (a list of items used around an FFI boundary), due to flaws how the inventory was structured, many places used and transformed the contained items in a very ad hoc fashion. For example, the C# codegen that wrote a `Service.Foo(byte[])` just had to assume another part actually wrote `service_foo(IntPtr, int)`. Without actually knowing. That made it hard to reason about the code and refactor it. 
- All models are fully ID based and deduplicated. In earlier versions, in a method `fn f(Foo(Bar(u32)), Bar(u32)) {}`, the type `Bar` would be represented twice, despite it being the same item. This made it nearly impossible for anyone to modify or even analyze API definitions programmatically. 
- Backends emit code via templates now instead of hard-coded string fragments. This is another massive QOL improvement for maintenance and readability. 
- Many codegen constructs have seen a long-needed refresh and UX improvements. The new `#[ffi]` attribute is easier to use, patterns feel fresh and modern and are much more composable, and the generated code has been decrufted.
- Massive compile-time error handling improvement. Up until 0.15 it was relatively easy to use types at the FFI boundary that don't have proper support. With 0.16 we introduced plenty of `const {}` type assert checks that should catch these situations straight in your IDE. 
- In services, you can now transparently call `async` between Rust and any language that supports it (C# at the moment). All you need is to implement the `AsyncRuntime` trait for your type.
- A new `Wire<T>` type allows you to use Rust's `std` types `String`, `Vec<T>`, and `HashMap<K, V>` in FFI boundaries by transparently serializing and deserializing their content. It's usually much faster than Protobuf-hacks and "just works" without any extra tooling. 
- Support for 'reverse interop' was added. Thanks to the model and code-gen improvements mentioned above, you can now define a plug-in interface in Rust, and load and transparently call foreign code at runtime. This is huge for projects wanting to move to Rust, but having to deal with legacy dependencies that have no easy replacement.  
- One little wrinkle: The main focus of this release has been C# which has full support for all features. Unfortunately the C and Python backends haven't been ported yet. While Interoptopus remains fully polyglot in its core (you could write your own backend without ever talking to us), we'd need contributor support to keep the other backends up to date.

Oh, and also, we finally got a web site now, but it appears you found it already 🙂


## 0.15-alpha

This started a massive C# cleanup and codegen improvements that triggered the rewrite in 0.16, but it was never released beyond various `-alpha` versions. 


## 0.14

Notable Changes
- The `inventory!` macro was removed, you now just write a regular function returning an `Inventory`, see the reference project for details


## 0.13

Notable Changes

- The Python CFFI backend was retired and replaced with a Python CTypes backend. This might require changing some invocations, please see `reference_project.py` for details.
- The attributes of `#[ffi_service_method]` were changed once more, but no behavior changed.
    - `wrap` is now `on_panic`
    - `direct` is `return_default`
    - `raw` is `undefined_behavior`

## 0.12

Notable Changes
- The wrapping behavior of `#[ffi_service_method]` changed, `#[ffi_service_method(direct)]` is now `#[ffi_service_method(wrap = "direct")]`.

## 0.11

Notable Changes
- C# backend switched constructors to static methods. Wherever you used `new Service(x)` you now use `Service.NewWith(x)` (or similar).

## 0.10

Notable Changes
-   The C# backend was split into a `DotNet` and `Unity` flavor. Missing methods can be restored by adding `.add_overload_writer(DotNet::new())` to `Generator` for regular C# projects, and `.add_overload_writer(Unity::new())` when targeting Unity.

## 0.9

Notable Changes
- Replaced most `pattern!` macros with `#[pattern]` attributes.
- Added type hints support, upgraded minimum supported Python version to 3.7
