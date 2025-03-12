#![doc(hidden)]

use crate::lang::c::Function;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Builtins {
    functions: Vec<Function>,
}

impl Builtins {
    #[must_use]
    pub fn new(functions: Vec<Function>) -> Self {
        Self { functions }
    }

    #[must_use]
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }
}

/// Emits helper functions used by String & co.
#[macro_export]
macro_rules! builtins {
    () => {{
        use interoptopus::lang::rust::FunctionInfo;

        #[$crate::ffi_function]
        pub fn interoptopus_string_create(utf8: *const ::std::ffi::c_void, len: u64, rval: &mut ::std::mem::MaybeUninit<$crate::pattern::string::String>) -> i64 {
            let slice = if utf8.is_null() {
                &[]
            } else {
                unsafe { ::std::slice::from_raw_parts::<u8>(utf8.cast(), len as usize) }
            };
            let vec = slice.to_vec();
            let string = unsafe { String::from_utf8_unchecked(vec) };
            rval.write($crate::pattern::string::String::from_string(string));
            0
        }

        #[$crate::ffi_function]
        pub fn interoptopus_string_destroy(utf8: $crate::pattern::string::String) -> i64 {
            0
        }

        let builtins = $crate::pattern::builtins::Builtins::new(vec![interoptopus_string_create::function_info(), interoptopus_string_destroy::function_info()]);
        let pattern = $crate::pattern::LibraryPattern::Builtins(builtins);
        $crate::inventory::Symbol::Pattern(pattern)
    }};
}
