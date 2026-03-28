use crate::pattern::assert_looks_like_exception_name;
use interoptopus::inventory::{Inventory, TypeId, hash_str};
use interoptopus::lang::meta::{Docs, Emission, FileEmission, Visibility};
use interoptopus::lang::types::{Field, Repr, Struct, Type, TypeInfo, TypeKind};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// A registered C# exception type for structured error mapping.
///
/// Each instance pairs a fully-qualified C# class name (e.g.
/// `"System.IO.FileNotFoundException"`) with a deterministic hash-based ID.
/// The backend uses these to generate typed `catch` clauses in `FromCall`
/// helpers — see the [module-level docs](super) for details.
///
/// Construct with [`Exception::new`] and pass to the builder via `.exception(…)`.
pub struct Exception(u64, &'static str);

impl Exception {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(fqp: &'static str) -> Self {
        assert_looks_like_exception_name(fqp);
        Self(hash_str(fqp) as u64, fqp)
    }

    /// The hash-derived ID for this exception type.
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.0
    }

    /// The fully-qualified C# exception class name (e.g. `"System.InvalidOperationException"`).
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.1
    }
}

/// Wire-level error type carried inside [`Try<T>`](super::Try).
///
/// On the C# side this is emitted as `DotnetException`, a struct with a single
/// `exception_id` field.  The ID is either:
///
/// - The hash of a registered [`Exception`] name (set by a typed `catch` block), or
/// - `0` for unknown / unregistered exceptions.
///
/// On the Rust side this type is mainly encountered when calling plugin methods
/// that return `Try<T>` — use [`TryExtension::ok()`](super::TryExtension::ok)
/// to convert the result for `?`-based error propagation.
pub struct ExceptionError {
    exception_id: u64,
}

impl Debug for ExceptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExceptionError({})", self.exception_id)
    }
}

impl Display for ExceptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "exception error (id: {})", self.exception_id)
    }
}

impl Error for ExceptionError {}

impl ExceptionError {
    #[must_use]
    pub const fn unknown() -> Self {
        Self { exception_id: 0 }
    }
}

unsafe impl TypeInfo for ExceptionError {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;
    const SERVICE_SAFE: bool = true;
    const SERVICE_CTOR_SAFE: bool = true;

    fn id() -> TypeId {
        TypeId::new(0x6CC48127B46F1B58B8D4FCFC55617873)
    }

    fn kind() -> TypeKind {
        let s =
            Struct { fields: vec![Field { name: "exception_id".to_string(), docs: Docs::default(), visibility: Visibility::default(), ty: u64::id() }], repr: Repr::c() };
        TypeKind::Struct(s)
    }

    fn ty() -> Type {
        Type {
            name: "DotnetException".to_string(),
            visibility: Visibility::Public,
            docs: Docs::default(),
            emission: Emission::FileEmission(FileEmission::Common),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut impl Inventory) {
        u64::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}
