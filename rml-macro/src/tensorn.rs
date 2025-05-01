use proc_macro::TokenStream as TokenExport;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::ParseStream;
use syn::token::{self, Token};
use syn::{Data, Expr, ExprArray, Fields, Ident, LitStr, bracketed};
use syn::{LitInt, Token, parse::Parse, parse_macro_input};

pub(crate) struct Slice {
    bracket_token: token::Bracket,
    content: TokenStream,
}

impl Parse for Slice {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let col;

        let bracket_token = bracketed!(col in input);

        match col.parse::<Expr>() {
            Ok(n) => match n {
                Expr::Array(e) => {
                    //let e = e.to_token_stream().into_iter().map(|d| );
                    todo!()
                }
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
        //if col.lookahead1().peek(token::Bracket)
        /*
        Ok(Slice {
            bracket_token: bracketed!(col in input),
            content: col.parse()?,
        }) */
        todo!()
    }
}
pub(crate) fn tensor_constructor(input: ParseStream) -> TokenExport {}
