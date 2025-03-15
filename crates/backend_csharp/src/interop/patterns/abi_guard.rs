use crate::converter::function_name_to_csharp_name;
use crate::{FunctionNameFlavor, Interop};
use interoptopus::backend::IndentWriter;
use interoptopus::lang::CType;
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::api_guard::inventory_hash;
use interoptopus::{Error, indented};

pub fn write_abi_guard(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_abi_guard")?;

    indented!(w, r"static {}()", i.class)?;
    indented!(w, r"{{")?;

    // Check if there is a API version marker for us to write
    if let Some(api_guard) = i
        .inventory
        .functions()
        .iter()
        .find(|x| matches!(x.signature().rval(), CType::Pattern(TypePattern::APIVersion)))
    {
        let version = inventory_hash(&i.inventory);
        let flavor = if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        };
        let fn_call = function_name_to_csharp_name(api_guard, flavor);
        indented!(w, [()], r"var api_version = {}.{}();", i.class, fn_call)?;
        indented!(w, [()], r"if (api_version != {}ul)", version)?;
        indented!(w, [()], r"{{")?;
        indented!(
            w,
            [()()],
            r#"throw new TypeLoadException($"API reports hash {{api_version}} which differs from hash in bindings ({}). You probably forgot to update / copy either the bindings or the library.");"#,
            version
        )?;
        indented!(w, [()], r"}}")?;
    }

    indented!(w, r"}}")?;

    Ok(())
}
