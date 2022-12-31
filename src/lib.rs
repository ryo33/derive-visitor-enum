mod data_structure;
mod visitor;
mod visitor_unit;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(VisitorUnit)]
pub fn derive_visitor_unit(input: TokenStream) -> TokenStream {
    let mut tokens = TokenStream::new();
    tokens.extend(input.clone());

    let input: DeriveInput = parse_macro_input!(input);
    let structure = data_structure::extract(input);
    tokens.extend(TokenStream::from(visitor_unit::generate_visitor_unit(
        structure,
    )));

    tokens
}

#[proc_macro_derive(Visitor)]
pub fn derive_visitor(input: TokenStream) -> TokenStream {
    let mut tokens = TokenStream::new();
    tokens.extend(input.clone());

    let input: DeriveInput = parse_macro_input!(input);
    let structure = data_structure::extract(input);
    tokens.extend(TokenStream::from(visitor::generate_visitor(structure)));

    tokens
}
