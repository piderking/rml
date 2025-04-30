use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::{ToTokens, quote};
use syn::{Data, ExprArray, Fields, Ident, LitStr};
use syn::{parse_macro_input, parse::Parse, Token, LitInt};


pub(crate) struct MacroInput {
    data: Vec<ExprArray>
}


impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(MacroInput {
            data:input.parse()?, // parse for ExprArray
        })
    }
}
pub(crate) fn tensor_constructor(input: MacroInput) -> TokenStream {
   
}
