/// Parse a `TokenStream` for an attribute into the corresponding `darling` struct.
macro_rules! darling_parse {
    ($t:ty, $args:expr) => {{
        use darling::ast::NestedMeta;
        use darling::Error;
        use darling::FromMeta;
        use proc_macro2::TokenStream;

        let attr_args = match NestedMeta::parse_meta_list($args.into()) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(Error::from(e).write_errors());
            }
        };

        let args = match <$t>::from_list(&attr_args) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };

        args
    }};
}

pub(crate) use darling_parse;
