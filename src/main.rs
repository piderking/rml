pub fn merge(data: Vec<Vec<i32>>) -> Vec<i32> {
    let mut d = data.iter().flatten().map(|f| *f).collect::<Vec<i32>>();
    d.sort();
    d
}
pub fn main() {
    let d = vec![vec![1, 2, 5], vec![2, 3, 6]];
    println!("{:?}", merge(d));
}
