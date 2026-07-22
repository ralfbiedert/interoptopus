use crate::service::args::FfiServiceArgs;
use crate::service::model::{ServiceModel, ServiceOwnership};
use syn::parse_quote;

#[test]
fn shared_service_uses_arc_storage() {
    let input = parse_quote! {
        impl RuntimeService {
            pub fn new() -> Self {
                Self
            }
        }
    };
    let model = ServiceModel::from_impl_item(input, FfiServiceArgs::default()).unwrap();
    let emitted = model.emit_ffi_functions().to_string();

    assert!(emitted.contains("Arc :: into_raw"));
    assert!(emitted.contains("Arc :: from_raw"));
    assert!(!emitted.contains("Box :: into_raw"));
    assert!(!emitted.contains("Box :: from_raw"));
}

#[test]
fn mutable_service_uses_box_storage() {
    let input = parse_quote! {
        impl MutableService {
            pub fn new() -> Self {
                Self
            }

            pub fn update(&mut self) {}
        }
    };
    let model = ServiceModel::from_impl_item(input, FfiServiceArgs::default()).unwrap();
    let emitted = model.emit_ffi_functions().to_string();
    let marker = model.emit_async_runtime_service_impl().to_string();

    assert!(emitted.contains("Box :: into_raw"));
    assert!(emitted.contains("Box :: from_raw"));
    assert!(!emitted.contains("Arc :: into_raw"));
    assert!(!emitted.contains("Arc :: from_raw"));
    assert!(marker.is_empty());
}

#[test]
fn shared_service_is_eligible_as_async_runtime() {
    let input = parse_quote! {
        impl RuntimeService {
            pub fn new() -> Self {
                Self
            }

            pub fn inspect(&self) {}
        }
    };
    let model = ServiceModel::from_impl_item(input, FfiServiceArgs::default()).unwrap();
    let marker = model.emit_async_runtime_service_impl().to_string();

    assert!(marker.contains("AsyncRuntimeService for RuntimeService"));
}

#[test]
fn explicit_mutable_receiver_uses_box_storage() {
    let input = parse_quote! {
        impl MutableService {
            pub fn new() -> Self {
                Self
            }

            pub fn update(self: &mut Self) {}
        }
    };
    let model = ServiceModel::from_impl_item(input, FfiServiceArgs::default()).unwrap();

    assert_eq!(model.ownership, ServiceOwnership::Unique);
}

#[test]
fn async_runtime_marker_preserves_where_clause() {
    let input = parse_quote! {
        impl<'a> RuntimeService<'a>
        where
            'a: 'static,
        {
            pub fn inspect(&self) {}
        }
    };
    let model = ServiceModel::from_impl_item(input, FfiServiceArgs::default()).unwrap();
    let marker = model.emit_async_runtime_service_impl().to_string();

    assert!(marker.contains("for RuntimeService < 'a > where 'a : 'static"));
}
