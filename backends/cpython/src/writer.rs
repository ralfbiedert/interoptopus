use crate::config::Config;
use crate::converter::Converter;
use interoptopus::lang::c::{CType, CompositeType, EnumType, Function, PrimitiveType};
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, safe_name, sort_types_by_dependencies};
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, non_service_functions, Error, Library};

/// Writes the Python file format, `impl` this trait to customize output.
pub trait PythonWriter {
    /// Returns the user config.
    fn config(&self) -> &Config;

    /// Returns the library to produce bindings for.
    fn library(&self) -> &Library;

    /// Returns the library to produce bindings for.
    fn converter(&self) -> &Converter;

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"from __future__ import annotations"#)?;
        indented!(w, r#"import ctypes"#)?;
        indented!(w, r#"import typing"#)?;
        w.newline()?;
        indented!(w, r#"T = typing.TypeVar("T")"#)?;
        Ok(())
    }

    fn write_api_load_fuction(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"c_lib = None"#)?;
        w.newline()?;
        indented!(w, r#"def init_lib(path):"#)?;
        indented!(w, [_], r#""""Initializes the native library. Must be called at least once before anything else.""""#)?;
        indented!(w, [_], r#"global c_lib"#)?;
        indented!(w, [_], r#"c_lib = ctypes.cdll.LoadLibrary(path)"#)?;

        w.newline()?;
        for f in self.library().functions() {
            let args = f
                .signature()
                .params()
                .iter()
                .map(|x| self.converter().to_ctypes_name(x.the_type(), false))
                .collect::<Vec<_>>();

            indented!(w, [_], r#"c_lib.{}.argtypes = [{}]"#, f.name(), args.join(", "))?;
        }

        w.newline()?;
        for f in self.library().functions() {
            let rtype = self.converter().to_ctypes_name(f.signature().rval(), false);
            if !rtype.is_empty() {
                indented!(w, [_], r#"c_lib.{}.restype = {}"#, f.name(), rtype)?;
            }
        }

        w.newline()?;
        for f in self.library().functions() {
            if let CType::Pattern(TypePattern::FFIErrorEnum(e)) = f.signature().rval() {
                let value = e.success_variant().value();
                indented!(w, [_], r#"c_lib.{}.errcheck = lambda rval, _fptr, _args: _errcheck(rval, {})"#, f.name(), value)?;
            }
        }

        Ok(())
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for c in self.library().constants() {
            indented!(w, r#"{} = {}"#, c.name(), self.converter().constant_value_to_value(c.value()))?;
        }

        Ok(())
    }

    fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        let all_types = self.library().ctypes().to_vec();
        let sorted_types = sort_types_by_dependencies(all_types);

        for t in &sorted_types {
            match t {
                CType::Composite(c) => self.write_struct(w, c)?,
                CType::Enum(e) => self.write_enum(w, e)?,
                CType::Pattern(p) => match p {
                    TypePattern::FFIErrorEnum(e) => self.write_enum(w, e.the_enum())?,
                    TypePattern::Slice(c) => self.write_slice(w, c, false)?,
                    TypePattern::SliceMut(c) => self.write_slice(w, c, true)?,
                    TypePattern::Option(c) => self.write_struct(w, c)?,
                    _ => continue,
                },
                _ => continue,
            }

            w.newline()?;
            w.newline()?;
        }

        Ok(())
    }

    fn write_struct(&self, w: &mut IndentWriter, c: &CompositeType) -> Result<(), Error> {
        let documentation = c.meta().documentation().lines().join("\n");

        indented!(w, r#"class {}(ctypes.Structure):"#, c.rust_name())?;
        if !documentation.is_empty() {
            indented!(w, [_], r#""""{}""""#, documentation)?;
        }

        w.newline()?;
        indented!(w, [_], r#"# These fields represent the underlying C data layout"#)?;
        indented!(w, [_], r#"_fields_ = ["#)?;
        for f in c.fields() {
            let type_name = self.converter().to_ctypes_name(f.the_type(), true);
            indented!(w, [_ _], r#"("{}", {}),"#, f.name(), type_name)?;
        }
        indented!(w, [_], r#"]"#)?;

        // Ctor
        let extra_args = c
            .fields()
            .iter()
            .map(|x| {
                let type_hint_in = self.converter().to_type_hint_in(x.the_type());

                format!("{}{} = None", x.name(), type_hint_in)
            })
            .collect::<Vec<_>>()
            .join(", ");

        if !c.fields().is_empty() {
            w.newline()?;
            indented!(w, [_], r#"def __init__(self, {}):"#, extra_args)?;
            for field in c.fields().iter() {
                indented!(w, [_ _], r#"if {} is not None:"#, field.name())?;
                indented!(w, [_ _ _], r#"self.{} = {}"#, field.name(), field.name())?;
            }
        }

        // Fields
        for f in c.fields() {
            let documentation = f.documentation().lines().join("\n");

            w.newline()?;

            let hint_in = self.converter().to_type_hint_in(f.the_type());
            let hint_out = self.converter().to_type_hint_out(f.the_type());

            indented!(w, [_], r#"@property"#)?;
            indented!(w, [_], r#"def {}(self){}:"#, f.name(), hint_out)?;
            if !documentation.is_empty() {
                indented!(w, [_ _], r#""""{}""""#, documentation)?;
            }

            match f.the_type() {
                CType::Pattern(p) => match p {
                    // This does not seem to do anything to callback-returned values
                    // TypePattern::AsciiPointer => indented!(w, [_ _], r#"return ctypes.string_at(ctypes.Structure.__get__(self, "{}"))"#, f.name())?,
                    _ => indented!(w, [_ _], r#"return ctypes.Structure.__get__(self, "{}")"#, f.name())?,
                },
                _ => indented!(w, [_ _], r#"return ctypes.Structure.__get__(self, "{}")"#, f.name())?,
            }

            w.newline()?;

            indented!(w, [_], r#"@{}.setter"#, f.name())?;
            indented!(w, [_], r#"def {}(self, value{}):"#, f.name(), hint_in)?;
            if !documentation.is_empty() {
                indented!(w, [_ _], r#""""{}""""#, documentation)?;
            }
            indented!(w, [_ _], r#"return ctypes.Structure.__set__(self, "{}", value)"#, f.name())?;
        }

        Ok(())
    }

    fn write_enum(&self, w: &mut IndentWriter, e: &EnumType) -> Result<(), Error> {
        indented!(w, r#"class {}:"#, e.rust_name())?;

        for v in e.variants() {
            indented!(w, [_], r#"{} = {}"#, v.name(), v.value())?;
        }

        Ok(())
    }

    fn write_callback_helpers(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"class {}:"#, self.config().callback_namespace)?;
        indented!(w, [_], r#""""Helpers to define callbacks.""""#)?;

        for callback in self.library().ctypes().iter().filter_map(|x| match x {
            CType::FnPointer(x) => Some(x),
            CType::Pattern(TypePattern::NamedCallback(x)) => Some(x.fnpointer()),
            _ => None,
        }) {
            indented!(
                w,
                [_],
                r#"{} = {}"#,
                safe_name(&callback.internal_name()),
                self.converter().fnpointer_to_typename(callback)
            )?;
        }

        Ok(())
    }

    fn write_function_proxies(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for function in non_service_functions(self.library()) {
            let rval_sig = self.converter().to_type_hint_out(function.signature().rval());
            let args = self.function_args_to_string(function, true, false);
            let documentation = function.meta().documentation().lines().join("\n");

            indented!(w, r#"def {}({}){}:"#, function.name(), args, rval_sig)?;

            if !documentation.is_empty() {
                indented!(w, [_], r#""""{}""""#, documentation)?;
            }

            self.write_param_helpers(w, function)?;
            self.write_library_call(w, function, None)?;

            w.newline()?;
        }

        Ok(())
    }

    fn write_param_helpers(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        for arg in function.signature().params() {
            match arg.the_type() {
                CType::FnPointer(x) => {
                    indented!(w, [_], r#"if not hasattr({}, "__ctypes_from_outparam__"):"#, arg.name())?;
                    indented!(w, [_ _], r#"{} = callbacks.{}({})"#, arg.name(), safe_name(&x.internal_name()), arg.name())?;
                    w.newline()?;
                }
                CType::Pattern(pattern) => match pattern {
                    TypePattern::NamedCallback(x) => {
                        let x = x.fnpointer();
                        indented!(w, [_], r#"if not hasattr({}, "__ctypes_from_outparam__"):"#, arg.name())?;
                        indented!(w, [_ _], r#"{} = callbacks.{}({})"#, arg.name(), safe_name(&x.internal_name()), arg.name())?;
                        w.newline()?;
                    }
                    TypePattern::AsciiPointer => {
                        indented!(w, [_], r#"if not hasattr({}, "__ctypes_from_outparam__"):"#, arg.name())?;
                        indented!(w, [_ _], r#"{} = ctypes.cast({}, ctypes.POINTER(ctypes.c_uint8))"#, arg.name(), arg.name())?;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn write_slice(&self, w: &mut IndentWriter, c: &CompositeType, mutable: bool) -> Result<(), Error> {
        let data_type = c
            .fields()
            .iter()
            .find(|x| x.name().contains("data"))
            .expect("Slice must contain field called 'data'.")
            .the_type()
            .deref_pointer()
            .expect("data must be a pointer type");

        let data_type_python = self.converter().to_ctypes_name(data_type, true);
        let hint_in = self.converter().to_type_hint_in(data_type);
        let hint_out = self.converter().to_type_hint_out(data_type);

        indented!(w, r#"class {}(ctypes.Structure):"#, c.rust_name())?;
        indented!(w, [_], r#"# These fields represent the underlying C data layout"#)?;
        indented!(w, [_], r#"_fields_ = ["#)?;
        indented!(w, [_], r#"    ("data", ctypes.POINTER({})),"#, data_type_python)?;
        indented!(w, [_], r#"    ("len", ctypes.c_uint64),"#)?;
        indented!(w, [_], r#"]"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __len__(self):"#)?;
        indented!(w, [_ _], r#"return self.len"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __getitem__(self, i){}:"#, hint_out)?;
        indented!(w, [_ _], r#"return self.data[i]"#)?;

        if mutable {
            w.newline()?;
            indented!(w, [_], r#"def __setitem__(self, i, v{}):"#, hint_in)?;
            indented!(w, [_ _], r#"self.data[i] = v"#)?;
        }

        Ok(())
    }

    fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.library().patterns() {
            match pattern {
                LibraryPattern::Service(x) => self.write_pattern_class(w, x)?,
            }
        }

        Ok(())
    }

    fn write_pattern_class(&self, w: &mut IndentWriter, class: &Service) -> Result<(), Error> {
        let context_type_name = class.the_type().rust_name();

        let mut all_functions = class.constructors().to_vec();
        all_functions.extend_from_slice(class.methods());
        all_functions.push(class.destructor().clone());

        let common_prefix = longest_common_prefix(&all_functions);

        indented!(w, r#"class {}:"#, context_type_name)?;
        indented!(w, [_], r#"__api_lock = object()"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __init__(self, api_lock, ctx):"#)?;
        indented!(w, [_ _], r#"assert(api_lock == {}.__api_lock), "You must create this with a static constructor." "#, context_type_name)?;
        indented!(w, [_ _], r#"self._ctx = ctx"#)?;
        w.newline()?;
        indented!(w, [_], r#"@property"#)?;
        indented!(w, [_], r#"def _as_parameter_(self):"#)?;
        indented!(w, [_ _], r#"return self._ctx"#)?;
        w.newline()?;

        for ctor in class.constructors() {
            let ctor_args = self.function_args_to_string(ctor, true, true);
            indented!(w, [_], r#"@staticmethod"#)?;
            indented!(w, [_], r#"def {}({}) -> {}:"#, ctor.name().replace(&common_prefix, ""), ctor_args, context_type_name)?;
            indented!(w, [_ _], r#"{}"#, self.converter().documentation(ctor.meta().documentation()))?;
            indented!(w, [_ _], r#"ctx = ctypes.c_void_p()"#)?;
            w.indent();
            self.write_success_enum_aware_rval(w, ctor, &self.get_method_args(ctor, "ctx"), false)?;
            w.unindent();
            indented!(w, [_ _], r#"self = {}({}.__api_lock, ctx)"#, context_type_name, context_type_name)?;
            indented!(w, [_ _], r#"return self"#)?;
            w.newline()?;
        }

        // Dtor
        indented!(w, [_], r#"def __del__(self):"#)?;
        // indented!(w, [_ _], r#"global _api, ffi"#)?;
        w.indent();
        self.write_success_enum_aware_rval(w, class.destructor(), &self.get_method_args(class.destructor(), "self._ctx"), false)?;
        w.unindent();

        for function in class.methods() {
            w.newline()?;

            let args = self.function_args_to_string(function, true, true);
            let type_hint_out = self.converter().to_type_hint_out(function.signature().rval());

            indented!(w, [_], r#"def {}(self, {}){}:"#, function.name().replace(&common_prefix, ""), &args, type_hint_out)?;
            indented!(w, [_ _], r#"{}"#, self.converter().documentation(function.meta().documentation()))?;

            w.indent();
            self.write_param_helpers(w, function)?;
            w.unindent();

            self.write_library_call(w, function, Some("self._ctx"))?;
        }

        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_library_call(&self, w: &mut IndentWriter, function: &Function, class_str: Option<&str>) -> Result<(), Error> {
        let args = match class_str {
            None => self.function_args_to_string(function, false, false),
            Some(class) => {
                w.indent();
                self.get_method_args(function, class)
            }
        };

        match function.signature().rval() {
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => {
                    indented!(w, [_], r#"rval = c_lib.{}({})"#, function.name(), &args)?;
                    indented!(w, [_], r#"return ctypes.string_at(rval)"#)?;
                }
                _ => self.write_success_enum_aware_rval(w, function, &args, true)?,
            },
            _ => self.write_success_enum_aware_rval(w, function, &args, true)?,
        }

        if class_str.is_some() {
            w.unindent();
        }

        Ok(())
    }

    fn function_args_to_string(&self, function: &Function, type_hints: bool, skip_first: bool) -> String {
        let skip = if skip_first { 1 } else { 0 };
        function
            .signature()
            .params()
            .iter()
            .skip(skip)
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

    fn write_success_enum_aware_rval(&self, w: &mut IndentWriter, function: &Function, args: &str, ret: bool) -> Result<(), Error> {
        if ret {
            indented!(w, [_], r#"return c_lib.{}({})"#, function.name(), &args)?;
        } else {
            indented!(w, [_], r#"c_lib.{}({})"#, function.name(), &args)?;
        }
        Ok(())
    }

    fn get_method_args(&self, function: &Function, ctx: &str) -> String {
        let mut args = self.function_args_to_string(function, false, true);
        args.insert_str(0, &format!("{}, ", ctx));
        args
    }

    fn write_utils(&self, w: &mut IndentWriter) -> Result<(), Error> {
        // indented!(w, r#"class Slice(ctypes.Structure, typing.Generic[T]):"#)?;
        // indented!(w, [_], r#"# These fields represent the underlying C data layout"#)?;
        // indented!(w, [_], r#"_fields_ = ["#)?;
        // indented!(w, [_], r#"    ("data", ctypes.c_void_p),"#)?;
        // indented!(w, [_], r#"    ("len", ctypes.c_uint64),"#)?;
        // indented!(w, [_], r#"]"#)?;
        // w.newline()?;
        // indented!(w, [_], r#"def cast(self, x):"#)?;
        // indented!(w, [_ _], r#"return ctypes.cast(self.data, ctypes.POINTER(x))"#)?;
        // w.newline()?;
        // w.newline()?;
        //
        // indented!(w, r#"class SliceMut(ctypes.Structure, typing.Generic[T]):"#)?;
        // indented!(w, [_], r#"# These fields represent the underlying C data layout"#)?;
        // indented!(w, [_], r#"_fields_ = ["#)?;
        // indented!(w, [_], r#"    ("data", ctypes.c_void_p),"#)?;
        // indented!(w, [_], r#"    ("len", ctypes.c_uint64),"#)?;
        // indented!(w, [_], r#"]"#)?;
        // w.newline()?;
        // indented!(w, [_], r#"def cast(self, x):"#)?;
        // indented!(w, [_ _], r#"return ctypes.cast(self.data, ctypes.POINTER(x))"#)?;
        // w.newline()?;
        // w.newline()?;

        indented!(w, r#"TRUE = ctypes.c_uint8(1)"#)?;
        indented!(w, r#"FALSE = ctypes.c_uint8(0)"#)?;
        w.newline()?;
        w.newline()?;

        indented!(w, r#"def _errcheck(returned, success):"#)?;
        indented!(w, [_], r#""""Checks for FFIErrors and converts them to an exception.""""#)?;
        indented!(w, [_], r#"if returned == success: return"#)?;
        indented!(w, [_], r#"else: raise Exception(f"Function returned error: {{returned}}")"#)?;
        w.newline()?;
        w.newline()?;

        indented!(w, r#"class CallbackVars(object):"#)?;
        indented!(
            w,
            [_],
            r#""""Helper to be used `lambda x: setattr(cv, "x", x)` when getting values from callbacks.""""#
        )?;
        indented!(w, [_], r#"def __str__(self):"#)?;
        indented!(w, [_ _], r#"rval = """#)?;
        indented!(w, [_ _], r#"for var in  filter(lambda x: "__" not in x, dir(self)):"#)?;
        indented!(w, [_ _ _], r#"rval += f"{{var}}: {{getattr(self, var)}}""#)?;
        indented!(w, [_ _], r#"return rval"#)?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_utils_primitive(&self, _: &mut IndentWriter, _: PrimitiveType) -> Result<(), Error> {
        Ok(())
    }

    fn write_all(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_imports(w)?;
        self.write_api_load_fuction(w)?;
        w.newline()?;
        w.newline()?;

        self.write_function_proxies(w)?;
        w.newline()?;
        w.newline()?;

        self.write_constants(w)?;
        w.newline()?;
        w.newline()?;

        self.write_utils(w)?;
        self.write_types(w)?;
        w.newline()?;
        w.newline()?;

        self.write_callback_helpers(w)?;
        w.newline()?;
        w.newline()?;

        self.write_patterns(w)?;

        Ok(())
    }
}
