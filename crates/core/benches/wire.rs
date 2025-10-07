#![feature(test)] // #[bench] is still experimental

extern crate test;

use std::io::{Cursor, Write};
use test::Bencher;

trait Pointer {
    fn size_of(&self) -> usize;
    unsafe fn write(&self, dst: *mut u8);
}

trait Writer {
    fn write(&self, dst: impl Write);
}

#[repr(C)]
struct Bar {
    v: Vec<u8>,
    f: f32,
}

impl Pointer for Bar {
    fn size_of(&self) -> usize {
        size_of::<u64>() + self.v.len() * size_of::<f32>()
    }

    unsafe fn write(&self, mut dst: *mut u8) {
        std::ptr::write_unaligned(dst.cast(), self.v.len() as u64);
        dst = dst.add(8);
        std::ptr::copy_nonoverlapping(self.v.as_ptr(), dst.cast(), self.v.len());
        dst = dst.add(self.v.len());
        std::ptr::write_unaligned(dst.cast(), self.f);
    }
}

impl Writer for Bar {
    fn write(&self, dst: impl Write) {
        dst.
    }
}

#[repr(C)]
struct Baz {
    f: f32,
    u: u32,
    i1: i32,
    b1: u8,
    b2: u8,
    i2: i32,
}

impl Pointer for Baz {
    fn size_of(&self) -> usize {
        size_of::<Self>()
    }

    unsafe fn write(&self, dst: *mut u8) {
        std::ptr::write_unaligned(dst.cast(), self);
    }
}

#[repr(C)]
struct Foo {
    // s: String,
    u: u32,
    b: Bar,
    z: Baz,
}

impl Pointer for Foo {
    fn size_of(&self) -> usize {
        size_of::<u32>() + self.b.size_of() + self.z.size_of()
    }

    unsafe fn write(&self, mut dst: *mut u8) {
        std::ptr::write_unaligned(dst.cast(), self.u);
        dst = dst.add(4);
        self.b.write(dst as *mut u8);
        dst = dst.add(self.b.size_of());
        self.z.write(dst as *mut u8);
    }
}

#[bench]
fn write_ptr(b: &mut Bencher) {
    let foo = Foo { u: 0, b: Bar { v: vec![1, 2, 3, 4, 5], f: 0.0 }, z: Baz { f: 0.0, u: 0, i1: 0, b1: 0, b2: 0, i2: 0 } };
    let mut dst = vec![0u8; 1024];

    b.iter(|| {
        unsafe { foo.write(dst.as_mut_ptr()) };
    })
}
