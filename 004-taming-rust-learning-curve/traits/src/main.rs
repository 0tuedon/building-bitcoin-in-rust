trait Quack {
    fn quack(&self);
}
trait Vehicle {
    type Energy;
    const WHEELS: u8;

    fn energy_source(&self) -> Self::Energy;

    fn print_wheels() {
        println!("This vehicle has {} wheels", Self::WHEELS);
    }
}
struct FormalDuck {
    name: String,
}
struct FixedArray<T, const N: usize> {
    data: [T; N],
}
impl FormalDuck {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Quack for FormalDuck {
    fn quack(&self) {
        println!("Good Evening I am {} , quack quack", self.name);
    }
}

// or we could do like a fn tied to the Quack

fn duck_says<T: Quack>(quacker: T) {
    quacker.quack();
}

fn no_param<T: Sized>(_: T) {}
fn main() {
    let formal = FormalDuck::new("Tuedon".to_string());
    duck_says(formal);
    let my_str = "Hello Tuedon";
    no_param(my_str);
}
