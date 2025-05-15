use super::traits::tensor::TensorBound;

pub struct StringTensor {
    data: Vec<String>,
}

impl StringTensor {
    pub fn new(data: &[&str]) -> Self {
        StringTensor::from(data.iter().map(|f| String::from(*f)).collect())
    }
    pub fn from(data: Vec<String>) -> Self {
        StringTensor { data }
    }
}


impl TensorBound for StringTensor {
    type inner = String;

    fn apply<F: Fn(&String) -> Self::inner>(self, f: F) -> Self {
        Self::from(self.data.iter().map(f).collect())
    }

    fn mutate<F: Fn(&Self::inner) -> Self::inner>(&mut self, f: F) -> () {
        for t in self.data.iter_mut() {
            *t = f(t)
        }
    }
}
