use crate::service::args::FfiServiceArgs;
use crate::service::model::ServiceModel;
use syn::parse_quote;

#[test]
fn sync_service_uses_arc_storage() {
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
