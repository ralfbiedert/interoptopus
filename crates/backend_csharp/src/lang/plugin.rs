use crate::lang::{FunctionId, ServiceId};

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
