use crate::backend::Prettifier;
use crate::lang::{Meta, Type};

/// Indicates the final desired return type in FFI'ed user code.
pub enum SugaredReturnType {
    Sync(Type),
    Async(Type),
}

impl SugaredReturnType {
    #[must_use]
    pub fn is_async(&self) -> bool {
        matches!(self, Self::Async(_))
    }

    #[must_use]
    pub fn is_sync(&self) -> bool {
        matches!(self, Self::Sync(_))
    }
}

/// A named, exported `#[no_mangle] extern "C" fn f()` function.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Function {
    name: String,
    meta: Meta,
    signature: Signature,
}

impl Function {
    #[must_use]
    pub const fn new(name: String, signature: Signature, meta: Meta) -> Self {
        Self { name, meta, signature }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[must_use]
    pub const fn signature(&self) -> &Signature {
        &self.signature
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    #[must_use]
    pub fn prettifier(&self) -> Prettifier {
        Prettifier::from_rust_lower(self.name())
    }
}

/// Represents multiple `in` and a single `out` parameters.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Signature {
    params: Vec<Parameter>,
    rval: Type,
}

impl Signature {
    #[must_use]
    pub const fn new(params: Vec<Parameter>, rval: Type) -> Self {
        Self { params, rval }
    }

    #[must_use]
    pub fn params(&self) -> &[Parameter] {
        &self.params
    }

    #[must_use]
    pub const fn rval(&self) -> &Type {
        &self.rval
    }
}

/// Parameters of a [`Signature`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Parameter {
    name: String,
    the_type: Type,
}

impl Parameter {
    #[must_use]
    pub const fn new(name: String, the_type: Type) -> Self {
        Self { name, the_type }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn the_type(&self) -> &Type {
        &self.the_type
    }
}
