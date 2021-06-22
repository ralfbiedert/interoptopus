//! Generates CPython CFFI bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! ## Usage
//!
//! In your library or a support project add this:
//!
//! ```
//! # mod my_crate { use interoptopus::{Library}; pub fn ffi_inventory() -> Library { todo!() } }
//! use my_crate::ffi_inventory;
//!
//! #[test]
//! fn generate_python_bindings() {
//!     use interoptopus::Interop;
//!     use interoptopus_backend_cpython_cffi::{Generator, InteropCPythonCFFI, Config};
//!
//!     // Converts an `ffi_inventory()` into Python interop definitions.
//!     Generator::new(Config::default(), ffi_inventory()).write_to("module.py")
//! }
//! ```
//!
//! And we might produce something like this:
//!
//! ```python
//! from cffi import FFI
//!
//! api_definition = """
//! typedef struct Vec3f32
//!     {
//!     float x;
//!     float y;
//!     float z;
//!     } Vec2f32;
//!
//! Vec3f32 my_game_function(Vec3f32* input);
//! """
//!
//!
//! ffi = FFI()
//! ffi.cdef(api_definition)
//! _api = None
//!
//!
//! def init_api(dll):
//!     """Initializes this library, call with path to DLL."""
//!     global _api
//!     _api = ffi.dlopen(dll)
//!
//!
//! class raw:
//!     """Raw access to all exported functions."""
//!
//!     def my_game_function(input):
//!     global _api
//!     return _api.my_game_function(input)
//! ```

use interoptopus::lang::c::{CType, CompositeType, ConstantValue, Documentation, EnumType, FnPointerType, Function, PrimitiveValue};
use interoptopus::patterns::class::Class;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, safe_name};
use interoptopus::writer::IndentWriter;
use interoptopus::Interop;
use interoptopus::{Error, Library};
use interoptopus_backend_c::InteropC;

/// Configures Python code generation.
#[derive(Clone, Debug)]
pub struct Config {
    /// How to name the function responsible for loading the DLL, e.g., `init_api`.
    pub init_api_function_name: String,
    /// Attribute by which the `cffi` object is exposed, e.g., `ffi`.
    pub ffi_attribute: String,
    /// Namespace to put functions into, e.g., `raw`.
    pub raw_fn_namespace: String,
    /// Namespace for callback helpers, e.g., `callbacks`.
    pub callback_namespace: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            init_api_function_name: "init_api".to_string(),
            ffi_attribute: "ffi".to_string(),
            raw_fn_namespace: "raw".to_string(),
            callback_namespace: "callbacks".to_string(),
        }
    }
}

/// Helper type implementing [`InteropCPythonCFFI`] and [`Interop`].
pub struct Generator {
    c_generator: interoptopus_backend_c::Generator,
    config: Config,
    library: Library,
}

impl Generator {
    pub fn new(config: Config, library: Library) -> Self {
        let c_generator = interoptopus_backend_c::Generator::new(
            interoptopus_backend_c::Config {
                directives: false,
                imports: false,
                file_header_comment: "".to_string(),
                ..interoptopus_backend_c::Config::default()
            },
            library.clone(),
        );

        Self { c_generator, config, library }
    }
}

/// Contains all Python generators, create sub-trait to customize.
pub trait InteropCPythonCFFI: Interop {
    /// Returns the user config.
    fn config(&self) -> &Config;

    /// Returns the library to produce bindings for.
    fn library(&self) -> &Library;

    /// Returns the C-generator
    fn c_generator(&self) -> &interoptopus_backend_c::Generator;

