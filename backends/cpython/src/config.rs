/// Configures Python code generation.
#[derive(Clone, Debug)]
pub struct Config {
    /// How to name the function responsible for loading the DLL, e.g., `init_api`.
    pub init_api_function_name: String,
    /// Attribute by which the `cffi` object is exposed, e.g., `ffi`.
    pub ffi_attribute: String,
    /// Namespace to put functions into, e.g., `api`.
    pub raw_fn_namespace: String,
    /// Namespace for callback helpers, e.g., `callbacks`.
    pub callback_namespace: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            init_api_function_name: "init_api".to_string(),
            ffi_attribute: "ffi".to_string(),
            raw_fn_namespace: "api".to_string(),
            callback_namespace: "callbacks".to_string(),
        }
    }
}
