use crate::lang::c::{CType, Function, OpaqueType};
use crate::patterns::TypePattern;

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
        let constructor_fist_parameter = self
            .constructor
            .signature()
            .params()
            .get(0)
            .expect("Constructor for must have at least one parameter.");

        match &constructor_fist_parameter.the_type() {
            CType::ReadWritePointer(x) => match **x {
                CType::ReadWritePointer(ref x) => match **x {
                    CType::Opaque(_) => {}
                    _ => panic!("First parameter must be opaque type"),
                },
                _ => panic!("First parameter must be opaque type"),
            },
            CType::Opaque(_) => {}
            _ => panic!("First parameter must be RwPointer(RwPointer(Opaque)) type"),
        }

        let destructor_first_parameter = self
            .destructor
            .signature()
            .params()
            .get(0)
            .expect("Constructor for must have at least one parameter.");

        match &destructor_first_parameter.the_type() {
            CType::ReadWritePointer(x) => match **x {
                CType::ReadWritePointer(ref x) => match **x {
                    CType::Opaque(_) => {}
                    _ => panic!("First parameter must be opaque type"),
                },
                _ => panic!("First parameter must be opaque type"),
            },
            CType::Opaque(_) => {}
            _ => panic!("First parameter must be RwPointer(RwPointer(Opaque)) type"),
        }

        match self.constructor.signature().rval() {
            CType::Pattern(TypePattern::SuccessEnum(_)) => {}
            _ => panic!("Constructor must return a success enum."),
        }

        match self.destructor.signature().rval() {
            CType::Pattern(TypePattern::SuccessEnum(_)) => {}
            _ => panic!("Destructor must return a success enum."),
        }
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

/// ```ignore
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

            let rval = interoptopus::patterns::class::Class::new(
                <$the_class>::type_info().as_opaque_type().cloned().expect("Class types may only be opaque types."),
                ctor, dtor, methods
            );

            rval.assert_valid();
            rval
        }
    };
}
