use crate::converter::function_name;
use crate::{FunctionNameFlavor, Interop};
use interoptopus::backend::IndentWriter;
use interoptopus::lang::Type;
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::api_guard::ApiHash;
use interoptopus::{Error, indented};

#[allow(clippy::literal_string_with_formatting_args)]
pub fn write_abi_guard(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_abi_guard")?;

    indented!(w, r"static {}()", i.class)?;
    indented!(w, r"{{")?;

    // Check if there is a API version marker for us to write
    if let Some(api_guard) = i
        .inventory
        .functions()
        .iter()
        .find(|x| matches!(x.signature().rval(), Type::Pattern(TypePattern::APIVersion)))
    {
        let hash = ApiHash::from(&i.inventory);
        let hash_hex = hash.hash_hex();
        let flavor = FunctionNameFlavor::RawFFIName;
        let fn_call = function_name(api_guard, flavor);
        indented!(w, [()], r"var api_version = {}.{}();", i.class, fn_call)?;
        indented!(w, [()], r"if (api_version != 0x{hash_hex})")?;
        indented!(w, [()], r"{{")?;
        indented!(
            w,
            [()()],
            r#"throw new TypeLoadException($"API reports hash 0x{{api_version:X}} which differs from hash in bindings (0x{hash_hex}). You probably forgot to update / copy either the bindings or the library.");"#,
        )?;
        indented!(w, [()], r"}}")?;
    }

    indented!(w, r"}}")?;

    Ok(())
}
