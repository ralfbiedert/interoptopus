use crate::lang2::meta::{Docs, Emission, Visibility};
use crate::lang2::types::TypeId;
use crate::new_id;

new_id!(FunctionId);

pub struct Argument {
    pub string: String,
    pub ty: TypeId,
}

pub struct Signature {
    pub arguments: Vec<Argument>,
    pub rval: TypeId,
}

pub struct Function {
    pub name: String,
    pub visibility: Visibility,
    pub docs: Docs,
    pub emission: Emission,
    pub signature: Signature,
}
