use crate::overloads::{write_function_overloaded_invoke_with_error_handling, Helper};
use crate::{OverloadWriter, Unsafe};
use interoptopus::lang::c::{CType, Function, FunctionSignature, Parameter, PrimitiveType};
use interoptopus::patterns::service::Service;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};
use std::ops::Deref;

pub struct DotNet {}

impl DotNet {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn has_overloadable(&self, signature: &FunctionSignature) -> bool {
        signature.params().iter().any(|x| match x.the_type() {
            CType::ReadPointer(x) | CType::ReadWritePointer(x) => match x.deref() {
                CType::Pattern(x) => matches!(x, TypePattern::Slice(_) | TypePattern::SliceMut(_)),
                _ => false,
            },
            CType::Pattern(x) => matches!(x, TypePattern::Slice(_) | TypePattern::SliceMut(_)),
            _ => false,
        })
    }

    fn pattern_to_native_in_signature(&self, h: &Helper, param: &Parameter, _signature: &FunctionSignature) -> String {
        match param.the_type() {
            CType::Pattern(p) => match p {
                TypePattern::Slice(p) => {
                    let element_type = p
                        .fields()
                        .get(0)
                        .expect("First parameter must exist")
                        .the_type()
                        .deref_pointer()
                        .expect("Must be pointer");

                    format!("{}[]", h.converter.to_typespecifier_in_param(element_type))
                }
                TypePattern::SliceMut(p) => {
                    let element_type = p
                        .fields()
                        .get(0)
                        .expect("First parameter must exist")
                        .the_type()
                        .deref_pointer()
                        .expect("Must be pointer");
                    format!("{}[]", h.converter.to_typespecifier_in_param(element_type))
                }
                _ => h.converter.to_typespecifier_in_param(param.the_type()),
            },
            CType::ReadPointer(x) | CType::ReadWritePointer(x) => match x.deref() {
                CType::Pattern(x) => match x {
                    TypePattern::Slice(p) => {
                        let element_type = p
                            .fields()
                            .get(0)
                            .expect("First parameter must exist")
                            .the_type()
                            .deref_pointer()
                            .expect("Must be pointer");

                        format!("{}[]", h.converter.to_typespecifier_in_param(element_type))
                    }
                    TypePattern::SliceMut(p) => {
                        let element_type = p
                            .fields()
                            .get(0)
                            .expect("First parameter must exist")
                            .the_type()
                            .deref_pointer()
                            .expect("Must be pointer");

                        format!("{}[]", h.converter.to_typespecifier_in_param(element_type))
                    }
                    _ => h.converter.to_typespecifier_in_param(param.the_type()),
                },
                _ => h.converter.to_typespecifier_in_param(param.the_type()),
            },

            x => h.converter.to_typespecifier_in_param(x),
        }
    }
}