    fn constant_value_to_value(&self, value: &ConstantValue) -> String {
        match value {
            ConstantValue::Primitive(x) => match x {
                PrimitiveValue::Bool(x) => format!("{}", x),
                PrimitiveValue::U8(x) => format!("{}", x),
                PrimitiveValue::U16(x) => format!("{}", x),
                PrimitiveValue::U32(x) => format!("{}", x),
                PrimitiveValue::U64(x) => format!("{}", x),
                PrimitiveValue::I8(x) => format!("{}", x),
                PrimitiveValue::I16(x) => format!("{}", x),
                PrimitiveValue::I32(x) => format!("{}", x),
                PrimitiveValue::I64(x) => format!("{}", x),
                PrimitiveValue::F32(x) => format!("{}", x),
                PrimitiveValue::F64(x) => format!("{}", x),
            },
        }
    }

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"from cffi import FFI"#))?;
        Ok(())
    }

    fn write_api_load_fuction(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"{} = FFI()"#, self.config().ffi_attribute))?;
        w.indented(|w| writeln!(w, r#"{}.cdef(api_definition)"#, self.config().ffi_attribute))?;
        w.indented(|w| writeln!(w, r#"_api = None"#))?;
        w.newline()?;
        w.newline()?;

        w.indented(|w| writeln!(w, r#"def {}(dll):"#, self.config().init_api_function_name))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""Initializes this library, call with path to DLL.""""#))?;
        w.indented(|w| writeln!(w, r#"global _api"#))?;
        w.indented(|w| writeln!(w, r#"_api = {}.dlopen(dll)"#, self.config().ffi_attribute))?;
        w.unindent();
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for constant in self.library().constants() {
            let docs = constant
                .meta()
                .documentation()
                .lines()
                .iter()
                .map(|x| format!("# {}", x))
                .collect::<Vec<_>>()
                .join("\n");
            w.indented(|w| writeln!(w, r#"{}"#, docs))?;
            w.indented(|w| writeln!(w, r#"{} = {}"#, constant.name(), self.constant_value_to_value(constant.value())))?;
        }

        Ok(())
    }

    fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for the_type in self.library().ctypes() {
            match the_type {
                CType::Enum(e) => self.write_enum(w, e)?,
                CType::Composite(c) => self.write_struct(w, c)?,
                CType::Pattern(TypePattern::SuccessEnum(e)) => self.write_enum(w, e.the_enum())?,
                _ => {}
            }
        }

        Ok(())
    }

    fn write_struct(&self, w: &mut IndentWriter, e: &CompositeType) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"class {}(object):"#, e.rust_name()))?;
        w.indent();
        self.write_documentation(w, e.meta().documentation())?;

        // Ctor
        w.indented(|w| writeln!(w, r#"def __init__(self):"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"global _api, ffi"#))?;
        w.indented(|w| writeln!(w, r#"self._ctx = ffi.new("{}[]", 1)[0]"#, e.rust_name()))?;
        w.unindent();
        w.newline()?;

        // Array constructor
        w.indented(|w| writeln!(w, r#"def array(n):"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"global _api, ffi"#))?;
        w.indented(|w| writeln!(w, r#"return ffi.new("{}[]", n)"#, e.rust_name()))?;
        w.unindent();
        w.newline()?;

        for field in e.fields() {
            w.indented(|w| writeln!(w, r#"@property"#))?;
            w.indented(|w| writeln!(w, r#"def {}(self):"#, field.name()))?;
            w.indent();
            self.write_documentation(w, field.documentation())?;
            w.indented(|w| writeln!(w, r#"return self._ctx.{}"#, field.name()))?;
            w.unindent();
            w.newline()?;

            let _docs = field.documentation().lines().join("\n");
            w.indented(|w| writeln!(w, r#"@{}.setter"#, field.name()))?;
            w.indented(|w| writeln!(w, r#"def {}(self, value):"#, field.name()))?;
            w.indent();
            // We also write _ptr to hold on to any allocated object created by CFFI. If we do not
            // then we might assign a pointer in the _ctx line below, but once the parameter (the CFFI handle)
            // leaves this function the handle might become deallocated and therefore the pointer
            // becomes invalid
            w.indented(|w| writeln!(w, r#"self._ptr_{} = value"#, field.name()))?;
            w.indented(|w| writeln!(w, r#"self._ctx.{} = value"#, field.name()))?;
            w.unindent();
            w.newline()?;
        }

        // General Getter (gives weird errors
        // w.indented(|w| writeln!(w, r#"def __get__(self, obj, objtype):"#))?;
        // w.indent();
        // w.indented(|w| writeln!(w, r#""""{}""""#, docs))?;
        // w.indented(|w| writeln!(w, r#"return getattr(obj, self.ctx)"#))?;
        // w.unindent();
        // w.newline()?;

        w.unindent();

        Ok(())
    }

    fn write_enum(&self, w: &mut IndentWriter, e: &EnumType) -> Result<(), Error> {
        let docs = e.meta().documentation().lines().join("\n");

        w.indented(|w| writeln!(w, r#"class {}:"#, e.rust_name()))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""{}""""#, docs))?;
        for v in e.variants() {
            w.indented(|w| writeln!(w, r#"{} = {}"#, v.name(), v.value()))?;
        }
        w.unindent();
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_callback_helpers(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"class {}:"#, self.config().callback_namespace))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""Helpers to define `@ffi.callback`-style callbacks.""""#))?;

        for callback in self.library().ctypes().iter().filter_map(|x| match x {
            CType::FnPointer(x) => Some(x),
            _ => None,
        }) {
            w.indented(|w| writeln!(w, r#"{} = "{}""#, safe_name(&callback.internal_name()), self.type_fnpointer_to_typename(callback)))?;
        }

        w.unindent();
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn type_fnpointer_to_typename(&self, fn_pointer: &FnPointerType) -> String {
        let rval = self.c_generator().type_to_type_specifier(&fn_pointer.signature().rval());
        let params = fn_pointer
            .signature()
            .params()
            .iter()
            .map(|x| self.c_generator().type_to_type_specifier(x.the_type()))
            .collect::<Vec<_>>()
            .join(",");

        format!("{}({})", rval, params)
    }

    fn write_function_proxies(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"class {}:"#, self.config().raw_fn_namespace))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""Raw access to all exported functions.""""#))?;

        for function in self.library().functions() {
            let args = function.signature().params().iter().map(|x| x.name().to_string()).collect::<Vec<_>>().join(", ");

            w.indented(|w| writeln!(w, r#"def {}({}):"#, function.name(), &args))?;
            w.indent();
            self.write_documentation(w, function.meta().documentation())?;
            w.indented(|w| writeln!(w, r#"global _api"#))?;

            // Determine if the function was called with a wrapper we produced which as a private `_ctx`.
            // If so, use that instead. Otherwise, just pass parameters and hope for the best.
            for param in function.signature().params() {
                w.indented(|w| writeln!(w, r#"if hasattr({}, "_ctx"):"#, param.name()))?;
                w.indent();
                w.indented(|w| writeln!(w, r#"{p} = {p}._ctx"#, p = param.name()))?;
                w.unindent()
            }

            w.indented(|w| writeln!(w, r#"return _api.{}({})"#, function.name(), &args))?;
            w.unindent();

            w.newline()?;
        }

        w.unindent();
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.library().patterns() {
            match pattern {
                LibraryPattern::Class(cls) => self.write_pattern_class(w, cls)?,
            }
        }

        Ok(())
    }

    fn pattern_class_args_without_first_to_string(&self, function: &Function) -> String {
        function
            .signature()
            .params()
            .iter()
            .skip(1)
            .map(|x| x.name().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn write_documentation(&self, w: &mut IndentWriter, documentation: &Documentation) -> Result<(), Error> {
        let docs: String = documentation.lines().join("\n");

        if !docs.is_empty() {
            w.indented(|w| writeln!(w, r#""""{}""""#, docs))?;
        }

        Ok(())
    }

    fn write_pattern_class_success_enum_aware_rval(&self, w: &mut IndentWriter, _class: &Class, function: &Function, deref_ctx: bool) -> Result<(), Error> {
        let args = self.pattern_class_args_without_first_to_string(function);

        let ctx = if deref_ctx { "self.ctx[0]" } else { "self.ctx" };

        match function.signature().rval() {
            CType::Pattern(TypePattern::SuccessEnum(e)) => {
                w.indented(|w| writeln!(w, r#"rval = {}.{}({}, {})"#, self.config().raw_fn_namespace, function.name(), ctx, &args))?;
                w.indented(|w| writeln!(w, r#"if rval == {}.{}:"#, e.the_enum().rust_name(), e.success_variant().name()))?;
                w.indent();
                w.indented(|w| writeln!(w, r#"return None"#))?;
                w.unindent();
                w.indented(|w| writeln!(w, r#"else:"#))?;
                w.indent();
                w.indented(|w| writeln!(w, r#"raise Exception(f"return value ${{rval}}")"#))?;
                w.unindent();
            }
            _ => {
                w.indented(|w| writeln!(w, r#"return _api.{}(self.ctx[0], {})"#, function.name(), &args))?;
            }
        }

        Ok(())
    }

    fn write_pattern_class(&self, w: &mut IndentWriter, class: &Class) -> Result<(), Error> {
        let context_type_name = class.the_type().rust_name();

        let mut all_functions = vec![class.constructor().clone(), class.destructor().clone()];
        all_functions.extend_from_slice(class.methods());
        let common_prefix = longest_common_prefix(&all_functions);

        w.indented(|w| writeln!(w, r#"class {}(object):"#, context_type_name))?;
        w.indent();

        // Ctor
        let args = self.pattern_class_args_without_first_to_string(class.constructor());
        w.indented(|w| writeln!(w, r#"def __init__(self, {}):"#, args))?;
        w.indent();
        self.write_documentation(w, class.constructor().meta().documentation())?;
        for param in class.constructor().signature().params().iter().skip(1) {
            w.indented(|w| writeln!(w, r#"if hasattr({}, "_ctx"):"#, param.name()))?;
            w.indent();
            w.indented(|w| writeln!(w, r#"{p} = {p}._ctx"#, p = param.name()))?;
            w.unindent()
        }
        w.indented(|w| writeln!(w, r#"global _api, ffi"#))?;
        w.indented(|w| writeln!(w, r#"self.ctx = ffi.new("{}**")"#, context_type_name))?;
        self.write_pattern_class_success_enum_aware_rval(w, class, class.constructor(), false)?;
        w.unindent();
        w.newline()?;

        // Dtor
        w.indented(|w| writeln!(w, r#"def __del__(self):"#))?;
        w.indent();
        self.write_documentation(w, class.destructor().meta().documentation())?;
        w.indented(|w| writeln!(w, r#"global _api, ffi"#))?;
        self.write_pattern_class_success_enum_aware_rval(w, class, class.destructor(), false)?;
        w.unindent();
        w.newline()?;

        for function in class.methods() {
            let args = self.pattern_class_args_without_first_to_string(function);

            w.indented(|w| writeln!(w, r#"def {}(self, {}):"#, function.name().replace(&common_prefix, ""), &args))?;
            w.indent();
            self.write_documentation(w, function.meta().documentation())?;
            w.indented(|w| writeln!(w, r#"global {}"#, self.config().raw_fn_namespace))?;

            self.write_pattern_class_success_enum_aware_rval(w, class, function, true)?;

            w.unindent();
            w.newline()?;
        }

        w.unindent();
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_c_header(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"api_definition = """"#))?;
        self.c_generator().write_to(w)?;
        w.indented(|w| writeln!(w, r#"""""#))?;
        Ok(())
    }

    fn write_utils(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"def ascii_string(x):"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""Must be called with a b"my_string".""""#))?;
        w.indented(|w| writeln!(w, r#"global ffi"#))?;
        w.indented(|w| writeln!(w, r#"return ffi.new("char[]", x)"#))?;
        w.unindent();
        w.newline()?;
        Ok(())
    }
}

impl Interop for Generator {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_imports(w)?;
        w.newline()?;

        self.write_c_header(w)?;
        w.newline()?;
        w.newline()?;

        self.write_api_load_fuction(w)?;
        w.newline()?;
        w.newline()?;

        self.write_constants(w)?;
        w.newline()?;
        w.newline()?;

        self.write_types(w)?;
        w.newline()?;
        w.newline()?;

        self.write_callback_helpers(w)?;
        w.newline()?;
        w.newline()?;

        self.write_function_proxies(w)?;
        w.newline()?;
        w.newline()?;

        self.write_patterns(w)?;
        w.newline()?;
        w.newline()?;

        self.write_utils(w)?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }
}

impl InteropCPythonCFFI for Generator {
    fn config(&self) -> &Config {
        &self.config
    }

    fn library(&self) -> &Library {
        &self.library
    }

    fn c_generator(&self) -> &interoptopus_backend_c::Generator {
        &self.c_generator
    }
}
