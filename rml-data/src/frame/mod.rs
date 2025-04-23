pub trait Frame {}

#[cfg(test)]
mod tests {
    use super::*;

    enum T {
        COL1(i32),
    }
    enum TD {
        COL1(Tensor<i32>),
    }
    struct DataFrame {
        data: Vec<TD>,
    }
}
