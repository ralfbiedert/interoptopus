use crate::lang::{FunctionId, ServiceId};

pub enum TrampolineTarget {
    Raw(FunctionId),
    Service(ServiceId),
}

pub struct Trampoline {
    pub targets: Vec<TrampolineTarget>,
}

//
// // All the functions from the inventory
// // - must know where to dispatch to
// //     - raw -> Plugin.Foo()
// //     - class -> Foo + Method
// public static class Interop { }
//
// // all functions
// public interface IPlugin { }
//
// public interface IFoo<TSelf> where TSelf : IFoo<TSelf>
// {
//     static abstract TSelf Create();
//     void Bar(int x);
//     int GetAccumulator();
// }
//
