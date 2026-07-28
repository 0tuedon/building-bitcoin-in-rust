// Show how long we want to references to live as long as the other
struct Holder<'a> {
    reference: &'a str,
}

fn main() {
    // Reborrowing
    let test_borrowing = "Hello World";
    let hell = &test_borrowing[..4];
    println!("{} {}", test_borrowing, hell);
    let holder = Holder {
        reference: &test_borrowing,
    };
    println!("{}", holder.reference);
}

fn max_ref<'a>(left: &'a i32, right: &'a i32) -> &'a i32 {
    if *left < *right { right } else { left }
}
