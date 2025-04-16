

use std::rc::Rc;

#[derive(Debug)]
pub struct Tensor <T>{
    data: Rc<Box<[T]>>,
}
trait Tensorable {

}
impl <T> Tensorable for Tensor<T> {
}

#[cfg(test)]
mod tests {
    use super::*;

}
