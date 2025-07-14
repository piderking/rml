use std::io::Error;




#[derive(Debug, Default)]
pub enum View {
    #[default]
    Home,
}
impl View {
    pub fn from_str(str: String) -> Option<View> {
        match str.to_ascii_lowercase().trim(){
            "home" => Option::Some(View::Home),
            _ => Option::None
        }
    }
}