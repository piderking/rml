use rml_macro::TensorCreator;

#[derive(TensorCreator)]
enum Example {
    Col1(f32),
    Col2(i32),
    Col3(String)

}

fn main() {
}