impl OverloadWriter for DotNet {
    fn write_imports(&self, w: &mut IndentWriter, h: Helper) -> Result<(), Error> {
        if h.config.use_unsafe == Unsafe::UnsafePlatformMemCpy {
            indented!(w, r#"using System.Runtime.CompilerServices;"#)?;
        }
        Ok(())
    }

    fn write_function_overload(&self, w: &mut IndentWriter, h: Helper, function: &Function) -> Result<(), Error> {
        let has_overload = self.has_overloadable(function.signature());
        let has_error_enum = h.converter.has_ffi_error_rval(function.signature());

        // If there is nothing to write, don't do it
        if !has_overload && !has_error_enum {
            return Ok(());
        }

        let mut to_pin_name = Vec::new();
        let mut to_pin_slice_type = Vec::new();
        let mut to_invoke = Vec::new();
        let raw_name = h.converter.function_name_to_csharp_name(function);
        let this_name = if has_error_enum && !has_overload {
            format!("{}_checked", raw_name)
        } else {
            raw_name.clone()
        };

        let rval = match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
            _ => h.converter.to_typespecifier_in_rval(function.signature().rval()),
        };

        let mut params = Vec::new();
        for (_, p) in function.signature().params().iter().enumerate() {
            let name = p.name();
            let native = self.pattern_to_native_in_signature(&h, p, function.signature());
            let the_type = h.converter.function_parameter_to_csharp_typename(p, function);

            let mut fallback = || {
                if native.contains("out ") {
                    to_invoke.push(format!("out {}", name.to_string()));
                } else if native.contains("ref ") {
                    to_invoke.push(format!("ref {}", name.to_string()));
                } else {
                    to_invoke.push(name.to_string());
                }
            };

            match p.the_type() {
                CType::Pattern(TypePattern::Slice(_) | TypePattern::SliceMut(_)) => {
                    to_pin_name.push(name);
                    to_pin_slice_type.push(the_type);
                    to_invoke.push(format!("{}_slice", name));
                }
                CType::ReadPointer(x) | CType::ReadWritePointer(x) => match x.deref() {
                    CType::Pattern(x) => match x {
                        TypePattern::Slice(_) => {
                            to_pin_name.push(name);
                            to_pin_slice_type.push(the_type.replace("ref ", ""));
                            to_invoke.push(format!("ref {}_slice", name));
                        }
                        TypePattern::SliceMut(_) => {
                            to_pin_name.push(name);
                            to_pin_slice_type.push(the_type.replace("ref ", ""));
                            to_invoke.push(format!("ref {}_slice", name));
                        }
                        _ => fallback(),
                    },
                    _ => fallback(),
                },
                _ => fallback(),
            }

            params.push(format!("{} {}", native, name));
        }

        indented!(w, r#"public static {} {}({}) {{"#, rval, this_name, params.join(", "))?;

        if h.config.use_unsafe.any_unsafe() {
            if !to_pin_name.is_empty() {
                indented!(w, [_], r#"unsafe"#)?;
                indented!(w, [_], r#"{{"#)?;
                w.indent();

                for (pin_var, slice_struct) in to_pin_name.iter().zip(to_pin_slice_type.iter()) {
                    indented!(w, [_], r#"fixed (void* ptr_{} = {})"#, pin_var, pin_var)?;
                    indented!(w, [_], r#"{{"#)?;
                    indented!(w, [_ _], r#"var {}_slice = new {}(new IntPtr(ptr_{}), (ulong) {}.Length);"#, pin_var, slice_struct, pin_var, pin_var)?;
                    w.indent();
                }
            }

            let call = format!(r#"{}({});"#, raw_name, to_invoke.join(", "));
            write_function_overloaded_invoke_with_error_handling(w, function, &call)?;

            if !to_pin_name.is_empty() {
                for _ in to_pin_name.iter() {
                    w.unindent();
                    indented!(w, [_], r#"}}"#)?;
                }

                w.unindent();
                indented!(w, [_], r#"}}"#)?;
            }
        } else {
            if !to_pin_name.is_empty() {
                for (pin_var, slice_struct) in to_pin_name.iter().zip(to_pin_slice_type.iter()) {
                    indented!(w, [_], r#"var {}_pinned = GCHandle.Alloc({}, GCHandleType.Pinned);"#, pin_var, pin_var)?;
                    indented!(
                        w,
                        [_],
                        r#"var {}_slice = new {}({}_pinned, (ulong) {}.Length);"#,
                        pin_var,
                        slice_struct,
                        pin_var,
                        pin_var
                    )?;
                }

                indented!(w, [_], r#"try"#)?;
                indented!(w, [_], r#"{{"#)?;

                w.indent();
            }

            let call = format!(r#"{}({});"#, raw_name, to_invoke.join(", "));
            write_function_overloaded_invoke_with_error_handling(w, function, &call)?;

            if !to_pin_name.is_empty() {
                w.unindent();
                indented!(w, [_], r#"}}"#)?;
                indented!(w, [_], r#"finally"#)?;
                indented!(w, [_], r#"{{"#)?;
                for pin in &to_pin_name {
                    indented!(w, [_ _], r#"{}_pinned.Free();"#, pin)?;
                }
                indented!(w, [_], r#"}}"#)?;
            }
        }

        indented!(w, r#"}}"#)
    }

    fn write_service_method_overload(&self, w: &mut IndentWriter, h: Helper, _class: &Service, function: &Function, fn_pretty: &str) -> Result<(), Error> {
        if !self.has_overloadable(function.signature()) {
            return Ok(());
        }

        let mut names = Vec::new();
        let mut to_invoke = Vec::new();
        let mut types = Vec::new();

        // Write checked method. These are "normal" methods that accept
        // common C# types.
        let rval = match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
            _ => h.converter.to_typespecifier_in_rval(function.signature().rval()),
        };

        // For every parameter except the first, figure out how we should forward
        // it to the invocation we perform.
        for p in function.signature().params().iter().skip(1) {
            let name = p.name();

            // If we call the checked function we want to resolve a `SliceU8` to a `byte[]`,
            // but if we call the unchecked version we want to keep that `Sliceu8` in our signature.
            // let native = self.to_typespecifier_in_param(p.the_type());
            let native = self.pattern_to_native_in_signature(&h, p, function.signature());

            // Forward `ref` and `out` accordingly.
            if native.contains("out ") {
                to_invoke.push(format!("out {}", name.to_string()));
            } else if native.contains("ref ") {
                to_invoke.push(format!("ref {}", name.to_string()));
            } else {
                to_invoke.push(name.to_string());
            }

            names.push(name);
            types.push(native);
        }

        let method_to_invoke = function.name().to_string();
        let extra_args = if to_invoke.is_empty() {
            "".to_string()
        } else {
            format!(", {}", to_invoke.join(", "))
        };

        // Assemble actual function call.
        let context = "_context";
        let arg_tokens = names.iter().zip(types.iter()).map(|(n, t)| format!("{} {}", t, n)).collect::<Vec<_>>();
        let fn_call = format!(r#"{}.{}({}{})"#, h.config.class, method_to_invoke, context, extra_args);

        // Write signature.
        indented!(w, r#"public {} {}({})"#, rval, fn_pretty, arg_tokens.join(", "))?;
        indented!(w, r#"{{"#)?;

        match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => {
                indented!(w, [_], r#"{};"#, fn_call)?;
            }
            CType::Primitive(PrimitiveType::Void) => {
                indented!(w, [_], r#"{};"#, fn_call)?;
            }
            _ => {
                indented!(w, [_], r#"return {};"#, fn_call)?;
            }
        }

        indented!(w, r#"}}"#)?;

        Ok(())
    }

    fn write_pattern_slice_overload(&self, _w: &mut IndentWriter, _h: Helper, _context_type_name: &str, _type_string: &str) -> Result<(), Error> {
        Ok(())
    }

    fn write_pattern_slice_unsafe_copied_fragment(&self, w: &mut IndentWriter, _h: Helper, _type_string: &str) -> Result<(), Error> {
        indented!(w, [_ _ _ _ _], r#"#elif NETCOREAPP"#)?;
        indented!(w, [_ _ _ _ _], r#"Unsafe.CopyBlock(dst, data.ToPointer(), (uint)len);"#)?;
        Ok(())
    }
}
