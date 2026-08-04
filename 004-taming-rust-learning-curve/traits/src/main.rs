trait Quack {
    fn quack(&self);
}

struct FormalDuck {
    name: String,
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
