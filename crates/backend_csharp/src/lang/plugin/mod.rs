pub mod interface;

/// Module for `IPlugin` and `IService`-like interfaces.
pub const PLUGIN_DEFAULT_MODULE: Module = Module::from_str("Default.Plugin");

/// Emission for `IPlugin` and `IService`-like interfaces.
pub const PLUGIN_DEFAULT_EMISSION: Emission = Emission::FileEmission(FileEmission::CustomModule(PLUGIN_DEFAULT_MODULE));

use crate::lang::{FunctionId, ServiceId};
use interoptopus::lang::meta::{Emission, FileEmission, Module};

#[derive(Debug, Clone)]
pub enum TrampolineKind {
    /// Forward to `Plugin.MethodName(args…)`.
    Raw,
    /// Allocate via `T.Create()`, wrap in `GCHandle`.
    ServiceCtor { service_id: ServiceId },
    /// Unwrap `GCHandle`, dispatch `obj.Method(args…)`.
    ServiceMethod { service_id: ServiceId },
    /// Free the `GCHandle`.
    ServiceDestructor { service_id: ServiceId },
}

#[derive(Debug, Clone)]
pub struct TrampolineEntry {
    pub fn_id: FunctionId,
    pub kind: TrampolineKind,
}
