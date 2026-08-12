use ::std::iter::Iterator;

struct ConstantNumber {
    number: usize,
}

impl Iterator for ConstantNumber {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.number)
    }
}
fn main() {
    let mut constant = ConstantNumber { number: 5 };
    let numbers: Vec<_> = constant.take(10).collect();
    println!("{:?}", numbers);
}
