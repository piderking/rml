use std::fmt::{Display, Error, Formatter};

use std::fmt::Debug;

use crate::tensor::traits::tensor::TensorBound;

#[derive(Debug, Clone)]
pub struct Deep<T>
where
    T: TensorBound,
{
    #[allow(dead_code)]
    pub(crate) data: Vec<T::inner>,
}


impl<T> Deep<T>
where
    T: TensorBound,
{
    pub fn new(data: Vec<T::inner>) -> Self{
        Deep { data }
    }

    pub fn get(&self, i: usize) -> Option<&T::inner> {
        self.data.get(i)
    } 
}

// Display All of Body
impl<T: TensorBound> Display for Deep<T> 
where T::inner: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let elements = self.data
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", elements)
    }
}
