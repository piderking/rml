use proc_macro::TokenStream;

use quote::{ToTokens, quote};
use syn::{Data, DeriveInput};

pub(crate) fn impl_debug(input: &DeriveInput) -> TokenStream {
    // Extract the struct name

    let _name = &input.ident;

    // Generate the method to count fields

    // Convert the generated code into a TokenStream
    TokenStream::from(match &input.data {
        Data::Enum(data) => {
            let v = data
                .variants
                .iter()
                .map(|variant| variant.to_token_stream().to_string())
                .collect::<Vec<String>>()
                .join(", ");

            println!("{:?}", v);
            quote![]
        }
        _ => quote![],
    })
}
