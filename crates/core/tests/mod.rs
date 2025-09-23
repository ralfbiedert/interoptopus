use trybuild::TestCases;

mod inventory {
    mod basic;
    mod forbidden;
}

#[test]
fn proc() {
    let t = TestCases::new();

    // fn
    t.compile_fail("tests/proc/fn/attr_extern_c.rs");
    t.compile_fail("tests/proc/fn/attr_no_mangle.rs");
    t.pass("tests/proc/fn/basic.rs");
    t.compile_fail("tests/proc/fn/forbidden_fn.rs");
    t.compile_fail("tests/proc/fn/forbidden_param.rs");
    t.compile_fail("tests/proc/fn/on_ty.rs");
    t.pass("tests/proc/fn/ref.rs");
    t.pass("tests/proc/fn/ref_lt.rs");
    t.pass("tests/proc/fn/unsafe.rs");
    t.pass("tests/proc/fn/wildcard.rs");

    // svc
    t.pass("tests/proc/svc/async_basic.rs");
    t.compile_fail("tests/proc/svc/async_mut_self.rs");
    t.pass("tests/proc/svc/basic.rs");
    t.pass("tests/proc/svc/ctor.rs");
    t.compile_fail("tests/proc/svc/ctor_rval_bad_self.rs");
    t.compile_fail("tests/proc/svc/ctor_rval_omit.rs");

    // ty
    t.pass("tests/proc/ty/basic.rs");
    t.compile_fail("tests/proc/ty/empty_struct.rs");
    t.compile_fail("tests/proc/ty/empty_unit.rs");
    t.compile_fail("tests/proc/ty/forbidden_field.rs");
    t.pass("tests/proc/ty/generic_basic.rs");
    t.compile_fail("tests/proc/ty/generic_no_typeinfo.rs");
    t.pass("tests/proc/ty/opaque_basic.rs");
    t.pass("tests/proc/ty/svc_basic.rs");
}
