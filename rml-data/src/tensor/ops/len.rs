use crate::tensor::len::TensorSize;

impl PartialEq for TensorSize {
    fn eq(&self, other: &Self) -> bool {
        // if lenths are equal
        if self.size.len().eq(&other.size.len()) {
            (self
                .size
                .iter()
                .zip(other.size.iter())
                .filter(|(v1, v2)| {
                    print!("{:} != {:}", v1, v2);
                    v1 != v2
                })
                .count())
                <= 0
        } else {
            false
        }
    }
}
