extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
//use tensorn::Slice;
mod tensor;
mod tensorn;
//mod tensorn;

#[proc_macro_derive(TensorCreator)]
pub fn tensor_creator(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code based on the parsed input
    tensor::impl_tensor_creator(&input)
}

#[proc_macro_derive(DataFrame)]
pub fn df_creator(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code based on the parsed input
    tensorn::impl_df_creator(&input)
}

/*
#[proc_macro]
pub fn tensor(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as Slice);

    // Generate code based on the parsed input
    tensorn::tensor_constructor(input)
}
*/
