//! Classifies each service constructor by the shape of its FFI return value.
//!
//! Two shapes are supported:
//! - `Bare`: the constructor returns `*const Service` directly (bare `Self`).
//! - `ResultWrapped`: the constructor returns `ffi::Result<*const Service, _>`.
//!
//! This classification is derived purely from the function's rval `TypeKind` and
//! is consumed by output passes (e.g. wrapper rendering) to choose between the
//! `.AsOk()`-suffixed and direct assignment shapes.

use crate::lang::FunctionId;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CtorShape {
    /// Constructor returns `*const Service` directly.
    Bare,
    /// Constructor returns `ffi::Result<*const Service, _>`.
    ResultWrapped,
}

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    shapes: HashMap<FunctionId, CtorShape>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, shapes: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        services: &model::common::service::all::Pass,
        fns: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (_, service) in services.iter() {
            for ctor_fn_id in &service.sources.ctors {
                if self.shapes.contains_key(ctor_fn_id) {
                    continue;
                }

                let Some(ctor_fn) = fns.get(*ctor_fn_id) else { continue };
                let Some(rval_ty) = types.get(ctor_fn.signature.rval) else { continue };

                let shape = match &rval_ty.kind {
                    TypeKind::Pointer(p) => {
                        let Some(target) = types.get(p.target) else { continue };
                        if matches!(&target.kind, TypeKind::Service) { CtorShape::Bare } else { continue }
                    }
                    TypeKind::TypePattern(TypePattern::Result(ok_ty, _, _)) => {
                        let Some(ok) = types.get(*ok_ty) else { continue };
                        let TypeKind::Pointer(p) = &ok.kind else { continue };
                        let Some(target) = types.get(p.target) else { continue };
                        if matches!(&target.kind, TypeKind::Service) {
                            CtorShape::ResultWrapped
                        } else {
                            continue;
                        }
                    }
                    _ => continue,
                };

                self.shapes.insert(*ctor_fn_id, shape);
                outcome.changed();
            }
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn get(&self, fn_id: FunctionId) -> Option<CtorShape> {
        self.shapes.get(&fn_id).copied()
    }
}
