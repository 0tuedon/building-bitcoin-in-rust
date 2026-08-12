use std::thread;

fn hi() {
    println!("Hello world")
}
fn main() {
    let x = 42;
    let u = || println!("{x}");
    u();

    let y = |x: i32| x * 2;

    let z = |(x, y): (i32, i32)| x * y;

    let numbers = vec![1, 2, 3, 4, 5];
    // new thread with a move closure
    let handle = thread::spawn(move || {
        let sum: i32 = numbers.iter().sum();
        println!("The Sum is  ; {}", sum);
    });

    handle.join().unwrap();
}
