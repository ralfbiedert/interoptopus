// This is proto_benchy.dll doing three variants of the API:
// - one is Protobuf ser/de based
// - one is interoptopus::ffi based
// - one is Wire<T> based
// The Wire<T> version does NOT need any protobuf files, and is defined
// solely using Rust types.

pub mod ffi;
mod protobuf;
mod protobuf_models;
// mod wire;
