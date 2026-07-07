fn factorial(n: usize) -> usize {
    let mut sum = 1;
    for x in 2..n + 1 {
        sum *= x;
    }
    sum
}

fn main() {
    let factmul = factorial(4);

    println!("The factorial is {}", factmul)
}
