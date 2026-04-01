+++
title = "C#"
sort_by = "weight"
weight = 100
+++


This page shows how Interoptopus constructs look and are used from C#. For the Rust-side definitions see [Constructs](@/constructs/_index.md).


## Types

### Primitives

Rust primitives map directly to .NET types.

| Rust | C# |
|---|---|
| `bool` | `Bool` (custom struct with explicit conversions) |
| `u8` / `i8` | `byte` / `sbyte` |
| `u16` / `i16` | `ushort` / `short` |
| `u32` / `i32` | `uint` / `int` |
| `u64` / `i64` | `ulong` / `long` |
| `f32` / `f64` | `float` / `double` |
| `usize` / `isize` | `nuint` / `nint` |

### Structs

Structs become C# types with the same field names.

```csharp
var v = new Vec3f32 { x = 1.0f, y = 2.0f, z = 3.0f };
var result = Interop.call_vec(v);
```

Structs may be emitted with an `IDisposable` implementation if they hold resources. Check the generated documentation or your IDE hints to see whether that is the case. 

### Enums

Enums are translated into special enum classes including variant constructors if they contain a payload. Helpers such as `.Is*` properties, and `.As*()` accessors are also emitted.

```csharp
// Create variants
var a = EnumPayload.A;
var b = EnumPayload.B(new Vec3f32 { x = 1.0f, y = 2.0f, z = 3.0f });
var c = EnumPayload.C(123);

// Check variant
if (b.IsB)
{
    Vec3f32 value = b.AsB();
}
```


## Patterns

### Slices

Slices wrap managed arrays and provide accessors. Use the `.Slice()` and `.SliceMut()` extension methods for convenience.

```csharp
// Immutable slice
using var data = new uint[] { 1, 2, 3 }.Slice();
var len = Interop.pattern_ffi_slice_1(data);

// Mutable slice
using var data = new byte[] { 0, 0, 0 }.SliceMut();
Interop.pattern_ffi_slice_3(data, (slice) =>
{
    slice[0] = 1;
    slice[1] = 100;
});
```

Slices must be disposed (or used in a `using` block) to unpin the underlying array.

### Option

Options become discriminated unions with `.IsSome` / `.IsNone` checks and `.AsSome()` extraction.

```csharp
var option = OptionInner.Some(new Inner { x = 123.0f });
```

If the inner type requires disposal, the Option does too.

### Result

Results are discriminated unions with `.IsOk` / `.IsErr` checks and `.AsOk()` / `.AsErr()` extraction.

```csharp
var result = Interop.pattern_result_2();
Assert.True(result.IsOk);

// Extract value — throws if Err
var value = result.AsOk();

// Check error
if (result.IsErr)
{
    Error err = result.AsErr();
}
```

In service methods, `Result<(), E>` is unwrapped automatically — errors throw an `EnumException<Error>`.

### Strings

Rust `ffi::String` maps to `Utf8String`, which is `IDisposable`. Use the `.Utf8()` extension method to create one from a C# string.

```csharp
// Create from C# string
using var s = "hello world".Utf8();
Interop.pattern_string_1(s);

// Read back
using var result = Interop.pattern_string_3();
Assert.Equal("pattern_string_3", result.String);

// Clone to extend lifetime
using var copy = s.Clone();
```

Passing by value transfers ownership to Rust and the current instance will be invalidated. Received values must be disposed. 

### Vectors

Rust `ffi::Vec<T>` maps to typed vector classes (e.g., `VecByte`, `VecUtf8String`).

```csharp
// Receive from Rust
using var vec = Interop.pattern_vec_1();
Assert.Equal(3ul, vec.Count);
Assert.Equal(1, vec[0]);

// Pass to Rust (moves ownership — vec becomes unusable)
Interop.pattern_vec_2(vec);

// Create from array
using var v = VecUtf8String.From(new[] { "a".Utf8(), "b".Utf8() });
```

Passing by value transfers ownership to Rust and the current instance will be invalidated. Received values must be disposed. 


### Callbacks

Callbacks wrap C# delegates or Fn closures so they can be passed around. They implement `IDisposable`.

```csharp
// Inline delegate (simplest)
var x = Interop.pattern_callback_1(value => value + 1, 0);
Assert.Equal(1u, x);

// Retained callback
using var cb = new MyCallback(value => value + 1);
var x = Interop.pattern_callback_1(cb, 0);
```

If a C# delegate throws, the exception is captured and re-thrown when you call `.Dispose()`. We recommend to use retained callbacks wherever possible, as the C# cost of pinning callbacks is significant (100s of ns).


## Wire

Wire types use the `.Wire()` extension method for creation. They are `IDisposable`.

