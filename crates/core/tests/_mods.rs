mod inventory {
    mod basic;
    mod forbidden;
}

mod proc_macros {
    mod fn_basic;
    mod svc_basic;
    mod ty_basic;
}

// #[test]
// fn ui() {
//     let x = quote! {
//         #[ffi_function]
//         fn foo() {}
//     };
//
//     let output = interoptopus_proc::ffi_function(x);
// }
