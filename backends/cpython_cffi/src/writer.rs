use crate::config::Config;
use crate::converter::{Converter, PythonTypeConverter};
use interoptopus::lang::c::{CType, CompositeType, EnumType, Function, PrimitiveType};
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, safe_name};
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error, Interop, Library};
use interoptopus_backend_c::CTypeConverter;

/// Writes the Python file format, `impl` this trait to customize output.
pub trait PythonWriter {
    /// Returns the user config.
    fn config(&self) -> &Config;

    /// Returns the library to produce bindings for.
    fn library(&self) -> &Library;

    /// Returns the library to produce bindings for.
    fn converter(&self) -> &Converter;

    /// Returns the C-generator
    fn c_generator(&self) -> &interoptopus_backend_c::Generator;

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"from __future__ import annotations"#)?;
        w.newline()?;
        indented!(w, r#"# Print usable error message if dependency is not installed."#)?;
        indented!(w, r#"try:"#)?;
        indented!(w, [_], r#"from cffi import FFI"#)?;
        indented!(w, [_], r#"from typing import TypeVar, Generic"#)?;
        indented!(w, [_], r#"T = TypeVar("T")"#)?;
        indented!(w, r#"except ImportError:"#)?;
        indented!(w, [_], r#"print("Ensure you run Python 3.7+ and have CFFI installed (`pip install cffi`).")"#)?;
        indented!(w, [_], r#"exit(1)"#)?;
        Ok(())
    }

    fn write_api_load_fuction(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"{} = FFI()"#, self.config().ffi_attribute)?;
        indented!(w, r#"{}.cdef(api_definition)"#, self.config().ffi_attribute)?;
        indented!(w, r#"_api = None"#)?;
        w.newline()?;
        w.newline()?;

        indented!(w, r#"def {}(dll):"#, self.config().init_api_function_name)?;
        indented!(w, [_], r#""""Initializes this library, call with path to DLL.""""#)?;
        indented!(w, [_], r#"global _api"#)?;
        indented!(w, [_], r#"_api = {}.dlopen(dll)"#, self.config().ffi_attribute)?;

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

            if !docs.is_empty() {
                indented!(w, r#"{}"#, docs)?;
            }
            indented!(w, r#"{} = {}"#, constant.name(), self.converter().constant_value_to_value(constant.value()))?;
        }

        Ok(())
    }

    fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for the_type in self.library().ctypes() {
            match the_type {
                CType::Enum(e) => {
                    w.newline()?;
                    w.newline()?;
                    self.write_enum(w, e)?;
                }
                CType::Composite(c) => {
                    w.newline()?;
                    w.newline()?;
                    self.write_struct(w, c)?;
                }
                CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
                    w.newline()?;
                    w.newline()?;
                    self.write_enum(w, e.the_enum())?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn write_struct(&self, w: &mut IndentWriter, e: &CompositeType) -> Result<(), Error> {
        let cname = self.converter().c_converter().composite_to_typename(e);

        indented!(w, r#"class {}(CHeapAllocated):"#, e.rust_name())?;
        indented!(w, [_], r#"{}"#, self.converter().documentation(e.meta().documentation()))?;

        // Ctor
        let extra_args = e
            .fields()
            .iter()
            .map(|x| {
                let type_hint_in = self.converter().to_type_hint_in(x.the_type());

                format!("{}{} = None", x.name(), type_hint_in)
            })
            .collect::<Vec<_>>()
            .join(", ");
        indented!(w, [_], r#"def __init__(self, {}):"#, extra_args)?;
        // indented!(w, [_ _], r#"global _api, ffi"#)?;
        indented!(w, [_ _], r#"self._ctx = ffi.new("{}[]", 1)"#, cname)?;
        for field in e.fields().iter() {
            indented!(w, [_ _], r#"if {} is not None:"#, field.name())?;
            indented!(w, [_ _ _], r#"self.{} = {}"#, field.name(), field.name())?;
        }
        w.newline()?;

        indented!(w, [_], r#"@staticmethod"#)?;
        indented!(w, [_], r#"def c_array(n: int) -> CArray[{}]:"#, e.rust_name())?;
        indented!(w, [_ _], r#"return CArray("{}", n)"#, cname)?;

        for field in e.fields() {
            w.newline()?;

            let type_hint_in = self.converter().to_type_hint_in(field.the_type());
            let type_hint_out = self.converter().to_type_hint_out(field.the_type());

            indented!(w, [_], r#"@property"#)?;
            indented!(w, [_], r#"def {}(self){}:"#, field.name(), type_hint_out)?;
            indented!(w, [_ _], r#"{}"#, self.converter().documentation(field.documentation()))?;
            indented!(w, [_ _], r#"return self._ctx[0].{}"#, field.name())?;
            w.newline()?;

            let _docs = field.documentation().lines().join("\n");
            indented!(w, [_], r#"@{}.setter"#, field.name())?;
            indented!(w, [_], r#"def {}(self, value{}):"#, field.name(), type_hint_in)?;
            indented!(w, [_ _], r#"if hasattr(value, "_ctx"):"#)?;
            indented!(w, [_ _ _], r#"if hasattr(value, "_c_array"):"#)?;
            indented!(w, [_ _ _ _], r#"value = value.c_ptr()"#)?;
            indented!(w, [_ _ _], r#"else:"#)?;
            indented!(w, [_ _ _ _], r#"value = value.c_value()"#)?;
            // We also write _ptr to hold on to any allocated object created by CFFI. If we do not
            // then we might assign a pointer in the _ctx line below, but once the parameter (the CFFI handle)
            // leaves this function the handle might become deallocated and therefore the pointer
            // becomes invalid
            // indented!(w, [_ _], r#"self._ptr_{} = value"#, field.name())?;
            indented!(w, [_ _], r#"self._ctx[0].{} = value"#, field.name())?;
        }

        Ok(())
    }

    fn write_enum(&self, w: &mut IndentWriter, e: &EnumType) -> Result<(), Error> {
        indented!(w, r#"class {}:"#, e.rust_name())?;
        indented!(w, [_], r#"{}"#, self.converter().documentation(e.meta().documentation()))?;
        for v in e.variants() {
            indented!(w, [_], r#"{} = {}"#, v.name(), v.value())?;
        }

        Ok(())
    }

    fn write_callback_helpers(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"class {}:"#, self.config().callback_namespace)?;
        indented!(w, [_], r#""""Helpers to define `@ffi.callback`-style callbacks.""""#)?;

        for callback in self.library().ctypes().iter().filter_map(|x| match x {
            CType::FnPointer(x) => Some(x),
            CType::Pattern(TypePattern::NamedCallback(x)) => Some(x.fnpointer()),
            _ => None,
        }) {
            indented!(
                w,
                [_],
                r#"{} = "{}""#,
                safe_name(&callback.internal_name()),
                self.converter().fnpointer_to_typename(callback)
            )?;
        }

        Ok(())
    }

    fn write_function_proxies(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"class {}:"#, self.config().raw_fn_namespace)?;
        indented!(w, [_], r#""""Raw access to all exported functions.""""#)?;
        w.newline()?;

        for function in self.library().functions() {
            let type_hint_out = self.converter().to_type_hint_out(function.signature().rval());
            let args = function.signature().params().iter().map(|x| x.name().to_string()).collect::<Vec<_>>().join(", ");
            let args_with_typehints = function
                .signature()
                .params()
                .iter()
                .map(|x| {
                    let name = x.name().to_string();
                    let type_hint = self.converter().to_type_hint_in(x.the_type());
                    format!("{}{}", name, type_hint)
                })
                .collect::<Vec<_>>()
                .join(", ");

            indented!(w, [_], r#"@staticmethod"#)?;
            indented!(w, [_], r#"def {}({}){}:"#, function.name(), &args_with_typehints, type_hint_out)?;
            indented!(w, [_ _], r#"{}"#, self.converter().documentation(function.meta().documentation()))?;
            // indented!(w, [_ _], r#"global _api"#)?;

            // Determine if the function was called with a wrapper we produced which as a private `_ctx`.
            // If so, use that instead. Otherwise, just pass parameters and hope for the best.
            for param in function.signature().params() {
                match param.the_type() {
                    CType::ReadPointer(_) => {
                        indented!(w, [_ _], r#"if hasattr({}, "_ctx"):"#, param.name())?;
                        indented!(w, [_ _ _], r#"{} = {}.c_ptr()"#, param.name(), param.name())?
                    }
                    CType::ReadWritePointer(_) => {
                        indented!(w, [_ _], r#"if hasattr({}, "_ctx"):"#, param.name())?;
                        indented!(w, [_ _ _], r#"{} = {}.c_ptr()"#, param.name(), param.name())?
                    }
                    x @ CType::Pattern(TypePattern::SliceMut(_)) => {
                        let slice_type = self.converter().c_converter().to_type_specifier(x);
                        indented!(w, [_ _], r#"_{} = ffi.new("{}[]", 1)"#, param.name(), slice_type)?;
                        indented!(w, [_ _], r#"_{}[0].data = {}.c_ptr()"#, param.name(), param.name())?;
                        indented!(w, [_ _], r#"_{}[0].len = len({})"#, param.name(), param.name())?;
                        indented!(w, [_ _], r#"{} = _{}[0]"#, param.name(), param.name())?;
                    }
                    x @ CType::Pattern(TypePattern::Slice(_)) => {
                        let slice_type = self.converter().c_converter().to_type_specifier(x);
                        indented!(w, [_ _], r#"_{} = ffi.new("{}[]", 1)"#, param.name(), slice_type)?;
                        indented!(w, [_ _], r#"_{}[0].data = {}.c_ptr()"#, param.name(), param.name())?;
                        indented!(w, [_ _], r#"_{}[0].len = len({})"#, param.name(), param.name())?;
                        indented!(w, [_ _], r#"{} = _{}[0]"#, param.name(), param.name())?;
                    }
                    CType::FnPointer(fnpointer) => {
                        let params = fnpointer.signature().params().iter().map(|x| x.name()).collect::<Vec<_>>().join(", ");
                        let callback_ref = safe_name(&fnpointer.internal_name());
                        indented!(w, [_ _], r#"_{} = {}"#, param.name(), param.name())?;
                        w.newline()?;
                        indented!(w, [_ _], r#"@ffi.callback(callbacks.{})"#, callback_ref)?;
                        indented!(w, [_ _], r#"def _{}_callback({}):"#, param.name(), params)?;
                        for p in fnpointer.signature().params() {
                            match p.the_type() {
                                CType::Pattern(TypePattern::Slice(_)) | CType::Pattern(TypePattern::SliceMut(_)) => {
                                    indented!(w, [_ _ _], r#"{} = CSlice({})"#, param.name(), param.name())?;
                                }
                                _ => {}
                            }
                        }
                        indented!(w, [_ _ _], r#"return _{}({})"#, param.name(), params)?;
                        w.newline()?;
                        indented!(w, [_ _], r#"{} = _{}_callback"#, param.name(), param.name())?;
                    }
                    CType::Pattern(TypePattern::NamedCallback(callback)) => {
                        let fnpointer = callback.fnpointer();
                        let params = fnpointer.signature().params().iter().map(|x| x.name()).collect::<Vec<_>>().join(", ");
                        let callback_ref = safe_name(&fnpointer.internal_name());
                        indented!(w, [_ _], r#"_{} = {}"#, param.name(), param.name())?;
                        w.newline()?;
                        indented!(w, [_ _], r#"@ffi.callback(callbacks.{})"#, callback_ref)?;
                        indented!(w, [_ _], r#"def _{}_callback({}):"#, param.name(), params)?;
                        for p in fnpointer.signature().params() {
                            match p.the_type() {
                                CType::Pattern(TypePattern::Slice(_)) | CType::Pattern(TypePattern::SliceMut(_)) => {
                                    indented!(w, [_ _ _], r#"{} = CSlice({})"#, p.name(), p.name())?;
                                }
                                _ => {}
                            }
                        }
                        indented!(w, [_ _ _], r#"return _{}({})"#, param.name(), params)?;
                        w.newline()?;
                        indented!(w, [_ _], r#"{} = _{}_callback"#, param.name(), param.name())?;
                    }
                    CType::Pattern(TypePattern::AsciiPointer) => {
                        indented!(w, [_ _], r#"if isinstance({}, bytes):"#, param.name())?;
                        indented!(w, [_ _ _], r#"{} = ascii_string({})"#, param.name(), param.name())?;
                    }
                    _ => {
                        indented!(w, [_ _], r#"if hasattr({}, "_ctx"):"#, param.name())?;
                        indented!(w, [_ _ _], r#"{} = {}.c_value()"#, param.name(), param.name())?
                    }
                }
            }

            w.newline()?;

            match function.signature().rval() {
                CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
                    indented!(w, [_ _], r#"_rval = _api.{}({})"#,  function.name(), &args)?;
                    indented!(w, [_ _], r#"if _rval == {}.{}:"#, e.the_enum().rust_name(), e.success_variant().name())?;
                    indented!(w, [_ _ _], r#"return _rval"#)?;
                    indented!(w, [_ _], r#"else:"#)?;
                    indented!(w, [_ _ _], r#"raise Exception(f"Function returned error {{_rval}}")"#)?;
                }
                _ => {
                    indented!(w, [_ _], r#"return _api.{}({})"#, function.name(), &args)?;
                }
            }

            // indented!(w, [_ _], r#"return _api.{}({})"#, function.name(), &args)?;

            w.newline()?;
        }

        Ok(())
    }

    fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.library().patterns() {
            match pattern {
                LibraryPattern::Service(cls) => self.write_pattern_class(w, cls)?,
            }
        }

        Ok(())
    }

    fn pattern_class_args_without_first_to_string(&self, function: &Function, type_hints: bool) -> String {
        function
            .signature()
            .params()
            .iter()
            .skip(1)
            .map(|x| {
                let type_hint = if type_hints {
                    self.converter().to_type_hint_in(x.the_type())
                } else {
                    "".to_string()
                };
                format!("{}{}", x.name().to_string(), type_hint)
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn write_pattern_class_success_enum_aware_rval(&self, w: &mut IndentWriter, _class: &Service, function: &Function, deref_ctx: bool) -> Result<(), Error> {
        let args = self.pattern_class_args_without_first_to_string(function, false);
        let ctx = if deref_ctx { "self.c_value()" } else { "self.c_ptr()" };

        if deref_ctx {
            indented!(w, [_ _], r#"return {}.{}({}, {})"#, self.config().raw_fn_namespace, function.name(), ctx, &args)?;
        } else {
            indented!(w, [_ _], r#"{}.{}({}, {})"#, self.config().raw_fn_namespace, function.name(), ctx, &args)?;
        }
        Ok(())
    }

    fn write_pattern_class(&self, w: &mut IndentWriter, class: &Service) -> Result<(), Error> {
        let context_type_name = class.the_type().rust_name();
        let context_cname = self.converter().c_converter().opaque_to_typename(class.the_type());

        let mut all_functions = class.constructors().to_vec();
        all_functions.extend_from_slice(class.methods());
        all_functions.push(class.destructor().clone());

        let common_prefix = longest_common_prefix(&all_functions);

        indented!(w, r#"class {}(CHeapAllocated):"#, context_type_name)?;
        indented!(w, [_], r#"__api_lock = object()"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __init__(self, api_lock):"#)?;
        indented!(w, [_ _], r#"assert(api_lock == {}.__api_lock), "You must create this with a static constructor." "#, context_type_name)?;
        indented!(w, [_ _], r#"self._ctx = ffi.new("{}**")"#, context_cname)?;
        w.newline()?;

        for ctor in class.constructors() {
            let ctor_args = self.pattern_class_args_without_first_to_string(ctor, true);
            indented!(w, [_], r#"@staticmethod"#)?;
            indented!(w, [_], r#"def {}({}) -> {}:"#, ctor.name().replace(&common_prefix, ""), ctor_args, context_type_name)?;
            indented!(w, [_ _], r#"{}"#, self.converter().documentation(ctor.meta().documentation()))?;
            indented!(w, [_ _], r#"self = {}({}.__api_lock)"#, context_type_name, context_type_name)?;
            for param in ctor.signature().params().iter().skip(1) {
                indented!(w, [_ _ ], r#"if hasattr({}, "_ctx"):"#, param.name())?;
                indented!(w, [_ _ _], r#"{} = {}.c_ptr()"#, param.name(), param.name())?;
            }
            self.write_pattern_class_success_enum_aware_rval(w, class, ctor, false)?;
            indented!(w, [_ _], r#"return self"#)?;
            w.newline()?;
        }

        // Dtor
        indented!(w, [_], r#"def __del__(self):"#)?;
        // indented!(w, [_ _], r#"global _api, ffi"#)?;
        self.write_pattern_class_success_enum_aware_rval(w, class, class.destructor(), false)?;

        for function in class.methods() {
            w.newline()?;

            let args = self.pattern_class_args_without_first_to_string(function, true);
            let type_hint_out = self.converter().to_type_hint_out(function.signature().rval());

            indented!(w, [_], r#"def {}(self, {}){}:"#, function.name().replace(&common_prefix, ""), &args, type_hint_out)?;
            indented!(w, [_ _], r#"{}"#, self.converter().documentation(function.meta().documentation()))?;
            // indented!(w, [_ _], r#"global {}"#, self.config().raw_fn_namespace)?;

            // // Determine if the function was called with a wrapper we produced which as a private `_ctx`.
            // // If so, use that instead. Otherwise, just pass parameters and hope for the best.
            // for param in function.signature().params().iter().skip(1) {
            //     indented!(w, [_ _], r#"if hasattr({}, "_ctx"):"#, param.name())?;
            //     indented!(w, [_ _ _], r#"{} = {}._ctx[0]"#, param.name(), param.name())?;
            // }

            self.write_pattern_class_success_enum_aware_rval(w, class, function, true)?;
        }

        Ok(())
    }

    fn write_c_header(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"# Raw API definition for CFFI."#)?;
        indented!(w, r#"api_definition = """"#)?;

        let mut c_header: Vec<u8> = Vec::new();
        let mut ident_writer = IndentWriter::new(&mut c_header);

        self.c_generator().write_to(&mut ident_writer)?;

        let as_string = String::from_utf8(c_header)?;

        writeln!(w.writer(), "{}", as_string.trim())?;

        indented!(w, r#"""""#)?;
        Ok(())
    }

    fn write_utils(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"class CHeapAllocated(Generic[T]):"#)?;
        indented!(w, [_], r#""""Base class from which all struct type wrappers are derived.""""#)?;
        indented!(w, [_], r#"def __init__(self):"#)?;
        indented!(w, [_ _], r#"pass"#)?;
        w.newline()?;
        indented!(w, [_], r#"def c_ptr(self):"#)?;
        indented!(w, [_ _], r#""""Returns a C-level pointer to the native data structure.""""#)?;
        indented!(w, [_ _], r#"return self._ctx"#)?;
        w.newline()?;
        indented!(w, [_], r#"def c_value(self) -> T:"#)?;
        indented!(w, [_ _], r#""""From the underlying pointer returns the (first) entry as a value.""""#)?;
        indented!(w, [_ _], r#"return self._ctx[0]"#)?;
        w.newline()?;
        w.newline()?;

        self.write_utils_primitive(w, PrimitiveType::I8)?;
        self.write_utils_primitive(w, PrimitiveType::I16)?;
        self.write_utils_primitive(w, PrimitiveType::I32)?;
        self.write_utils_primitive(w, PrimitiveType::I64)?;
        self.write_utils_primitive(w, PrimitiveType::U8)?;
        self.write_utils_primitive(w, PrimitiveType::U16)?;
        self.write_utils_primitive(w, PrimitiveType::U32)?;
        self.write_utils_primitive(w, PrimitiveType::U64)?;

        indented!(w, r#"class CArray(CHeapAllocated, Generic[T]):"#)?;
        indented!(w, [_], r#""""Holds a native C array with a given length.""""#)?;
        indented!(w, [_], r#"def __init__(self, type, n):"#)?;
        indented!(w, [_ _], r#"self._ctx = ffi.new(f"{{type}}[{{n}}]")"#)?;
        indented!(w, [_ _], r#"self._c_array = True"#)?;
        indented!(w, [_ _], r#"self._len = n"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __getitem__(self, key) -> T:"#)?;
        indented!(w, [_ _], r#"return self._ctx[key]"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __setitem__(self, key, value: T):"#)?;
        indented!(w, [_ _], r#"self._ctx[key] = value"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __len__(self):"#)?;
        indented!(w, [_ _], r#"return self._len"#)?;
        w.newline()?;
        w.newline()?;

        indented!(w, r#"class CSlice(CHeapAllocated, Generic[T]):"#)?;
        indented!(w, [_], r#""""Holds a native C array with a given length.""""#)?;
        indented!(w, [_], r#"def __init__(self, c_slice):"#)?;
        indented!(w, [_ _], r#"self._ctx = c_slice"#)?;
        indented!(w, [_ _], r#"self._c_slice = True"#)?;
        indented!(w, [_ _], r#"self._len = c_slice.len"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __getitem__(self, key) -> T:"#)?;
        indented!(w, [_ _], r#"return self._ctx.data[key]"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __setitem__(self, key, value: T):"#)?;
        indented!(w, [_ _], r#"self._ctx.data[key] = value"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __len__(self):"#)?;
        indented!(w, [_ _], r#"return self._ctx.len"#)?;
        w.newline()?;
        w.newline()?;

        indented!(w, r#"def ascii_string(x: bytes):"#)?;
        indented!(w, [_], r#""""Must be called with a b"my_string".""""#)?;
        // indented!(w, [_], r#"global ffi"#)?;
        indented!(w, [_], r#"return ffi.new("char[]", x)"#)?;
        Ok(())
    }

    fn write_utils_primitive(&self, w: &mut IndentWriter, primitive: PrimitiveType) -> Result<(), Error> {
        let the_type = self.converter().c_converter.to_type_specifier(&CType::Primitive(primitive));
        let type_hint_in = self.converter().to_type_hint_in(&CType::Primitive(primitive));
        let type_hint = self.converter().to_type_hint(&CType::Primitive(primitive));

        indented!(w, r#"class {}(CHeapAllocated[T]):"#, the_type)?;
        indented!(w, [_], r#""""One or more heap allocated primitive `{}` values.""""#, the_type)?;
        indented!(w, [_], r#"def __init__(self, x{} = None):"#, type_hint_in)?;
        indented!(w, [_ _], r#"self._ctx = ffi.new(f"{}[1]", [0])"#, the_type)?;
        indented!(w, [_ _], r#"if x is not None:"#)?;
        indented!(w, [_ _ _], r#"self._ctx[0] = x"#)?;
        w.newline()?;
        indented!(w, [_], r#"@staticmethod"#)?;
        indented!(w, [_], r#"def c_array(n:int = None) -> CArray[{}]:"#, type_hint)?;
        indented!(w, [_ _], r#"return CArray("{}", n)"#, the_type)?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_all(&self, w: &mut IndentWriter) -> Result<(), Error> {
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

        self.write_utils(w)?;
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

        Ok(())
    }
}
