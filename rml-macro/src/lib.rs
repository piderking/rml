extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};
use quote::{quote, ToTokens};


#[proc_macro_derive(TensorCreator)]
pub fn tensor_creator(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code based on the parsed input
    impl_tensor_creator(&input)
}

fn impl_tensor_creator(input: &DeriveInput) -> TokenStream {
    // Extract the struct name
    let n =  &input.ident;

    
    let name = Ident::new(&format!{"{}Tensor", &input.ident}, input.ident.span());
    
    // Generate the method to count fields

    // Convert the generated code into a TokenStream
    TokenStream::from( match &input.data {
        Data::Enum(data) =>  {
            let v = data.variants.iter().filter_map(|variant| {
            match &variant.fields {
                Fields::Unnamed(_) => Option::Some({
                    let ident = &variant.ident;
                    let p : Vec<Tokens> = variant.fields.iter().map(|l|{
                        let d = l.to_token_stream();
                        quote! [ Tensor<#d>]
                    }).collect();
                    quote![#ident ( #(#p),* )]
                    

                    
                    
                }),
                _ => Option::None,
            }
            
            }).collect::<Vec<Tokens>>();

            quote! {
                use rml_data::tensor::Tensor;

                #[derive(Debug)]
                enum #name {
                    #(#v),*,
                    
                }
            }
        },
        _ => quote![]
    })
}
