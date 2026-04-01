use trybuild::TestCases;

#[test]
fn all_ui_tests() {
    let t = TestCases::new();

    // proc/const
    t.pass("tests/ui/proc/const/basic.rs");

    // proc/fn
    t.compile_fail("tests/ui/proc/fn/attr_extern_c.rs");
    t.compile_fail("tests/ui/proc/fn/attr_no_mangle.rs");
    t.pass("tests/ui/proc/fn/basic.rs");
    t.compile_fail("tests/ui/proc/fn/forbidden_fn.rs");
    t.compile_fail("tests/ui/proc/fn/forbidden_param.rs");
    t.pass("tests/ui/proc/fn/module.rs");
    t.compile_fail("tests/ui/proc/fn/on_ty.rs");
    t.pass("tests/ui/proc/fn/ref.rs");
    t.pass("tests/ui/proc/fn/ref_lt.rs");
    t.pass("tests/ui/proc/fn/unsafe.rs");
    t.pass("tests/ui/proc/fn/wildcard.rs");

    // proc/svc
    t.pass("tests/ui/proc/svc/async_basic.rs");
    t.pass("tests/ui/proc/svc/async_double.rs");
    t.compile_fail("tests/ui/proc/svc/async_non_send.rs");
    t.compile_fail("tests/ui/proc/svc/async_mut_self.rs");
    t.compile_fail("tests/ui/proc/svc/async_ref_self.rs");
    t.pass("tests/ui/proc/svc/basic.rs");
    t.pass("tests/ui/proc/svc/ctor.rs");
    t.compile_fail("tests/ui/proc/svc/ctor_forbidden_name.rs");
    t.compile_fail("tests/ui/proc/svc/ctor_rval_bad_self.rs");
    t.compile_fail("tests/ui/proc/svc/ctor_rval_omit.rs");
    t.pass("tests/ui/proc/svc/lifetime.rs");
    // t.compile_fail("tests/ui/proc/svc/module.rs"); TODO: later
    t.compile_fail("tests/ui/proc/svc/opaque.rs"); // TODO: should have better error warning about `opaque`

    // proc/plugin
    t.compile_fail("tests/ui/proc/plugin/svc_static_method.rs");

    // proc/ty
    t.compile_fail("tests/ui/proc/ty/empty_struct.rs");
    t.compile_fail("tests/ui/proc/ty/empty_unit.rs");
    t.compile_fail("tests/ui/proc/ty/field_non_wire.rs");
    t.compile_fail("tests/ui/proc/ty/forbidden_field.rs");
    t.pass("tests/ui/proc/ty/generic_basic.rs");
    t.compile_fail("tests/ui/proc/ty/generic_no_typeinfo.rs");
    t.pass("tests/ui/proc/ty/module.rs");
    t.pass("tests/ui/proc/ty/opaque_basic.rs");
    t.pass("tests/ui/proc/ty/opaque_no_typeinfo.rs");
    t.pass("tests/ui/proc/ty/service_basic.rs");
    t.pass("tests/ui/proc/ty/service_no_typeinfo.rs");

    // wire
    t.compile_fail("tests/ui/wire/invalid.rs");
}
