//! Useful when `extern "C" fn()` delegate types give compile errors.

use crate::lang::c::FnPointerType;

/// Internal helper naming a generated callback type wrapper.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamedCallback {
    name: String,
    fnpointer: FnPointerType,
}

impl NamedCallback {
    /// Creates a new named callback.
    pub fn new(name: String, callback: FnPointerType) -> Self {
        Self { name, fnpointer: callback }
    }

    /// Gets the type name of this callback.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the function pointer type.
    pub fn fnpointer(&self) -> &FnPointerType {
        &self.fnpointer
    }
}

/// Defines a callback type, akin to a `fn f(T) -> R` wrapped in an Option.
#[macro_export]
macro_rules! pattern_callback {
    ($name:ident($($param:ident: $ty:ty),*) -> $rval:ty) => {
        #[repr(transparent)]
        pub struct $name(Option<extern "C" fn($($ty),*) -> $rval>);

        impl $name {
            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, $($param: $ty),*) -> $rval {
                self.0.expect("Assumed function would exist but it didn't.")($($param),*)
            }
        }

        unsafe impl interoptopus::lang::rust::CTypeInfo for $name {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::CTypeInfo;

                let rval = < $rval as CTypeInfo >::type_info();
                let params = vec![
                $(
                    interoptopus::lang::c::Parameter::new(stringify!($param).to_string(), < $ty as CTypeInfo >::type_info()),
                )*
                ];

                let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = interoptopus::lang::c::FnPointerType::new(sig);
                let named_callback = interoptopus::patterns::callbacks::NamedCallback::new(stringify!($name).to_string(), fn_pointer);

                interoptopus::lang::c::CType::Pattern(interoptopus::patterns::TypePattern::NamedCallback(named_callback))
            }
        }
    };
}
