use std::fmt::Debug;
use std::fmt::Display;
#[derive(Clone)]
pub struct TString(String);

impl From<&str> for TString {
    fn from(v: &str) -> Self {
        Self::new(v.into())
    }
}
impl From<String> for TString {
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

#[macro_export]
macro_rules! tstring {
    [$e:expr] => {
        crate::tensor::types::tstring::TString::new($e.to_string())
    };
}

impl Display for TString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <TString as Into<String>>::into(self.clone()))
    }
}
impl Debug for TString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
