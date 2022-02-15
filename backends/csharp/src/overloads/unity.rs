use crate::overloads::{write_common_service_method_overload, write_function_overloaded_invoke_with_error_handling, Helper};
use crate::{OverloadWriter, Unsafe};
use interoptopus::lang::c::{CType, CompositeType, Field, Function, FunctionSignature, Parameter};
use interoptopus::patterns::service::Service;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};
use std::ops::Deref;

/// Provides Unity overloads, make sure to use [`Unsafe::UnsafeKeyword`](crate::Unsafe::UnsafeKeyword) or higher.
///
/// This provider adds convenience methods when working with `unsafe` and Burst Unity. It adds:
///
/// - signatures with `NativeArray<>`,
/// - fast memcpy for slices,
/// - handling of the `ref Slice` pattern for Burst,
/// - IntPtr overloads for callbacks, making them Burst compatible.
pub struct Unity {}

impl Unity {
    /// Creates a new Unity overload generator.
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn has_overloadable(&self, signature: &FunctionSignature) -> bool {
        self.has_non_delegate_overloadable(signature) || self.has_delegate(signature)
    }

    fn has_non_delegate_overloadable(&self, signature: &FunctionSignature) -> bool {
        signature.params().iter().any(|x| match x.the_type() {
            CType::ReadPointer(x) | CType::ReadWritePointer(x) => match x.deref() {
                CType::Pattern(x) => matches!(x, TypePattern::Slice(_) | TypePattern::SliceMut(_)),
                _ => false,
            },
            CType::Pattern(x) => matches!(x, TypePattern::Slice(_) | TypePattern::SliceMut(_)),
            _ => false,
        })
    }

    fn has_delegate(&self, signature: &FunctionSignature) -> bool {
        signature
            .params()
            .iter()
            .any(|x| matches!(x.the_type(), CType::FnPointer(_) | CType::Pattern(TypePattern::NamedCallback(_))))
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

                    format!("NativeArray<{}>", h.converter.to_typespecifier_in_param(element_type))
                }
                TypePattern::SliceMut(p) => {
                    let element_type = p
                        .fields()
                        .get(0)
                        .expect("First parameter must exist")
                        .the_type()
                        .deref_pointer()
                        .expect("Must be pointer");
                    format!("NativeArray<{}>", h.converter.to_typespecifier_in_param(element_type))
                }
                TypePattern::NamedCallback(_) => "IntPtr".to_string(),
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

                        format!("NativeArray<{}>", h.converter.to_typespecifier_in_param(element_type))
                    }
                    TypePattern::SliceMut(p) => {
                        let element_type = p
                            .fields()
                            .get(0)
                            .expect("First parameter must exist")
                            .the_type()
                            .deref_pointer()
                            .expect("Must be pointer");

                        format!("NativeArray<{}>", h.converter.to_typespecifier_in_param(element_type))
                    }
                    _ => h.converter.to_typespecifier_in_param(param.the_type()),
                },

                _ => h.converter.to_typespecifier_in_param(param.the_type()),
            },
            CType::FnPointer(_) => "IntPtr".to_string(),
            x => h.converter.to_typespecifier_in_param(x),
        }
    }

    fn write_function_delegate_overload_helper(&self, w: &mut IndentWriter, h: &Helper, function: &Function) -> Result<(), Error> {
        let rval = h.converter.function_rval_to_csharp_typename(function);
        let name = h.converter.function_name_to_csharp_name(function, h.config.rename_symbols);

        let mut params = Vec::new();
        for (_, p) in function.signature().params().iter().enumerate() {
            let name = p.name();
            let the_type = match p.the_type() {
                CType::FnPointer(_) => "IntPtr".to_string(),
                CType::Pattern(TypePattern::NamedCallback(_)) => "IntPtr".to_string(),
                _ => h.converter.function_parameter_to_csharp_typename(p, function),
            };

            params.push(format!("{} {}", the_type, name));
        }

        indented!(
            w,
            r#"[DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "{}")]"#,
            function.name()
        )?;
        indented!(w, r#"public static extern {} {}({});"#, rval, name, params.join(", "))?;
        w.newline()?;

        Ok(())
    }
}

