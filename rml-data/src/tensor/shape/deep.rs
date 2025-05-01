use std::fmt::{Display, Error, Formatter};

use std::fmt::Debug;

use crate::tensor::traits::tensor::TensorBound;

#[derive(Debug, Clone)]
pub struct Deep<T>
where
    T: TensorBound,
{
    #[allow(dead_code)]
    pub(crate) data: Vec<T>,
}

impl<T> Deep<T>
where
    T: TensorBound,
{
    pub fn new(data: Vec<T>) {
        Deep { data }
    }
}

// Display All of Body
impl<T: TensorBound> Display for Deep<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let data = self.data.borrow();
        let elements = data
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", elements)
    }
}
