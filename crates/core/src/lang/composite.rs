use crate::lang::{Documentation, Meta, Primitive, Type, Visibility};

/// How a struct is laid out in memory.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Layout {
    C,
    Transparent,
    Packed,
    Opaque,
    /// For use with enum discriminant.
    Primitive(Primitive),
}

/// How a type is represented in memory.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Representation {
    layout: Layout,
    alignment: Option<usize>,
}

impl Default for Representation {
    fn default() -> Self {
        Self { layout: Layout::C, alignment: None }
    }
}

impl Representation {
    #[must_use]
    pub const fn new(layout: Layout, alignment: Option<usize>) -> Self {
        Self { layout, alignment }
    }

    #[must_use]
    pub const fn layout(&self) -> Layout {
        self.layout
    }

    #[must_use]
    pub const fn alignment(&self) -> Option<usize> {
        self.alignment
    }
}

/// Used for Rust and C `struct` with named fields, must be `#[repr(C)]`.
///
/// Might translate to a struct or class in another language, equivalent on
/// C-level to:
///
/// ```ignore
/// typedef struct MyComposite
/// {
///     int   field_1;
///     float field_2;
///     char  field_3;
///     // ...
/// } MyComposite;
/// ```
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Composite {
    name: String,
    fields: Vec<Field>,
    repr: Representation,
    meta: Meta,
}

impl Composite {
    /// Creates a new composite with the given name and fields and no documentation.
    #[must_use]
    pub fn new(name: String, fields: Vec<Field>) -> Self {
        Self::with_meta(name, fields, Meta::new())
    }

    /// Creates a new composite with the given name and type-level documentation.
    #[must_use]
    pub fn with_meta(name: String, fields: Vec<Field>, meta: Meta) -> Self {
        Self { name, fields, meta, repr: Representation::default() }
    }

    /// Creates a new composite with the given name and type-level documentation.
    #[must_use]
    pub const fn with_meta_repr(name: String, fields: Vec<Field>, meta: Meta, repr: Representation) -> Self {
        Self { name, fields, repr, meta }
    }

    /// Gets the type's name.
    #[must_use]
    pub fn rust_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    /// If this were a wrapper over a pointer type, get the type of what we're pointing go.
    #[must_use]
    pub fn try_deref_pointer(&self) -> Option<Type> {
        self.fields().first()?.the_type().try_deref_pointer().cloned()
    }

    /// True if this struct has no contained fields (which happens to be illegal in C99).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    #[must_use]
    pub const fn repr(&self) -> &Representation {
        &self.repr
    }

    #[must_use]
    pub fn into_ctype(&self) -> Type {
        Type::Composite(self.clone())
    }
}

/// Fields of a [`Composite`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Field {
    name: String,
    visibility: Visibility,
    the_type: Type,
    documentation: Documentation,
}

impl Field {
    #[must_use]
    pub fn new(name: String, the_type: Type) -> Self {
        Self::with_documentation(name, the_type, Visibility::Public, Documentation::new())
    }

    #[must_use]
    pub const fn with_documentation(name: String, the_type: Type, visibility: Visibility, documentation: Documentation) -> Self {
        Self { name, visibility, the_type, documentation }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn the_type(&self) -> &Type {
        &self.the_type
    }

    #[must_use]
    pub const fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    #[must_use]
    pub const fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}

/// A named `struct` that becomes a fieldless `typedef struct S S;` in C.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Opaque {
    name: String,
    meta: Meta,
}

impl Opaque {
    #[must_use]
    pub const fn new(name: String, meta: Meta) -> Self {
        Self { name, meta }
    }

    #[must_use]
    pub fn rust_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }
}
