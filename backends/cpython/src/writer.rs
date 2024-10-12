use crate::config::Config;
use crate::converter::Converter;
use interoptopus::lang::c::{CType, CompositeType, EnumType, Function, Layout, PrimitiveType};
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, safe_name, sort_types_by_dependencies};
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, non_service_functions, Error, Inventory};

/// Writes the Python file format, `impl` this trait to customize output.
pub trait PythonWriter {
    /// Returns the user config.
    fn config(&self) -> &Config;

    /// Returns the library to produce bindings for.
    fn inventory(&self) -> &Inventory;

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
        for f in self.inventory().functions() {
            let args = f
                .signature()
                .params()
                .iter()
                .map(|x| self.converter().to_ctypes_name(x.the_type(), false))
                .collect::<Vec<_>>();

            indented!(w, [_], r#"c_lib.{}.argtypes = [{}]"#, f.name(), args.join(", "))?;
        }

        w.newline()?;
        for f in self.inventory().functions() {
            let rtype = self.converter().to_ctypes_name(f.signature().rval(), false);
            if !rtype.is_empty() {
                indented!(w, [_], r#"c_lib.{}.restype = {}"#, f.name(), rtype)?;
            }
        }

        w.newline()?;
        for f in self.inventory().functions() {
            if let CType::Pattern(TypePattern::FFIErrorEnum(e)) = f.signature().rval() {
                let value = e.success_variant().value();
                indented!(w, [_], r#"c_lib.{}.errcheck = lambda rval, _fptr, _args: _errcheck(rval, {})"#, f.name(), value)?;
            }
        }

        Ok(())
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for c in self.inventory().constants() {
            indented!(w, r#"{} = {}"#, c.name(), self.converter().constant_value_to_value(c.value()))?;
        }

        Ok(())
    }

    fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        let all_types = self.inventory().ctypes().to_vec();
        let sorted_types = sort_types_by_dependencies(all_types);

        for t in &sorted_types {
            match t {
                CType::Composite(c) => self.write_struct(w, c, WriteFor::Code)?,
                CType::Enum(e) => self.write_enum(w, e, WriteFor::Code)?,
                CType::Pattern(p) => match p {
                    TypePattern::FFIErrorEnum(e) => self.write_enum(w, e.the_enum(), WriteFor::Code)?,
                    TypePattern::Slice(c) => self.write_slice(w, c, false)?,
                    TypePattern::SliceMut(c) => self.write_slice(w, c, true)?,
                    TypePattern::Option(c) => {
                        self.write_option(w, c)?;
                    }
                    _ => continue,
                },
                _ => continue,
            }

            w.newline()?;
            w.newline()?;
        }

        Ok(())
    }

    fn write_struct(&self, w: &mut IndentWriter, c: &CompositeType, write_for: WriteFor) -> Result<(), Error> {
        let documentation = c.meta().documentation().lines().join("\n");

        indented!(w, r#"class {}(ctypes.Structure):"#, c.rust_name())?;
        if !documentation.is_empty() && write_for == WriteFor::Code {
            indented!(w, [_], r#""""{}""""#, documentation)?;
        }

        match c.repr().layout() {
            Layout::Packed => indented!(w, [_], r#"_pack_ = 1"#)?,
            _ => {}
        }

        let alignment = c.repr().alignment();
        if let Some(align) = alignment {
            indented!(w, [_], r#"_align_ = {}"#, align)?;
        }

        w.newline()?;
        if write_for == WriteFor::Code {
            indented!(w, [_], r#"# These fields represent the underlying C data layout"#)?;
        }
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
                let type_hint_in = self.converter().to_type_hint_in(x.the_type(), false);

                format!("{}{} = None", x.name(), type_hint_in)
            })
            .collect::<Vec<_>>()
            .join(", ");

        if !c.fields().is_empty() {
            w.newline()?;
            indented!(w, [_], r#"def __init__(self, {}):"#, extra_args)?;

            if write_for == WriteFor::Code {
                for field in c.fields().iter() {
                    indented!(w, [_ _], r#"if {} is not None:"#, field.name())?;
                    indented!(w, [_ _ _], r#"self.{} = {}"#, field.name(), field.name())?;
                }
            } else {
                indented!(w, [_ _], r#"..."#)?;
            }
        }

        if write_for == WriteFor::Docs {
            return Ok(());
        }

        // Fields
        for f in c.fields() {
            let documentation = f.documentation().lines().join("\n");

            w.newline()?;

            let hint_in = self.converter().to_type_hint_in(f.the_type(), false);
            let hint_out = self.converter().to_type_hint_out(f.the_type());

            indented!(w, [_], r#"@property"#)?;
            indented!(w, [_], r#"def {}(self){}:"#, f.name(), hint_out)?;

            if !documentation.is_empty() {
                indented!(w, [_ _], r#""""{}""""#, documentation)?;
            }

            match f.the_type() {
                CType::Pattern(_) => indented!(w, [_ _], r#"return ctypes.Structure.__get__(self, "{}")"#, f.name())?,
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

    fn write_enum(&self, w: &mut IndentWriter, e: &EnumType, write_for: WriteFor) -> Result<(), Error> {
        let documentation = e.meta().documentation().lines().join("\n");

        indented!(w, r#"class {}:"#, e.rust_name())?;
        if !documentation.is_empty() && write_for == WriteFor::Code {
            indented!(w, [_], r#""""{}""""#, documentation)?;
        }

        for v in e.variants() {
            if write_for == WriteFor::Code {
                for line in v.documentation().lines() {
                    indented!(w, [_], r#"# {}"#, line)?;
                }
            }
            indented!(w, [_], r#"{} = {}"#, v.name(), v.value())?;
        }

        Ok(())
    }

    fn write_callback_helpers(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"class {}:"#, self.config().callback_namespace)?;
        indented!(w, [_], r#""""Helpers to define callbacks.""""#)?;

        for callback in self.inventory().ctypes().iter().filter_map(|x| match x {
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
        for function in non_service_functions(self.inventory()) {
            self.write_function(w, function, WriteFor::Code)?;
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
        let rval_sig = self.converter().to_type_hint_out(function.signature().rval());
        let args = self.function_args_to_string(function, true, false);
        let documentation = function.meta().documentation().lines().join("\n");

        indented!(w, r#"def {}({}){}:"#, function.name(), args, rval_sig)?;

        if write_for == WriteFor::Code {
            if !documentation.is_empty() {
                indented!(w, [_], r#""""{}""""#, documentation)?;
            }

            self.write_param_helpers(w, function)?;
            self.write_library_call(w, function, None)?;
            w.newline()?;
        } else {
            indented!(w, [_], r#"..."#)?;
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
                    TypePattern::CStrPointer => {
                        indented!(w, [_], r#"if not hasattr({}, "__ctypes_from_outparam__"):"#, arg.name())?;
                        indented!(w, [_ _], r#"{} = ctypes.cast({}, ctypes.POINTER(ctypes.c_char))"#, arg.name(), arg.name())?;
                    }
                    TypePattern::Slice(t) | TypePattern::SliceMut(t) => {
                        let inner = self.converter().to_ctypes_name(
                            t.fields()
                                .iter()
                                .find(|i| i.name().eq_ignore_ascii_case("data"))
                                .expect("slice must have a data field")
                                .the_type()
                                .deref_pointer()
                                .expect("data must be a pointer type"),
                            false,
                        );

                        indented!(
                            w,
                            [_],
                            r#"if hasattr({}, "_length_") and getattr({}, "_type_", "") == {}:"#,
                            arg.name(),
                            arg.name(),
                            inner
                        )?;

                        indented!(w, [_ _], r#"{} = {}(data=ctypes.cast({}, ctypes.POINTER({})), len=len({}))"#,
                                arg.name(), arg.the_type().name_within_lib(), arg.name(), inner, arg.name())?;
                        w.newline()?;
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
        let hint_in = self.converter().to_type_hint_in(data_type, false);
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
        indented!(w, [_ _], r#"if i < 0:"#)?;
        indented!(w, [_ _ _], r#"index = self.len+i"#)?;
        indented!(w, [_ _], r#"else:"#)?;
        indented!(w, [_ _ _], r#"index = i"#)?;
        w.newline()?;
        indented!(w, [_ _], r#"if index >= self.len:"#)?;
        indented!(w, [_ _ _], r#"raise IndexError("Index out of range")"#)?;
        w.newline()?;
        indented!(w, [_ _], r#"return self.data[index]"#)?;

        if mutable {
            w.newline()?;
            indented!(w, [_], r#"def __setitem__(self, i, v{}):"#, hint_in)?;
            indented!(w, [_ _], r#"if i < 0:"#)?;
            indented!(w, [_ _ _], r#"index = self.len+i"#)?;
            indented!(w, [_ _], r#"else:"#)?;
            indented!(w, [_ _ _], r#"index = i"#)?;
            w.newline()?;
            indented!(w, [_ _], r#"if index >= self.len:"#)?;
            indented!(w, [_ _ _], r#"raise IndexError("Index out of range")"#)?;
            w.newline()?;
            indented!(w, [_ _], r#"self.data[index] = v"#)?;
        }

        w.newline()?;
        indented!(w, [_], r#"def copied(self) -> {}:"#, c.rust_name())?;
        indented!(w, [_ _], r#""""Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely.""""#)?;
        indented!(w, [_ _], r#"array = ({} * len(self))()"#, data_type_python)?;
        indented!(w, [_ _], r#"ctypes.memmove(array, self.data, len(self) * ctypes.sizeof({}))"#, data_type_python)?;
        indented!(w, [_ _], r#"rval = {}(data=ctypes.cast(array, ctypes.POINTER({})), len=len(self))"#, c.rust_name(), data_type_python)?;
        indented!(w, [_ _], r#"rval.owned = array  # Store array in returned slice to prevent memory deallocation"#)?;
        indented!(w, [_ _], r#"return rval"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __iter__(self) -> typing.Iterable[{}]:"#, data_type_python)?;
        indented!(w, [_ _], r#"return _Iter(self)"#)?;
        w.newline()?;
        indented!(w, [_], r#"def iter(self) -> typing.Iterable[{}]:"#, data_type_python)?;
        indented!(w, [_ _], r#""""Convenience method returning a value iterator.""""#)?;
        indented!(w, [_ _], r#"return iter(self)"#)?;
        w.newline()?;
        indented!(w, [_], r#"def first(self){}:"#, hint_out)?;
        indented!(w, [_ _], r#""""Returns the first element of this slice.""""#)?;
        indented!(w, [_ _], r#"return self[0]"#)?;
        w.newline()?;
        indented!(w, [_], r#"def last(self){}:"#, hint_out)?;
        indented!(w, [_ _], r#""""Returns the last element of this slice.""""#)?;
        indented!(w, [_ _], r#"return self[len(self)-1]"#)?;

        // Only write this for byte-like types right now
        if data_type.size_of() == 1 {
            w.newline()?;
            indented!(w, [_], r#"def bytearray(self):"#)?;
            indented!(w, [_ _], r#""""Returns a bytearray with the content of this slice.""""#)?;
            indented!(w, [_ _], r#"rval = bytearray(len(self))"#)?;
            indented!(w, [_ _], r#"for i in range(len(self)):"#)?;
            indented!(w, [_ _ _], r#"rval[i] = self[i]"#)?;
            indented!(w, [_ _], r#"return rval"#)?;
        }

        Ok(())
    }

    fn write_option(&self, w: &mut IndentWriter, c: &CompositeType) -> Result<(), Error> {
        let data_type = c
            .fields()
            .iter()
            .find(|x| x.name().contains('t'))
            .expect("Slice must contain field called 't'.")
            .the_type();

        let data_type_python = self.converter().to_ctypes_name(data_type, true);

        indented!(w, r#"class {}(ctypes.Structure):"#, c.rust_name())?;
        indented!(w, [_], r#""""May optionally hold a value.""""#)?;
        w.newline()?;
        indented!(w, [_], r#"_fields_ = ["#)?;
        indented!(w, [_], r#"    ("_t", {}),"#, data_type_python)?;
        indented!(w, [_], r#"    ("_is_some", ctypes.c_uint8),"#)?;
        indented!(w, [_], r#"]"#)?;
        w.newline()?;
        indented!(w, [_], r#"@property"#)?;
        indented!(w, [_], r#"def value(self) -> {}:"#, data_type_python)?;
        indented!(w, [_ _], r#""""Returns the value if it exists, or None.""""#)?;
        indented!(w, [_ _], r#"if self._is_some == 1:"#)?;
        indented!(w, [_ _ _], r#"return self._t"#)?;
        indented!(w, [_ _], r#"else:"#)?;
        indented!(w, [_ _ _], r#"return None"#)?;
        w.newline()?;
        indented!(w, [_], r#"def is_some(self) -> bool:"#)?;
        indented!(w, [_ _], r#""""Returns true if the value exists.""""#)?;
        indented!(w, [_ _], r#"return self._is_some == 1"#)?;
        w.newline()?;
        indented!(w, [_], r#"def is_none(self) -> bool:"#)?;
        indented!(w, [_ _], r#""""Returns true if the value does not exist.""""#)?;
        indented!(w, [_ _], r#"return self._is_some != 0"#)?;

        Ok(())
    }

    fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.inventory().patterns() {
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

        let _common_prefix = longest_common_prefix(&all_functions);
        let documentation = class.the_type().meta().documentation().lines().join("\n");

        indented!(w, r#"class {}:"#, context_type_name)?;
        if !documentation.is_empty() {
            indented!(w, [_], r#""""{}""""#, documentation)?;
        }
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
            self.write_pattern_class_ctor(w, class, ctor, WriteFor::Code)?;
        }

        // Dtor
        indented!(w, [_], r#"def __del__(self):"#)?;
        // indented!(w, [_ _], r#"global _api, ffi"#)?;
        w.indent();
        self.write_success_enum_aware_rval(w, class.destructor(), &self.get_method_args(class.destructor(), "self._ctx"), false)?;
        w.unindent();

        for function in class.methods() {
            self.write_pattern_class_method(w, class, function, WriteFor::Code)?;
        }

        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_pattern_class_ctor(&self, w: &mut IndentWriter, class: &Service, ctor: &Function, write_for: WriteFor) -> Result<(), Error> {
        let context_type_name = class.the_type().rust_name();
        let mut all_functions = class.constructors().to_vec();
        all_functions.extend_from_slice(class.methods());
        all_functions.push(class.destructor().clone());

        let common_prefix = longest_common_prefix(&all_functions);

        let ctor_args = self.function_args_to_string(ctor, true, true);
        indented!(w, [_], r#"@staticmethod"#)?;
        indented!(w, [_], r#"def {}({}) -> {}:"#, ctor.name().replace(&common_prefix, ""), ctor_args, context_type_name)?;

        if write_for == WriteFor::Docs {
            return Ok(());
        }

        indented!(w, [_ _], r#"{}"#, self.converter().documentation(ctor.meta().documentation()))?;
        indented!(w, [_ _], r#"ctx = ctypes.c_void_p()"#)?;
        w.indent();
        self.write_param_helpers(w, ctor)?;
        self.write_success_enum_aware_rval(w, ctor, &self.get_method_args(ctor, "ctx"), false)?;
        w.unindent();
        indented!(w, [_ _], r#"self = {}({}.__api_lock, ctx)"#, context_type_name, context_type_name)?;
        indented!(w, [_ _], r#"return self"#)?;
        w.newline()?;

        Ok(())
    }

    fn write_pattern_class_method(&self, w: &mut IndentWriter, class: &Service, function: &Function, write_for: WriteFor) -> Result<(), Error> {
        let mut all_functions = class.constructors().to_vec();
        all_functions.extend_from_slice(class.methods());
        all_functions.push(class.destructor().clone());

        let common_prefix = longest_common_prefix(&all_functions);

        let args = self.function_args_to_string(function, true, true);
        let type_hint_out = self.converter().to_type_hint_out(function.signature().rval());

        indented!(w, [_], r#"def {}(self, {}){}:"#, function.name().replace(&common_prefix, ""), &args, type_hint_out)?;

        if write_for == WriteFor::Docs {
            return Ok(());
        }

        indented!(w, [_ _], r#"{}"#, self.converter().documentation(function.meta().documentation()))?;

        w.indent();
        self.write_param_helpers(w, function)?;
        w.unindent();

        self.write_library_call(w, function, Some("self._ctx"))?;
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
            CType::Pattern(TypePattern::CStrPointer) => {
                indented!(w, [_], r#"rval = c_lib.{}({})"#, function.name(), &args)?;
                indented!(w, [_], r#"return ctypes.string_at(rval)"#)?;
            }
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
                    self.converter().to_type_hint_in(x.the_type(), true)
                } else {
                    "".to_string()
                };
                format!("{}{}", x.name(), type_hint)
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

        indented!(w, r#"class _Iter(object):"#)?;
        indented!(w, [_], r#""""Helper for slice iterators.""""#)?;
        indented!(w, [_], r#"def __init__(self, target):"#)?;
        indented!(w, [_ _], r#"self.i = 0"#)?;
        indented!(w, [_ _], r#"self.target = target"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __iter__(self):"#)?;
        indented!(w, [_ _], r#"self.i = 0"#)?;
        indented!(w, [_ _], r#"return self"#)?;
        w.newline()?;
        indented!(w, [_], r#"def __next__(self):"#)?;
        indented!(w, [_ _], r#"if self.i >= self.target.len:"#)?;
        indented!(w, [_ _ _], r#"raise StopIteration()"#)?;
        indented!(w, [_ _], r#"rval = self.target[self.i]"#)?;
        indented!(w, [_ _], r#"self.i += 1"#)?;
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