```csharp
// Pass a struct via Wire
var x = new MyString { x = "hello world" }.Wire();
Interop.wire_accept_string_2(x);

// Unwire and rewire
public WireOfY ProcessWire(WireOfX x)
{
    var dict = x.Unwire();
    dict["hello"] = "world";
    return dict.Wire();
}
```


## Forward Interop

### Services

Services become `IDisposable` classes with factory methods and instance methods.

```csharp
// Create and use
using var service = ServiceBasic.Create();

// Methods
using var service = ServiceResult.Create();
var val = service.ResultU32();          // returns uint
var str = service.ResultString();       // returns Utf8String

// Error-returning methods throw on failure
Assert.Throws<EnumException<Error>>(() => service.Test());

// Service dependencies
using var main = ServiceMain.Create(42);
using var dependent = ServiceDependent.FromMain(main);
```

Multiple constructors are each exposed as separate factory methods.

### Async Services

Async methods return `Task` or `Task<T>` and support `CancellationToken`.

```csharp
using var service = ServiceAsyncBasic.Create();
await service.Call();

// With return value
using var service = ServiceAsyncSleep.Create();
var result = await service.ReturnAfterMs(42, 100);
Assert.Equal(42ul, result);

// Cancellation
using var service = ServiceAsyncCancel.Create();
using var cts = new CancellationTokenSource();
var task = service.LongRunning(100, 50, cts.Token);
cts.CancelAfter(200);
await Assert.ThrowsAnyAsync<Exception>(async () => await task);
```


## Plugins

When you declare a `plugin!` macro in Rust, the C# backend generates one or more files, usually these:

| File | Contents |
|---|---|
| **Interop.Common.cs** | Shared types, marshallers, utility classes |
| **Interop.User.cs** | Interfaces you implement, type definitions |
| **Interop.Plugin.cs** | Internal trampolines with `[UnmanagedCallersOnly]` that Rust calls |

We also create a **Plugin.cs** for you to implement. The generated trampolines bridge between Rust's FFI calls and your managed C# code.

### Static Functions

The simplest plugin exports static functions. Implement the generated `IPlugin` interface.

```csharp
// Plugin.cs — you implement this
public class Plugin : IPlugin
{
    public static void PrimitiveVoid() { }

    public static uint PrimitiveU32(uint x)
    {
        return x + 1;
    }
}
```


### Services

Each service becomes a C# class implementing a generated interface. 

```csharp
// Plugin.cs — you implement this
partial class ServiceA : IServiceA<ServiceA>
{
    public static ServiceA Create()
    {
        return new();
    }

    public uint Call(uint x)
    {
        return x + 1;
    }
}
```

Rust uses the service like any other object — `Drop` calls the destructor automatically:

```rust
let svc = plugin.service_a_create();
svc.call(42);
// svc dropped here → ServiceA instance is probably GCed (if no other user exists)
```


### Async

Async plugin functions use `Task` / `Task<T>` and receive a `CancellationToken`. When Rust drops the future, the token is cancelled.

```csharp
partial class AsyncBasic : IAsyncBasic<AsyncBasic>
{
    public static AsyncBasic Create() => new();

    public async Task<uint> AddOne(uint x, CancellationToken ct)
    {
        await Task.Yield();
        return x + 1;
    }
}
```




### Exception Handling

General exceptions are caught by the generated trampolines and forwarded to Rust via the global exception handler, preventing process crashes.

```csharp
public class Plugin : IPlugin
{
    public static void Panic()
    {
        throw new Exception("Something went wrong");
    }
}
```

```rust
rt.exception_handler(|msg| eprintln!("C# exception: {msg}"));
```

However, this is a last-resort mechanism and strongly discouraged. Instead, use `Try<T>` for structured exception handling. It is a type alias for `ffi::Result<T, ExceptionError>`. When the C# backend sees a function returning `Try<T>`, it generates typed `catch` blocks that automatically capture exceptions and convert them into an `ExceptionError`. The C# side just returns `T` directly — no wrapping needed.

Declare it in the plugin macro:

```rust
use interoptopus_csharp::pattern::Try;

plugin!(MyPlugin {
    fn compute(x: u32) -> Try<u32>;
});
```

The C# implementation simply returns the value. Exceptions are caught automatically by the generated trampoline:

```csharp
partial class Plugin : IPlugin
{
    public static uint Compute(uint x)
    {
        return x + 1;
    }
}
```

On the Rust side, use `.ok()` from `TryExtension` to convert into a standard `Result` for `?`-based error propagation:

```rust
use interoptopus_csharp::pattern::TryExtension;

let value = plugin.compute(42).ok()?;
```

