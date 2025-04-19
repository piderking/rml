use super::Tensor;
pub trait Tensorable {
    type Data: Clone;
    fn to_tensor(&self) -> &Tensor<Self::Data>;
}

impl<T: Clone> Tensorable for Tensor<T> {
    type Data = T;
    fn to_tensor(&self) -> &Tensor<Self::Data> {
        // slipsided
        self
    }
}

impl<T: Clone> From<&dyn Tensorable<Data = T>> for Tensor<T> {
    fn from(value: &dyn Tensorable<Data = T>) -> Tensor<T> {
        value.to_tensor().clone()
    }
}
