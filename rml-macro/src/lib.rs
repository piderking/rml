extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
mod tensor;
mod debug;

#[proc_macro_derive(TensorCreator)]
pub fn tensor_creator(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code based on the parsed input
    tensor::impl_tensor_creator(&input)
}

#[proc_macro_derive(TensorDebug)]
pub fn tensor_debug(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code based on the parsed input
    debug::impl_debug(&input)
}
