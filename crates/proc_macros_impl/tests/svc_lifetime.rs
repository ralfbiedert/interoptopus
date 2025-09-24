use quote::quote;
use syn::{parse_quote, ItemImpl};

mod util;

#[test]
fn supports_lifetimes() {
    let item: ItemImpl = parse_quote! {
        #[ffi]
            impl Service {
                pub fn new() -> ffi::Result<Self, Error> {
                    ffi::Ok(Self { data: Vec::new() })
                }

                #[allow(clippy::needless_lifetimes)]
                pub fn l1<'a, 'b>(&mut self, x: &u8, _y: &mut u8, _slice: ffi::Slice<'a, u8>, _slice2: ffi::Slice<'b, u8>) -> u8 {
                    *x
                }

                pub fn return_slice(&mut self) -> ffi::Slice<'_, u32> {
                    self.data.as_slice().into()
                }
            }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
