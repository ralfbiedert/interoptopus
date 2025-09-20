
Implement the `#[ffi_service]` attribute macro inside `proc_macros`. I already started an empty function `ffi_service` 
in `service/mod.rs` for you.

You can see examples how the macro is used in the file `svc_basic.rs`, and the code must compile when you are done 
(except that one `&mut self` case which must fail).

## Overview

The attribute is always applied to an `impl {}` block like so:

```rust
#[ffi_type(service)]
pub struct Svc {}

#[ffi_service]
impl Svc {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }

    pub fn ok(&self) {}
}
```
It then does 3 things: 
- It generates a set of free-standing FFI functions (ctor, dtor, methods) that allow the service to be called from FFI. 
- It also implements the `ServiceInfo` trait so the rest of interoptopus can work with it. 
- It should also emit a bunch of `const _:() = { ... }` helper blocks to verify a few things during compile time.  


## Function Generation

1) Each function present inside a `#[ffi_service]` block should yield a function, plus an additional destructor 
   (we come to that later)
2) Each function emitted should generally be in the form, where `service_name` is the converted name of the struct (e.g.,
   `MyService` becoming `my_service`, and `method_name` the verbatim name of the original method): 
   Most parameters should be accepted as in the original, and similar the return type, but there are exceptions, see below.

```rust
#[ffi_function]
pub fn service_name_method_name(t: T) -> R {}
```

3) About names and method categories:
3a) Each method that does not contain some `&self`, `&mut self`, ... as the first parameter is considered a constructor. 
   A constructor is expected to return a `ffi::Result<Self, Error>` type.
3b) A method that accepts `&self`, ... is a regular method
3c) In there is a destructor. Users never write a constructor themselves, but you should emit one automatically. The 
    destructor name is `service_name_destroy`

4) A service that with at least one `async` method is considered and `async` service, otherwise a regular service.

5) Constructors generally look like so, where `ctor_name` comes from the method like `new` above, and instance is
   a pointer to a pointer to an instance of that service.

```rust
#[ffi_function]
pub fn service_name_ctor_name(...) -> <ffi::Result<(), Error> as ResultAsPtr>::AsPtr {}
```
Inside the body of a ctor, you should
- Construct the servce instance by calling `ServiceName::ctor_name(...)` and pass along all parameters
- Put the instance in a Box (normal services) or Arc (async services), leak that, and store the resulting pointer 
  in `instance`. 
- Return the resulting `*const ServiceName` in an `ffi::Ok()` (which the ::AsPtr should point to)

6) The destructor should generally look like  

```rust
#[ffi_function]
pub fn service_destroy(*const ServiceName) {}
```
It should take the raw pointer, convert it back to a Box or Arc, and drop it. 

7) If you find regular methods (that take &self or &mut self), like `pub fn ok(&self) {}` convert them like so

```rust
// pub fn ok(&self) {}
#[ffi_function]
pub fn service_ok(instance: *const ServiceName, ...) {}

// pub fn ok(&mut) {}
#[ffi_function]
pub fn service_ok(instance: *mut ServiceName, ...) {}
```
The generated function should summon a (mutable) reference from the passed pointer, and then forward the method call 
to the struct / service implementation.

If the method is an async service no `&mut self` is allowed anywhere. If you encounter that, emit a compile error.  

8) Any `///` documentation on any method or ctor should be forwarded to your auto generated methods  

9) If this is an async service, its get more special. As mentioned, async services may only have &self in regualr methods,
   and must use an `Arc` instead of a `Box`. In addition, `async` methods are wrapped in a special way. Async methods inside a 
   service are declared like this:

```rust
pub async fn call(this: Async<Self>, ...) -> ffi::Result<Foo, Error> {
}
```

The first parameter `this` is a special type `Async<Self>` that wraps the `Arc<Self>` you created in the constructor. From an async 
function you created you should make a new Arc clone and put than in `Async<Self>`. You can then assume the current `Self` implements
the `AsyncRuntime` trait, therefore you on the instance you should be able to `instance.spawn(async {})` a new async task. This async
task should then invoke the actual user coded `call().await` method, with all parameters passed. 

In addition, the method you actually generate under the hood should have an additional parameter added of type `AsyncCallback<Foo>`, 
where `Foo` is the type the user originally returned. When that `call().await` finishes you should attempt to obtain the `Foo` from the 
Result and invoke the AsyncCallback with that one (what we are doing we are translating an async Rust call into something that can be 
invoked sychronously over FFI with a provided callback, like in ancient JavaScript before async).

Lastly, you probably want to `mem::forget` the value you passed into that callback, because you're sending it over FFI (and is thus copied
over, but should actually be moved which we can't model in FFI)

The ffi method you write should as a return value have <ffi::Result<Foo, Error> as ResultAsUnitT>::AsUnitT, and you should return an `Ok(())` for it.



## ServiceInfo

You should also implement `ServiceInfo` for the type, the details look like so:

```rust
pub trait ServiceInfo {
    fn id() -> ServiceId;
    fn service() -> Service;
    fn register(inventory: &mut Inventory);
}

pub struct Service {
    ty: TypeId,
    ctors: Vec<FunctionId>,
    destructor: FunctionId,
    methods: Vec<FunctionId>,
}
```

- id() should be generated via the `id()` helper macro 
- register() should register itself, and before that:
   - every parameter type involved
   - the return value
   - `ffi::Result<Foo, Error> as ResultAsUnitT` 
- the `ty` type ID should be the one of the current service struct (e.g., `Svc` above)
- all other function IDs will come from the respective `#[ffi_function]` trait `FunctionInfo` that is being implemented for the secret helper 
  structs (e.g., a ffi_function `foo() {}` will have a struct called `foo{}` that implements `FunctionInfo` )


## Misc

1) You should not use any external crates except the ones present (prettyplease, syn, proc_macro2, quote)
2) Keep the code nice and human readable. Files should not be too long, do not add excessive documentation. 
   First abstract the argument parsing into some struct. Then build a model of the elements to emit, then emit them. Create
   helper files inside `service/` (in the proc macro) if needed to compartmentalize the code. Keep the code inside `mod.rs` 
   minimal, instead, add logic to appropriately named files next to `mod.rs`. Your code should align with how the `types` and 
  `function` macro works already.
3) You should tracks spans properly and otherwise crate a "well behaved" proc macro. If the macro determines that something 
   will not work out it should emit a compiler error.
4) You can assume the `interoptopus` crate is in scope for emitting code. That said, be nice and prefix all items with 
   the fully qualified path, e.g., `::interoptopus::foo`, `::std::option::Option`, ...
 
