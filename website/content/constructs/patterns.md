+++
title = "Patterns"
weight = 20
+++

Patterns are higher-level abstractions for common interop scenarios. They can generally be used in both forward and reverse interop scenarios, although not all patterns are supported everywhere. Check the `#[ffi]` / `plugin!()` API documentation for the latest details. 

## Slices

Borrow contiguous data across the FFI boundary without copying.

```rust
#[ffi]
pub fn pattern_ffi_slice(slice: ffi::Slice<u32>) -> u32 {
    slice.as_slice().len() as u32
}

#[ffi]
pub fn pattern_ffi_slice_mut(mut slice: ffi::SliceMut<u8>, callback: CallbackSliceMut) {
    if let [x, ..] = slice.as_slice_mut() { *x += 1; }
    callback.call(slice);
}
```

## Option

Nullable values.

```rust
#[ffi]
pub fn pattern_ffi_option(x: ffi::Option<Inner>) -> ffi::Option<Inner> { x }
```

Note, this refers to the `ffi::Option` type. We also support `std::option::Option` inside `Wire<T>` directly.


## Result

Error handling with a custom error enum.

```rust
#[ffi]
pub enum Error { Fail }

#[ffi]
pub fn pattern_result(x: ffi::Result<u32, Error>) -> ffi::Result<u32, Error> { x }
```

## Strings

We have an owned UTF-8 string type that can be faster than marshalling in certain scenarios.

```rust
#[ffi]
pub fn utf8_string(x: ffi::String) -> ffi::String { x }
```

Note, this refers to the `ffi::String` type. We also support `std::string::String` inside `Wire<T>` directly.

## Vectors

Owned, growable arrays.

```rust
#[ffi]
pub fn pattern_vec() -> ffi::Vec<u32> {
    vec![1, 2, 3].into()
}
```

Note, this refers to the `ffi::Vec` type. We also support `std::vec::Vec` inside `Wire<T>` directly.

## Callbacks

Define named typed function-pointer callbacks (e.g., delegates in C#) with the `callback!` macro.

```rust
callback!(MyCallback(value: u32) -> u32);
callback!(SumDelegate(x: i32, y: i32) -> i32);
callback!(StringCallback(s: ffi::String));

#[ffi]
pub fn pattern_callback(callback: MyCallback, x: u32) -> u32 {
    callback.call(x)
}
```
