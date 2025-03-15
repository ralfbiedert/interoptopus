use crate::lang::CType;

/// A (C-style) `type[N]` containing a fixed number of elements of the same type.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ArrayType {
    array_type: Box<CType>,
    len: usize,
}

impl ArrayType {
    #[must_use]
    pub fn new(array_type: CType, len: usize) -> Self {
        Self { array_type: Box::new(array_type), len }
    }

    #[must_use]
    pub fn rust_name(&self) -> String {
        format!("{}[{}]", self.array_type.name_within_lib(), self.len)
    }

    #[must_use]
    pub const fn array_type(&self) -> &CType {
        &self.array_type
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}
