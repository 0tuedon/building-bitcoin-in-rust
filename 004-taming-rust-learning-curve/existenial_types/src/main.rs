fn main() {
    println!("Hello, world!");
}
fn get_iterator() -> impl Iterator<Item = i32> {
    let mut i = 0;
    std::iter::from_fn(move || {
        i += 1;
        if i < 10 { Some(i) } else { None }
    })
}
