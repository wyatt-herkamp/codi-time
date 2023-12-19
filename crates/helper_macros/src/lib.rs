use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
mod database_helpers;

#[proc_macro_derive(DatabaseHelpers, attributes(column))]
pub fn database_helpers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match database_helpers::expand(input) {
        Ok(ok) => ok.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
