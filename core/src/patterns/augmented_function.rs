use crate::lang::c::{Meta, Parameter, Function, CType, FunctionSignature};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AugmentedFunction {
    name: String,
    meta: Meta,
    signature: AugmentedFunctionSignature,
}

/// Represents multiple `in` and a single `out` parameters.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct AugmentedFunctionSignature {
    params: Vec<Parameter>,

    // NOTE: This is of type Vec<Parameter> and not Vec<CType> since in the FFI representation of C
    // we need to name the output parameters
    rvals: Vec<Parameter>,

    // TODO: here you can put errors that will be translated into exceptions etc by whatever back
    // end is being used
}

impl AugmentedFunctionSignature {
    fn fallback_type(&self) -> FunctionSignature {
        if self.rvals.len() == 1 {
            FunctionSignature::new(self.params.clone(), self.rvals[0].the_type().to_owned())
        } else {
            let mut params = self.params.clone();
            params.extend(
                self.rvals.iter().map(|rval| Parameter::new(
                    rval.name().to_owned(),
                    CType::ReadWritePointer(Box::new(rval.the_type().to_owned()))
                )
            ));

            FunctionSignature::new(params, CType::Primitive(crate::lang::c::PrimitiveType::Void))
        }
    }
}

impl AugmentedFunction {
    pub fn fallback_type(&self) -> Function {
        let f = Function::new(
            self.name.clone(), self.signature.fallback_type(), self.meta.clone()
        );

        f
    }
}
