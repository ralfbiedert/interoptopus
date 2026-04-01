+++
title = "Types"
weight = 10
+++

Types that can be used in both forward interop (foreign calling Rust) and reverse interop (Rust calling foreign plugins).


## Structs

Regular structs with fields. The `#[ffi]` attribute handles everything needed for the FFI boundary. Unless otherwise stated, every contained field has to be a supported `#[ffi]` type again.

```rust
#[ffi]
pub struct Vec3f32 {
    x: f32,
    y: f32,
    z: f32,
}

// Single-field tuple structs are supported too
#[ffi]
pub struct Tupled(u8);
```


## Enums

C-style enums, enums with explicit discriminants, and enums with data payloads (tagged unions). 
At the moment, only enums with a single payload item per variant are supported. Unless otherwise stated, every contained variant payload has to be a supported `#[ffi]` type again. 


```rust
#[ffi]
pub enum EnumDocumented {
    /// Variant A.
    A,
    /// Variant B.
    B,
}

#[ffi]
pub enum EnumPayload {
    A,
    B(Vec3f32),
    C(u32),
}

#[ffi]
pub enum EnumNegative {
    A = -1,
    B = 0,
    C = 1,
}
```

## Opaque Types

Mark a struct `#[ffi(opaque)]` to hide its layout from foreign code entirely. The struct can only ever be passed by pointer.

```rust
#[ffi(opaque)]
pub struct SomeContext {
    some_field: u32,
}
```

## Generic Types

Structs and enums can be generic over type parameters, lifetimes, and const generics. Each concrete monomorphisation you register becomes a distinct type in the generated bindings.

```rust
#[ffi]
pub struct Generic<'a, T: TypeInfo> {
    x: &'a T,
}

#[ffi]
pub struct FixedString<const N: usize> {
    data: [u8; N],
}
```


## Fixed-Size Arrays

Fixed-size arrays are supported as struct fields.

```rust
#[ffi]
pub struct Array {
    data: [u8; 16],
}
```


## Transparent & Packed

- `#[ffi(transparent)]` makes a newtype wrapper transparent to the FFI boundary.
- `#[ffi(packed)]` produces a tightly packed struct.

```rust
#[ffi(transparent)]
pub struct Transparent(Tupled);

#[ffi(packed)]
pub struct Packed1 {
    x: u8,
    y: u16,
}
```


## Naming & Organisation

### Renaming

Override the generated name with `#[ffi(name = "...")]`.

```rust
#[ffi(name = "EnumRenamed")]
pub enum EnumRenamedXYZ { X }

#[ffi(name = "StructRenamed")]
pub struct StructRenamedXYZ { pub e: EnumRenamedXYZ }
```

### Modules / Namespaces

Assign types to logical namespaces in the generated output.

```rust
#[ffi(module = "math")]
pub struct Vec { x: f64, y: f64 }
```

### Constants

Export Rust `const` values.

```rust
#[ffi]
pub const U8: u8 = u8::MAX;

#[ffi]
pub const COMPUTED_I32: i32 = f(i32::MAX);
```

### Skipping

Use `#[ffi::skip]` on fields or methods you want to exclude from bindings.

```rust
#[ffi]
pub struct Phantom<'a, T: 'static + TypeInfo> {
    pub x: u32,
    #[ffi::skip]
    pub p: PhantomData<&'a T>,
}
```
