use interoptopus::util::NamespaceMappings;

/// The types to write for the given recorder.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WriteTypes {
    /// Only write items defined in the library for this namespace.
    Namespace,
    /// Write types in this namespace and global interoptopus types (e.g., FFIBool)
    NamespaceAndInteroptopusGlobal,
    /// Write every type in the library, regardless of namespace association.
    All,
}

/// How to handle generation of unsupported elements
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Unsupported {
    /// Emit a panic during binding generation.
    Panic,
    /// Try to finish binding generation. Unsupported items may be broken and a code comment is added.
    Comment,
}

impl WriteTypes {
    pub fn write_interoptopus_globals(&self) -> bool {
        match self {
            WriteTypes::Namespace => false,
            WriteTypes::NamespaceAndInteroptopusGlobal => true,
            WriteTypes::All => true,
        }
    }
}

/// The access modifiers for generated CSharp types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CSharpVisibility {
    /// Mimics Rust visibility.
    AsDeclared,
    /// Generates all types as `public class` / `public struct`.
    ForcePublic,
    /// Generates all types as `internal class` / `internal struct`.
    ForceInternal,
}

impl CSharpVisibility {
    pub fn to_access_modifier(&self) -> &'static str {
        match self {
            // TODO: `AsDeclared` should ultimately use the declared visibility but for now copy the previous
            // behavior which is to make everything public.
            CSharpVisibility::AsDeclared => "public",
            CSharpVisibility::ForcePublic => "public",
            CSharpVisibility::ForceInternal => "internal",
        }
    }
}

/// Whether and how `unsafe` in generated C# should be emitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Unsafe {
    /// Do not use C# `unsafe`.
    None,
    /// Use `unsafe` for performance optimizations (Unity compatible).
    UnsafeKeyword,
    /// Also use `unsafe` for slice copies.
    UnsafePlatformMemCpy,
}

impl Unsafe {
    pub fn any_unsafe(self) -> bool {
        match self {
            Unsafe::None => false,
            Unsafe::UnsafeKeyword => true,
            Unsafe::UnsafePlatformMemCpy => true,
        }
    }
}

/// The kind of types to use when generating FFI method overloads.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParamSliceType {
    /// Slices should be passed in as C# arrays.
    Array,
    /// Slices should be passed in as Span and ReadOnlySpan.
    Span,
}

/// Configures C# code generation.
#[derive(Clone, Debug)]
pub struct Config {
    /// The file header, e.g., `// (c) My Company`.
    pub file_header_comment: String,
    /// Name of static class for Interop methods, e.g., `Interop`.
    pub class: String,
    /// Name of static class for Interop constants, e.g., `Interop`. If [None] then [Self.class] is used
    pub class_constants: Option<String>,
    /// DLL to load, e.g., `my_library`.
    pub dll_name: String,
    /// Maps which namespace id belongs into which FQN (e.g., "common" => "MyCompany.Common").
    pub namespace_mappings: NamespaceMappings,
    /// Namespace ID of _this_ namespace to write (default "").
    pub namespace_id: String,
    /// Sets the visibility access modifiers for generated types.
    pub visibility_types: CSharpVisibility,
    /// Whether, say, a `x: [u8; 3]` should become 3 `x0: u8, ...` instead.
    ///
    /// If this is not set, interop generation with arrays in structs will fail. This is a somewhat
    /// open issue w.r.t Unity-sans-unsafe support and feedback would be greatly welcome!
    pub unroll_struct_arrays: bool,
    /// Which types to write.
    pub write_types: WriteTypes,
    /// If enabled bindings will use C# `unsafe` for increased performance; but will need to be enabled in C# project settings to work.
    pub use_unsafe: Unsafe,
    /// Generate functions and field names matching C# conventions, instead of mapping them 1:1 with Rust.
    pub rename_symbols: bool,
    /// Also generate markers for easier debugging
    pub debug: bool,
    /// Whether we should attempt to work around issues where a callback back to C# might not
    /// reenter Rust code when an exception happened. This requires callbacks to return
    /// an FFIError type.   
    pub work_around_exception_in_callback_no_reentry: bool,
    /// If signatures that normally use arrays should instead use span and readonly span.
    /// Requires use_unsafe, as pinning spans requires the fixed keyword.
    pub param_slice_type: ParamSliceType,
    /// How to handle unsupported constructs.
    pub unsupported: Unsupported,
    /// The string to use for reporting within FFIError. Use `{error}` to reference the inner error content.
    pub error_text: String,
}

impl Config {}

impl Default for Config {
    fn default() -> Self {
        Self {
            file_header_comment: "// Automatically generated by Interoptopus.".to_string(),
            class: "Interop".to_string(),
            class_constants: None,
            dll_name: "library".to_string(),
            namespace_mappings: NamespaceMappings::new("My.Company"),
            namespace_id: "".to_string(),
            visibility_types: CSharpVisibility::AsDeclared,
            unroll_struct_arrays: true,
            write_types: WriteTypes::NamespaceAndInteroptopusGlobal,
            use_unsafe: Unsafe::None,
            rename_symbols: false,
            debug: false,
            work_around_exception_in_callback_no_reentry: true,
            param_slice_type: ParamSliceType::Array,
            unsupported: Unsupported::Panic,
            error_text: "Something went wrong: {error}".to_string(),
        }
    }
}

/// Configures C# documentation generation.
#[derive(Clone, Debug, Default)]
pub struct DocConfig {
    /// Header to append to the generated documentation.
    pub header: String,
}
