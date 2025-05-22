#[derive(Clone)]
pub struct TString(String);

impl From<&str> for TString{
    fn from(v: &str) -> Self {
        Self::new(v.into())
    }
}
impl From<String> for TString{
    fn from(v: String) -> Self {
        Self::new(v)
    }
}

impl Into<String> for TString {
    fn into(self) -> String {
        self.0
    }
}


impl TString {
    pub fn new(s: String) -> Self {
        TString(s)
    }
}