impl OverloadWriter for Unity {
    fn write_imports(&self, w: &mut IndentWriter, h: Helper) -> Result<(), Error> {
        if !h.config.use_unsafe.any_unsafe() {
            panic!("The Unity overload writer requires `Unsafe::UnsafeKeyword` or higher.")
        }

        if h.config.use_unsafe == Unsafe::UnsafePlatformMemCpy {
            indented!(w, r#"#if UNITY_2018_1_OR_NEWER"#)?;
            indented!(w, r#"using Unity.Collections.LowLevel.Unsafe;"#)?;
            indented!(w, r#"using Unity.Collections;"#)?;
            indented!(w, r#"#endif"#)?;
        }
        Ok(())
    }

    fn write_field_decorators(&self, w: &mut IndentWriter, h: Helper, field: &Field, strct: &CompositeType) -> Result<(), Error> {
        match field.the_type() {
            // Must not act on arrays (would panic, not supported in C#).
            CType::Array(_) => {}
            _ => {
                let the_type = h.converter.to_typespecifier_in_field(field.the_type(), field, strct);

                if the_type == "IntPtr" {
                    indented!(w, r#"#if UNITY_2018_1_OR_NEWER"#)?;
                    indented!(w, r#"[NativeDisableUnsafePtrRestriction]"#)?;
                    indented!(w, r#"#endif"#)?;
                }
            }
        }

        Ok(())
    }

    fn write_function_overload(&self, w: &mut IndentWriter, h: Helper, function: &Function) -> Result<(), Error> {
        let signature = function.signature();
        let has_overload = self.has_overloadable(signature);
        let has_error_enum = h.converter.has_ffi_error_rval(signature);

        // If there is nothing to write, don't do it
        if !has_overload || !h.config.use_unsafe.any_unsafe() {
            return Ok(());
        }

        // If we have delegates we need to write a version with IntPtr only
        if self.has_delegate(signature) {
            self.write_function_delegate_overload_helper(w, &h, function)?;
        }

        // If we _only_ have function delegates we're done, since no conversion logic will have to take place.
        if self.has_delegate(function.signature()) && !self.has_non_delegate_overloadable(signature) {
            return Ok(());
        };

        indented!(w, r#"#if UNITY_2018_1_OR_NEWER"#)?;

        let mut to_pin_name = Vec::new();
        let mut to_pin_slice_type = Vec::new();
        let mut to_invoke = Vec::new();
        let raw_name = h.converter.function_name_to_csharp_name(function, h.config.rename_symbols);
        let this_name = if has_error_enum && !has_overload {
            format!("{}_checked", raw_name)
        } else {
            raw_name.clone()
        };

        let rval = match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
            CType::Pattern(TypePattern::AsciiPointer) => "string".to_string(),
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

        if !to_pin_name.is_empty() {
            for (pin_var, slice_struct) in to_pin_name.iter().zip(to_pin_slice_type.iter()) {
                indented!(w, [_], r#"var {}_slice = new {}({});"#, pin_var, slice_struct, pin_var)?;
            }
        }

        let fn_name = h.converter.function_name_to_csharp_name(function, h.config.rename_symbols);
        let call = format!(r#"{}({});"#, fn_name, to_invoke.join(", "));
        write_function_overloaded_invoke_with_error_handling(w, function, &call)?;

        indented!(w, r#"}}"#)?;
        indented!(w, r#"#endif"#)?;

        Ok(())
    }

    fn write_service_method_overload(&self, w: &mut IndentWriter, h: Helper, _class: &Service, function: &Function, fn_pretty: &str) -> Result<(), Error> {
        if !self.has_overloadable(function.signature()) {
            return Ok(());
        }

        w.newline()?;

        indented!(w, r#"#if UNITY_2018_1_OR_NEWER"#)?;
        write_common_service_method_overload(w, h, function, fn_pretty, |h, p| self.pattern_to_native_in_signature(h, p, function.signature()))?;
        indented!(w, r#"#endif"#)?;

        Ok(())
    }

    fn write_pattern_slice_overload(&self, w: &mut IndentWriter, h: Helper, context_type_name: &str, type_string: &str) -> Result<(), Error> {
        if h.config.use_unsafe == Unsafe::UnsafePlatformMemCpy {
            // Ctor unity
            indented!(w, [_], r#"#if UNITY_2018_1_OR_NEWER"#)?;
            indented!(w, [_], r#"public {}(NativeArray<{}> handle)"#, context_type_name, type_string)?;
            indented!(w, [_], r#"{{"#)?;
            indented!(w, [_ _], r#"unsafe"#)?;
            indented!(w, [_ _], r#"{{"#)?;
            indented!(w, [_ _ _], r#"this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));"#)?;
            indented!(w, [_ _ _], r#"this.len = (ulong) handle.Length;"#)?;
            indented!(w, [_ _], r#"}}"#)?;
            indented!(w, [_], r#"}}"#)?;
            indented!(w, [_], r#"#endif"#)?;
        }

        Ok(())
    }

    fn write_pattern_slice_mut_overload(&self, _w: &mut IndentWriter, _h: Helper, _context_type_name: &str, _type_string: &str) -> Result<(), Error> {
        Ok(())
    }

    fn write_pattern_slice_unsafe_copied_fragment(&self, w: &mut IndentWriter, _h: Helper, type_string: &str) -> Result<(), Error> {
        indented!(w, [_ _ _ _ _], r#"#elif UNITY_2018_1_OR_NEWER"#)?;
        indented!(w, [_ _ _ _ _], r#"UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof({})));"#, type_string)?;
        Ok(())
    }
}
