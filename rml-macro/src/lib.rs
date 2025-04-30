extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, LitStr};
use tensorn::MacroInput;
mod tensor;
mod tensorn;

#[proc_macro_derive(TensorCreator)]
pub fn tensor_creator(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code based on the parsed input
    tensor::impl_tensor_creator(&input)
}

#[proc_macro]
pub fn tensor(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as MacroInput);

    // Generate code based on the parsed input
    tensorn::tensor_constructor(input)
}