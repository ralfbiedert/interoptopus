use crate::lang::c::{Function, OpaqueType};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Class {
    the_type: OpaqueType,
    constructor: Function,
    destructor: Function,
    methods: Vec<Function>,
}

impl Class {
    pub fn new(the_type: OpaqueType, constructor: Function, destructor: Function, methods: Vec<Function>) -> Self {
        Self {
            the_type,
            constructor,
            destructor,
            methods,
        }
    }

    /// Checks if the signature of this class is compatible with the `Class` pattern, panic with
    /// error message otherwise.
    ///
    /// This function is mainly called during compile time therefore panicking with a good error
    /// message is beneficial.
    pub fn assert_valid(&self) {
        let first = self
            .constructor
            .signature()
            .params()
            .get(0)
            .expect("Constructor for must have at least one parameter.");
        // first.the_type().
    }

    pub fn the_type(&self) -> &OpaqueType {
        &self.the_type
    }

    pub fn constructor(&self) -> &Function {
        &self.constructor
    }

    pub fn destructor(&self) -> &Function {
        &self.destructor
    }

    pub fn methods(&self) -> &[Function] {
        &self.methods
    }
}

/// ```
/// pattern_class!(
///     my_class_pattern_context,
///     types::Context,
///     functions::pattern_class_create,
///     functions::pattern_class_destroy
///     [
///         functions::pattern_class_method,
///     ]
/// );
/// ```
#[macro_export]
macro_rules! pattern_class {
    (
        $pattern_name:ident,
        $the_class:path,
        $constructor:path,
        $destructor:path,
        [$(
            $method:path
        ),*]
    ) => {
        fn $pattern_name() -> interoptopus::patterns::class::Class {
            use interoptopus::lang::rust::CTypeInfo;
            use interoptopus::lang::rust::FunctionInfo;

            let mut methods = Vec::new();

            {
                $({
                    use $method as x;
                    methods.push(x::function_info());
                })*
            }

            let ctor = {
                use $constructor as x;
                x::function_info()
            };

            let dtor = {
                use $destructor as x;
                x::function_info()
            };

            interoptopus::patterns::class::Class::new(
                <$the_class>::type_info().as_opaque_type().cloned().expect("Class types may only be opaque types."),
                ctor, dtor, methods
            )
        }
    };
}
