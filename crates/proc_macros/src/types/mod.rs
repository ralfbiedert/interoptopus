mod args;
mod input;
mod codegen;

use proc_macro2::TokenStream;
use syn::{parse2, DeriveInput};

use args::FfiTypeArgs;
use input::ParsedInput;
use codegen::CodeGenerator;

pub fn ffi_type(attr: TokenStream, input: TokenStream) -> TokenStream {
    match ffi_type_impl(attr, input) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    }
}

fn ffi_type_impl(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let args = FfiTypeArgs::from_token_stream(attr)?;
    let derive_input: DeriveInput = parse2(input)?;

    let parsed_input = ParsedInput::from_derive_input(derive_input, args)?;
    let generator = CodeGenerator::new(&parsed_input);
    let result = generator.generate();

    // Debug output if requested
    if parsed_input.args.debug {
        println!("Raw generated tokens:\n{}", result);
        eprintln!("Raw generated tokens:\n{}", result);
    }

    Ok(result)
}
