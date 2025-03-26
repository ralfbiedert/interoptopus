use crate::lang::function::Signature;

/// Represents `extern "C" fn()` types in Rust and `(*f)().` in C.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FnPointer {
    name: Option<String>,
    signature: Box<Signature>,
}

impl FnPointer {
    #[must_use]
    pub fn new(signature: Signature) -> Self {
        Self { signature: Box::new(signature), name: None }
    }

    #[must_use]
    pub fn new_named(signature: Signature, name: String) -> Self {
        Self { signature: Box::new(signature), name: Some(name) }
    }

    #[must_use]
    pub const fn signature(&self) -> &Signature {
        &self.signature
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[must_use]
    pub fn internal_name(&self) -> String {
        let signature = self.signature();
        let params = signature.params().iter().map(|x| x.the_type().name_within_lib()).collect::<Vec<_>>().join(",");
        let rval = signature.rval().name_within_lib();

        format!("fn({params}) -> {rval}")
    }

    #[must_use]
    pub fn rust_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| self.internal_name())
    }
}
