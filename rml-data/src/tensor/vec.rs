use super::{tensor::Tensor, tensorable::Tensorable};

impl<'a, T: Tensorable<'a>> From<Vec<T>> for Tensor<'a, T>
where
    &'a T: 'a,
{
    fn from(value: Vec<T>) -> Self {
        Tensor::new(value)
    }
}

impl<'a, T: Tensorable<'a>> From<Tensor<'a, T>> for Vec<T> {
    fn from(value: Tensor<'a, T>) -> Self {
        value.into()
    }
}
