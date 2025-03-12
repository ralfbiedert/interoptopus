use crate::Interop;
use crate::converter::{documentation, to_ctypes_name, to_type_hint_in, to_type_hint_out};
use crate::interop::functions::write_param_helpers;
use crate::interop::utils::write_success_enum_aware_rval;
use interoptopus::backend::util::longest_common_prefix;
use interoptopus::backend::writer::{IndentWriter, WriteFor};
use interoptopus::lang::c::{CType, CompositeType, Function};
use interoptopus::patterns::service::ServiceDefinition;
use interoptopus::patterns::slice::SliceType;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::{Error, indented};

pub fn write_slice(_i: &Interop, w: &mut IndentWriter, c: &SliceType, mutable: bool) -> Result<(), Error> {
    let data_type = c.target_type();
    let data_type_python = to_ctypes_name(data_type, true);
    let hint_in = to_type_hint_in(data_type, false);
    let hint_out = to_type_hint_out(data_type);

    indented!(w, r"class {}(ctypes.Structure):", c.rust_name())?;
    indented!(w, [()], r"# These fields represent the underlying C data layout")?;
    indented!(w, [()], r"_fields_ = [")?;
    indented!(w, [()], r#"    ("data", ctypes.POINTER({})),"#, data_type_python)?;
    indented!(w, [()], r#"    ("len", ctypes.c_uint64),"#)?;
    indented!(w, [()], r"]")?;
    w.newline()?;
    indented!(w, [()], r"def __len__(self):")?;
    indented!(w, [()()], r"return self.len")?;
    w.newline()?;
    indented!(w, [()], r"def __getitem__(self, i){}:", hint_out)?;
    indented!(w, [()()], r"if i < 0:")?;
    indented!(w, [()()()], r"index = self.len+i")?;
    indented!(w, [()()], r"else:")?;
    indented!(w, [()()()], r"index = i")?;
    w.newline()?;
    indented!(w, [()()], r"if index >= self.len:")?;
    indented!(w, [()()()], r#"raise IndexError("Index out of range")"#)?;
    w.newline()?;
    indented!(w, [()()], r"return self.data[index]")?;

    if mutable {
        w.newline()?;
        indented!(w, [()], r"def __setitem__(self, i, v{}):", hint_in)?;
        indented!(w, [()()], r"if i < 0:")?;
        indented!(w, [()()()], r"index = self.len+i")?;
        indented!(w, [()()], r"else:")?;
        indented!(w, [()()()], r"index = i")?;
        w.newline()?;
        indented!(w, [()()], r"if index >= self.len:")?;
        indented!(w, [()()()], r#"raise IndexError("Index out of range")"#)?;
        w.newline()?;
        indented!(w, [()()], r"self.data[index] = v")?;
    }

    w.newline()?;
    indented!(w, [()], r"def copied(self) -> {}:", c.rust_name())?;
    indented!(
        w,
        [()()],
        r#""""Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely.""""#
    )?;
    indented!(w, [()()], r"array = ({} * len(self))()", data_type_python)?;
    indented!(w, [()()], r"ctypes.memmove(array, self.data, len(self) * ctypes.sizeof({}))", data_type_python)?;
    indented!(w, [()()], r"rval = {}(data=ctypes.cast(array, ctypes.POINTER({})), len=len(self))", c.rust_name(), data_type_python)?;
    indented!(w, [()()], r"rval.owned = array  # Store array in returned slice to prevent memory deallocation")?;
    indented!(w, [()()], r"return rval")?;
    w.newline()?;
    indented!(w, [()], r"def __iter__(self) -> typing.Iterable[{}]:", data_type_python)?;
    indented!(w, [()()], r"return _Iter(self)")?;
    w.newline()?;
    indented!(w, [()], r"def iter(self) -> typing.Iterable[{}]:", data_type_python)?;
    indented!(w, [()()], r#""""Convenience method returning a value iterator.""""#)?;
    indented!(w, [()()], r"return iter(self)")?;
    w.newline()?;
    indented!(w, [()], r"def first(self){}:", hint_out)?;
    indented!(w, [()()], r#""""Returns the first element of this slice.""""#)?;
    indented!(w, [()()], r"return self[0]")?;
    w.newline()?;
    indented!(w, [()], r"def last(self){}:", hint_out)?;
    indented!(w, [()()], r#""""Returns the last element of this slice.""""#)?;
    indented!(w, [()()], r"return self[len(self)-1]")?;

    // Only write this for byte-like types right now
    if data_type.size_of() == 1 {
        w.newline()?;
        indented!(w, [()], r"def bytearray(self):")?;
        indented!(w, [()()], r#""""Returns a bytearray with the content of this slice.""""#)?;
        indented!(w, [()()], r"rval = bytearray(len(self))")?;
        indented!(w, [()()], r"for i in range(len(self)):")?;
        indented!(w, [()()()], r"rval[i] = self[i]")?;
        indented!(w, [()()], r"return rval")?;
    }

    Ok(())
}

pub fn write_option(_i: &Interop, w: &mut IndentWriter, c: &CompositeType) -> Result<(), Error> {
    let data_type = c
        .fields()
        .iter()
        .find(|x| x.name().contains('t'))
        .expect("Slice must contain field called 't'.")
        .the_type();

    let data_type_python = to_ctypes_name(data_type, true);

    indented!(w, r"class {}(ctypes.Structure):", c.rust_name())?;
    indented!(w, [()], r#""""May optionally hold a value.""""#)?;
    w.newline()?;
    indented!(w, [()], r"_fields_ = [")?;
    indented!(w, [()], r#"    ("_t", {}),"#, data_type_python)?;
    indented!(w, [()], r#"    ("_is_some", ctypes.c_uint8),"#)?;
    indented!(w, [()], r"]")?;
    w.newline()?;
    indented!(w, [()], r"@property")?;
    indented!(w, [()], r"def value(self) -> {}:", data_type_python)?;
    indented!(w, [()()], r#""""Returns the value if it exists, or None.""""#)?;
    indented!(w, [()()], r"if self._is_some == 1:")?;
    indented!(w, [()()()], r"return self._t")?;
    indented!(w, [()()], r"else:")?;
    indented!(w, [()()()], r"return None")?;
    w.newline()?;
    indented!(w, [()], r"def is_some(self) -> bool:")?;
    indented!(w, [()()], r#""""Returns true if the value exists.""""#)?;
    indented!(w, [()()], r"return self._is_some == 1")?;
    w.newline()?;
    indented!(w, [()], r"def is_none(self) -> bool:")?;
    indented!(w, [()()], r#""""Returns true if the value does not exist.""""#)?;
    indented!(w, [()()], r"return self._is_some != 0")?;

    Ok(())
}

pub fn write_patterns(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for pattern in i.inventory.patterns() {
        match pattern {
            LibraryPattern::Service(x) => write_pattern_class(i, w, x)?,
            LibraryPattern::Builtins(_) => { /* TODO */ }
            _ => panic!("Pattern not explicitly handled"),
        }
    }

    Ok(())
}

pub fn write_pattern_class(i: &Interop, w: &mut IndentWriter, class: &ServiceDefinition) -> Result<(), Error> {
    let context_type_name = class.the_type().rust_name();

    let mut all_functions = class.constructors().to_vec();
    all_functions.extend_from_slice(class.methods());
    all_functions.push(class.destructor().clone());

    let _common_prefix = longest_common_prefix(&all_functions);
    let documentation = class.the_type().meta().documentation().lines().join("\n");

    indented!(w, r"class {}:", context_type_name)?;
    if !documentation.is_empty() {
        indented!(w, [()], r#""""{}""""#, documentation)?;
    }
    indented!(w, [()], r"__api_lock = object()")?;
    w.newline()?;
    indented!(w, [()], r"def __init__(self, api_lock, ctx):")?;
    indented!(w, [()()], r#"assert(api_lock == {}.__api_lock), "You must create this with a static constructor." "#, context_type_name)?;
    indented!(w, [()()], r"self._ctx = ctx")?;
    w.newline()?;
    indented!(w, [()], r"@property")?;
    indented!(w, [()], r"def _as_parameter_(self):")?;
    indented!(w, [()()], r"return self._ctx")?;
    w.newline()?;

    for ctor in class.constructors() {
        write_pattern_class_ctor(i, w, class, ctor, WriteFor::Code)?;
    }

    // Dtor
    indented!(w, [()], r"def __del__(self):")?;
    // indented!(w, [_ _], r#"global _api, ffi"#)?;
    w.indent();
    write_success_enum_aware_rval(i, w, class.destructor(), &i.get_method_args(class.destructor(), "self._ctx"), false)?;
    w.unindent();

    for function in class.methods() {
        write_pattern_class_method(i, w, class, function, WriteFor::Code)?;
    }

    w.newline()?;
    w.newline()?;

    Ok(())
}

pub fn write_pattern_class_ctor(i: &Interop, w: &mut IndentWriter, class: &ServiceDefinition, ctor: &Function, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_pattern_class_ctor")?;

    let context_type_name = class.the_type().rust_name();
    let mut all_functions = class.constructors().to_vec();
    all_functions.extend_from_slice(class.methods());
    all_functions.push(class.destructor().clone());

    let common_prefix = longest_common_prefix(&all_functions);

    let ctor_args = i.function_args_to_string(ctor, true, true);
    indented!(w, [()], r"@staticmethod")?;
    indented!(w, [()], r"def {}({}) -> {}:", ctor.name().replace(&common_prefix, ""), ctor_args, context_type_name)?;

    if write_for == WriteFor::Docs {
        return Ok(());
    }

    indented!(w, [()()], r"{}", documentation(ctor.meta().documentation()))?;
    w.indent();
    write_param_helpers(i, w, ctor)?;
    let invokes = i.function_args_to_string(ctor, false, true);
    indented!(w, [()], r"ctx = c_lib.{}({invokes}).t", ctor.name())?;
    w.unindent();
    indented!(w, [()()], r"self = {}({}.__api_lock, ctx)", context_type_name, context_type_name)?;
    indented!(w, [()()], r"return self")?;
    w.newline()?;

    Ok(())
}

pub fn write_pattern_class_method(i: &Interop, w: &mut IndentWriter, class: &ServiceDefinition, function: &Function, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_pattern_class_method")?;
    let mut all_functions = class.constructors().to_vec();
    all_functions.extend_from_slice(class.methods());
    all_functions.push(class.destructor().clone());

    let common_prefix = longest_common_prefix(&all_functions);

    let args = i.function_args_to_string(function, true, true);
    let type_hint_out = to_type_hint_out(function.signature().rval());

    indented!(w, [()], r"def {}(self, {}){}:", function.name().replace(&common_prefix, ""), &args, type_hint_out)?;

    if write_for == WriteFor::Docs {
        return Ok(());
    }

    indented!(w, [()()], r"{}", documentation(function.meta().documentation()))?;

    w.indent();
    write_param_helpers(i, w, function)?;
    w.unindent();

    write_library_call(i, w, function, Some("self._ctx"))?;
    w.newline()?;

    Ok(())
}

pub fn write_library_call(i: &Interop, w: &mut IndentWriter, function: &Function, class_str: Option<&str>) -> Result<(), Error> {
    let args = match class_str {
        None => i.function_args_to_string(function, false, false),
        Some(class) => {
            w.indent();
            i.get_method_args(function, class)
        }
    };

    match function.signature().rval() {
        CType::Pattern(TypePattern::CStrPointer) => {
            indented!(w, [()], r"rval = c_lib.{}({})", function.name(), &args)?;
            indented!(w, [()], r"return ctypes.string_at(rval)")?;
        }
        _ => write_success_enum_aware_rval(i, w, function, &args, true)?,
    }

    if class_str.is_some() {
        w.unindent();
    }

    Ok(())
}
