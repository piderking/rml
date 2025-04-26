extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
mod tensor;

#[proc_macro_derive(TensorCreator)]
pub fn tensor_creator(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code based on the parsed input
    tensor::impl_tensor_creator(&input)
}
