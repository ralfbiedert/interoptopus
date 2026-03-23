/// Marks a Rust item for FFI export and generates the required interop metadata.
///
/// `#[ffi]` is the unified entry point for annotating structs, enums, functions, constants, and
/// service `impl` blocks. It dispatches to item-specific code generation based on the annotated
/// item kind and accepts a set of optional attributes to control the output.
///
/// # Structs and Enums
///
/// Annotating a struct or enum registers it as an FFI type and derives [`TypeInfo`]. A
/// `#[repr(C)]` is added to structs and `#[repr(u32)]` to enums if no `repr` is present.
///
/// ```rust
/// # use interoptopus::ffi;
/// #[ffi]
/// pub struct Vec2 {
///     pub x: f32,
///     pub y: f32,
/// }
///
/// #[ffi]
/// pub enum Status { Ok, Err }
/// ```
///
/// **Attributes:**
///
/// | Attribute | Description |
/// |-----------|-------------|
/// | `name = "Name"` | Override the name used in generated bindings. |
/// | `module = "name"` / `module = common`  | Assign to a named or common module. |
/// | `opaque` | Declare the type opaque (no field layout exposed). |
/// | `packed` | Apply `#[repr(C, packed)]`. |
/// | `transparent` | Apply `#[repr(transparent)]`. |
/// | `debug` | Print the generated code to stderr during compilation. |
///
/// # Functions
///
/// Annotating a `pub fn` registers it as an FFI function and generates a companion struct
/// implementing [`FunctionInfo`]. The function itself is kept as-is.
///
/// ```rust
/// # use interoptopus::ffi;
/// #[ffi]
/// pub fn sum(a: u32, b: u32) -> u32 { a + b }
/// ```
///
/// **Attributes:**
///
/// | Attribute | Description |
/// |-----------|-------------|
/// | `export = "name"` | Override the symbol name used in the generated `.dll` / `.so`. |
/// | `export = unique` | Generate a unique export name to avoid symbol clashes. |
/// | `module = "name"` / `module = common` | Assign to a named or common module. |
/// | `debug` | Print the generated code to stderr during compilation. |
///
/// The generated function uses `extern "C-unwind"` calling convention. If a panic
/// reaches the FFI boundary it will unwind into the caller. What then happens is platform specific.
/// In C# this can surface as a `System.Runtime.InteropServices.SEHException` on Windows, or process
/// aborts in other cases. In other words, you should generally avoid panicking in FFI code through
/// `#[ffi]` functions.
///
/// # Constants
///
/// Annotating a `const` item registers it as an FFI constant and derives [`ConstantInfo`].
///
/// ```rust
/// # use interoptopus::ffi;
/// #[ffi]
/// pub const MAX_ITEMS: u32 = 1024;
/// ```
///
/// **Attributes:**
///
/// | Attribute | Description |
/// |-----------|-------------|
/// | `name = "NAME"` | Override the name used in generated bindings. |
/// | `module = "name"` / `module = common` | Assign to a named or common module. |
/// | `debug` | Print the generated code to stderr during compilation. |
///
/// # Services (impl blocks)
///
/// Annotating an `impl` block turns its public methods into standalone FFI functions and registers
/// the type as a service. The struct itself must also be annotated with `#[ffi]` (no additional
/// attributes required on the `impl` line).
///
/// Methods that should not be exposed can be marked with `#[ffi::skip]`. Non-`pub` methods are
/// automatically excluded.
///
/// ```rust
/// # use interoptopus::ffi;
/// #[ffi]
/// pub enum MyError { General }
///
/// #[ffi(service)]
/// pub struct Counter { count: u32 }
///
/// #[ffi]
/// impl Counter {
///     pub fn create() -> ffi::Result<Self, MyError> { ffi::Ok(Self { count: 0 }) }
///     pub fn increment(&mut self) { self.count += 1; }
///     pub fn get_count(&self) -> u32 { self.count }
/// }
/// ```
///
/// **Attributes:**
///
/// | Attribute | Description |
/// |-----------|-------------|
/// | `prefix = "name"` | Override the `snake_case` prefix used for generated FFI function names. |
/// | `export = unique` | Generate unique export names for all emitted methods to avoid symbol clashes.  |
/// | `debug` | Print the generated code to stderr during compilation. |
///
/// # Skipping Fields
///
/// Individual struct fields can be excluded from the FFI layout with `#[ffi::skip]`. This is
/// typically used for marker types like `PhantomData` that carry no runtime data.
///
/// ```rust
/// # use interoptopus::ffi;
/// use std::marker::PhantomData;
///
/// #[ffi]
/// pub struct Tagged {
///     pub id: u32,
///     #[ffi::skip]
///     pub _marker: PhantomData<u32>,
/// }
/// ```
///
/// [`TypeInfo`]: crate::lang::rust::TypeInfo
/// [`FunctionInfo`]: crate::lang::rust::FunctionInfo
/// [`ConstantInfo`]: crate::lang::rust::ConstantInfo
#[cfg(feature = "derive")]
pub use interoptopus_proc::ffi;

