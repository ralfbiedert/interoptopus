use syn::Error;

use crate::types::{
    args::FfiTypeArgs,
    model::{TypeData, TypeModel},
};

impl TypeModel {
    /// Validates the type model, checking for all validation rules.
    pub fn validate(&self) -> syn::Result<()> {
        self.validate_non_empty()?;
        // Add more validation calls here as needed
        Ok(())
    }

    /// Validates that empty types are not allowed unless they are opaque structs.
    fn validate_non_empty(&self) -> syn::Result<()> {
        match &self.data {
            TypeData::Struct(struct_data) => {
                // Count non-skipped fields
                let non_skipped_fields = struct_data.fields.iter().filter(|f| !f.skip).count();

                if non_skipped_fields == 0 && !self.args.opaque && !self.args.service {
                    return Err(Error::new_spanned(&self.name, "Empty structs are not allowed unless marked as #[ffi_type(opaque)] or #[ffi_type(service)]."));
                }
            }
            TypeData::Enum(enum_data) => {
                if enum_data.variants.is_empty() {
                    return Err(Error::new_spanned(&self.name, "Enums without variants are not allowed"));
                }
            }
        }

        Ok(())
    }
}

impl FfiTypeArgs {
    /// Validates the arguments, checking for all validation rules.
    pub fn validate(&self) -> syn::Result<()> {
        self.validate_mutually_exclusive_attributes(None, None, None)?;
        Ok(())
    }

    /// Validates that opaque, transparent, and service attributes are mutually exclusive.
    fn validate_mutually_exclusive_attributes(
        &self,
        opaque_span: Option<proc_macro2::Span>,
        transparent_span: Option<proc_macro2::Span>,
        service_span: Option<proc_macro2::Span>,
    ) -> syn::Result<()> {
        let mut conflicts = Vec::new();

        if self.opaque {
            conflicts.push(("opaque", opaque_span));
        }
        if self.transparent {
            conflicts.push(("transparent", transparent_span));
        }
        if self.service {
            conflicts.push(("service", service_span));
        }

        if conflicts.len() > 1 {
            let names: Vec<&str> = conflicts.iter().map(|(name, _)| *name).collect();
            let span = conflicts[1].1.or(conflicts[0].1).unwrap_or_else(proc_macro2::Span::call_site);

            return Err(Error::new(
                span,
                format!(
                    "Cannot use {} attributes together - they are mutually exclusive",
                    match names.len() {
                        2 => format!("'{}' and '{}'", names[0], names[1]),
                        3 => format!("'{}', '{}', and '{}'", names[0], names[1], names[2]),
                        _ => names.join(", "),
                    }
                ),
            ));
        }

        Ok(())
    }
}
