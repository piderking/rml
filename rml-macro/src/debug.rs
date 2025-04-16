use proc_macro::TokenStream;

use syn::{Data, DeriveInput};
use quote::{quote, ToTokens};


pub (crate) fn impl_debug(input: &DeriveInput) -> TokenStream {
    // Extract the struct name
    
    let name =  &input.ident;
    
    // Generate the method to count fields

    // Convert the generated code into a TokenStream
    TokenStream::from( match &input.data {
        Data::Enum(data) =>  {
            let v = data.variants.iter().map(|variant| variant.to_token_stream().to_string()).collect::<Vec<String>>().join(", ");
            
            println!("{:?}", v);
            quote! {                
                impl #name {
                    fn list() -> String {
                        String::from(#v)
                    }
                }

            }
        },
        _ => quote![]
    })
}
