mod loader;
#[cfg(test)]
mod lib {
    use super::*;

    #[test]
    fn unit_testname() {
        assert_eq!(1, 1);
    }
    fn fail() {
        assert_eq!(0, 1);
    }
}