/// Derives the [`AsyncRuntime`](crate::pattern::asynk::AsyncRuntime) trait for a service struct
/// by forwarding to one of its fields.
///
/// The macro looks for the runtime field in this order:
/// 1. A field annotated with `#[runtime]`.
/// 2. A field named `runtime`.
///
/// The chosen field's type must itself implement [`AsyncRuntime`](crate::pattern::asynk::AsyncRuntime). The generated impl
/// delegates [`spawn`](crate::pattern::asynk::AsyncRuntime::spawn) to that field.
///
/// # Example
///
/// ```rust
/// use interoptopus::{AsyncRuntime, ffi};
/// use interoptopus::rt::Tokio;
///
/// #[ffi(service)]
/// #[derive(AsyncRuntime)]
/// pub struct MyService {
///     runtime: Tokio,
/// }
/// ```
///
/// When the field is not named `runtime`, mark it with `#[runtime]`:
///
/// ```rust
/// use interoptopus::{AsyncRuntime, ffi};
/// use interoptopus::rt::Tokio;
///
/// #[ffi(service)]
/// #[derive(AsyncRuntime)]
/// pub struct MyService {
///     #[runtime]
///     rt: Tokio,
/// }
/// ```
#[cfg(feature = "derive")]
pub use interoptopus_proc::AsyncRuntime;

/// Declares a plugin interface for reverse interop, e.g., loading a C# DLL from Rust.
///
/// Whereas normal interoptopus use exports Rust code to other languages, `plugin!` works in the
/// opposite direction: it lets Rust *call into* a foreign library (e.g. a C# `.dll`) by generating
/// a typed Rust struct whose methods dispatch through FFI function pointers loaded at runtime.
///
/// # Syntax
///
/// Plugins are defined with a special syntax that can declare direct functions and instantiatable
/// services:
///
/// ```rust
/// use interoptopus::ffi;
/// use interoptopus::lang::meta::Visibility::Public;
/// use interoptopus::lang::types::TypeKind::Enum;
/// use interoptopus::wire::Wire;
///
/// # #[ffi]
/// # #[derive(Clone)]
/// # enum Error { A }
/// #
/// interoptopus::plugin!(MyPlugin {
///     // Direct synchronous and async functions
///     fn add_one(x: u32) -> u32;
///     async fn process(data: Wire<String>);
///
///     // Service blocks: a constructor returning Self plus instance methods
///     impl Processor {
///         fn create(name: Wire<String>) -> Self;
///         fn run(&self, x: f32) -> f32;
///         async fn run_async(&self) -> ffi::Result<u8, Error>;
///     }
/// });
/// ```
///
/// For the example above, the macro generates a `MyPlugin` struct with the following methods:
///
/// ```rust,ignore
/// // Direct functions — call straight through to the loaded FFI symbol:
/// plugin.add_one(1)
/// plugin.process(Wire::from("hi")).await
///
/// // Service constructor — symbol name is the lowercased type name + "_create":
/// let proc: Processor = plugin.processor_create(Wire::from("my-proc"));
///
/// // Instance methods on the returned Processor value:
/// proc.run(1.5)
/// proc.run_async().await
/// ```
///
/// # Loading a plugin
///
/// To instantiate a plugin a backend-specific loader is needed. For example,
/// the `interoptopus_csharp` crate provides `DotNetRuntime` and `DllLoader` for
/// loading `.NET` assemblies:
///
/// ```rust,ignore
/// let loader = DotNetRuntime::new()?
///     .exception_handler(|msg| eprintln!("plugin error: {msg}"))
///     .dll_loader("path/to/my_plugin.dll")?;
///
/// let plugin = MyPlugin::new(&loader)?;
/// ```
///
/// Note, this example is illustrative, the actual API is subject to change.
#[cfg(all(feature = "derive", feature = "unstable-plugins"))]
pub use interoptopus_proc::plugin;

/// Strips module paths from a fully-qualified Rust type name, preserving generic structure.
///
/// For example, `"my_crate::module::Struct<alloc::string::String>"` becomes `"Struct<String>"`.
/// Handles nested generics and multiple type parameters.
#[must_use]
pub fn strip_module_paths(full: &str) -> String {
    // Find the first top-level '<' (not nested)
    let mut depth = 0usize;
    let mut angle_pos = None;
    for (i, b) in full.bytes().enumerate() {
        match b {
            b'<' if depth == 0 => {
                angle_pos = Some(i);
                break;
            }
            b'<' => depth += 1,
            b'>' => depth -= 1,
            _ => {}
        }
    }

    if let Some(pos) = angle_pos {
        // Split into base path and <...> suffix
        let base = &full[..pos];
        let rest = &full[pos..]; // includes '<' and '>'

        // Strip module path from base: take last :: segment
        let short_base = base.rsplit("::").next().unwrap_or(base);

        // Recursively strip inside angle brackets
        // rest is "<inner_content>" — strip the outer < >
        let inner = &rest[1..rest.len() - 1];

        // Split inner by top-level commas and strip each part
        let mut parts = Vec::new();
        let mut part_start = 0;
        let mut d = 0usize;
        for (i, b) in inner.bytes().enumerate() {
            match b {
                b'<' => d += 1,
                b'>' => d -= 1,
                b',' if d == 0 => {
                    parts.push(inner[part_start..i].trim());
                    part_start = i + 1;
                }
                _ => {}
            }
        }
        parts.push(inner[part_start..].trim());

        let stripped_parts: Vec<String> = parts.iter().map(|p| strip_module_paths(p)).collect();
        format!("{}<{}>", short_base, stripped_parts.join(", "))
    } else {
        // No generics — just strip the module path
        full.rsplit("::").next().unwrap_or(full).to_string()
    }
}
