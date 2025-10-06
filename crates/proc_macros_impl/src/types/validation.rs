use syn::Error;

use crate::forbidden::is_forbidden_name;
use crate::types::{
    args::FfiTypeArgs,
    model::{TypeData, TypeModel},
};

impl TypeModel {
    /// Validates the type model, checking for all validation rules.
    pub fn validate(&self) -> syn::Result<()> {
        self.validate_non_empty()?;
        self.validate_forbidden_names()?;
        Ok(())
    }

    /// Validates that empty types are not allowed unless they are opaque structs.
    fn validate_non_empty(&self) -> syn::Result<()> {
        match &self.data {
            TypeData::Struct(struct_data) => {
                // Count non-skipped fields
                let non_skipped_fields = struct_data.fields.iter().filter(|f| !f.skip).count();

                if non_skipped_fields == 0 && !self.args.opaque && !self.args.service {
                    return Err(Error::new_spanned(&self.name, "Empty structs are not allowed unless marked as #[ffi(opaque)] or #[ffi(service)]."));
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

    /// Validates that no forbidden names are used for types, fields, or variants.
    fn validate_forbidden_names(&self) -> syn::Result<()> {
        // Check type name
        if is_forbidden_name(self.name.to_string()) {
            return Err(Error::new_spanned(&self.name, format!("Using the name '{}' can cause conflicts in generated code.", self.name)));
        }

        // Check fields and variants based on data type
        match &self.data {
            TypeData::Struct(struct_data) => {
                for field in &struct_data.fields {
                    if let Some(field_name) = &field.name
                        && is_forbidden_name(field_name.to_string())
                    {
                        return Err(Error::new_spanned(field_name, format!("Using the name '{field_name}' can cause conflicts in generated code.")));
                    }
                }
            }
            TypeData::Enum(enum_data) => {
                for variant in &enum_data.variants {
                    if is_forbidden_name(variant.name.to_string()) {
                        return Err(Error::new_spanned(&variant.name, format!("Using the name '{}' can cause conflicts in generated code.", variant.name)));
                    }
                }
            }
        }

        Ok(())
    }
}

impl FfiTypeArgs {
    /// Validates the arguments, checking for all validation rules.
    pub fn validate(&self) -> syn::Result<()> {
        self.validate_mutually_exclusive_attributes()?;
        Ok(())
    }

    /// Validates that opaque, transparent, and service attributes are mutually exclusive.
    fn validate_mutually_exclusive_attributes(&self) -> syn::Result<()> {
        let mut conflicts = Vec::new();

        if self.opaque {
            conflicts.push(("opaque", &self.opaque_token));
        }
        if self.transparent {
            conflicts.push(("transparent", &self.transparent_token));
        }
        if self.service {
            conflicts.push(("service", &self.service_token));
        }

        if conflicts.len() > 1 {
            let names: Vec<&str> = conflicts.iter().map(|(name, _)| *name).collect();
            // Use the second conflict's token for the error location, or the first as fallback
            let error_token = conflicts[1].1.as_ref().or(conflicts[0].1.as_ref());

            let message = format!(
                "Cannot use {} attributes together - they are mutually exclusive",
                match names.len() {
                    2 => format!("'{}' and '{}'", names[0], names[1]),
                    3 => format!("'{}', '{}', and '{}'", names[0], names[1], names[2]),
                    _ => names.join(", "),
                }
            );

            return if let Some(token) = error_token {
                Err(Error::new_spanned(token, message))
            } else {
                // Fallback if no tokens are available (shouldn't happen normally)
                Err(Error::new(proc_macro2::Span::call_site(), message))
            };
        }

        Ok(())
    }
}
