//! Helpers for API loading patterns as [described in this blog post](https://anteru.net/blog/2016/designing-c-apis-in-2016/).<sup>🚧</sup>
//!
//! See the macro [api_entry](crate::api_entry) for details.

/// Defines a new API entry function and corresponding struct.
///
/// The resulting function can be called via FFI and will return a struct, pointing to all exported functions.
///
/// # Example
///
/// In this example, other languages can call `my_api_init_v1` to obtain a struct of type `MyAPIv1`
/// exporting `f1` and `f2`.
///
/// ```rust
/// use interoptopus::{api_entry, ffi_function};
///
/// #[ffi_function]
/// extern "C" fn f1() {}
///
/// #[ffi_function]
/// extern "C" fn f2() {}
///
/// api_entry!(MyAPIv1, my_api_init_v1, [f1, f2]);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! api_entry {
    (
        $struct:ident,
        $init:ident,
        [
            $($function:ident),*
        ]
    ) => {
        pub struct $struct {
            $($function: <$function as interoptopus::lang::FunctionInfo>::Signature,)*
        }

        #[interoptopus::ffi_function]
        pub fn $init(api: *mut $struct) {
            if api.is_null() {
                return;
            }

            let s = $struct {
                $(
                    $function: $function,
                )*
            };

            unsafe { *api = s; }
        }


        unsafe impl interoptopus::lang::TypeInfo for $struct {
            fn type_info() -> interoptopus::lang::CType {
                let mut fields = Vec::new();

                $(
                    {
                        use interoptopus::lang::FunctionInfo;
                        use $function as x;
                        let function: interoptopus::lang::Function = x::function_info();
                        let t = interoptopus::lang::CType::FnPointer(interoptopus::lang::FnPointerType::new(function.signature().clone()));
                        let field = interoptopus::lang::Field::new(function.name().to_string(), t);
                        fields.push(field);
                    }
                )*

                let composite = interoptopus::lang::CompositeType::new(stringify!($struct).to_string(), fields);
                interoptopus::lang::CType::Composite(composite)
            }
        }
    };
}
