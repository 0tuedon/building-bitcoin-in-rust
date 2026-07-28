// borrowed
fn main() {
    give_me_a_ref();
}

fn give_me_a_ref<'a>() -> String {
    let temp = String::from("Just testing this");
    // &temp cannot return it because it's lifetime expired with the function
    temp
}
