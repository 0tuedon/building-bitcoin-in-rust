use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <op> <text>", args[0]);
        exit(1);
    }
    let op = &args[1];
    let text = &args[2];
    println!("op: {op} text: {text}");

    let res = match op.as_str() {
        "uppercase" => text.to_uppercase(),
        "reverse" => text.chars().rev().collect::<String>(),
        "invert" => text.chars().map(invert).collect::<String>(),
        "leet" => text
            .chars()
            .map(|c| match c {
                'a' | 'A' => '4',
                'e' | 'E' => '3',
                'i' | 'I' => '1',
                '0' | 'O' => '0',
                's' | 'S' => '5',
                't' | 'T' => '7',
                _ => c,
            })
            .collect::<String>(),
        "no-spaces" => text
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        "acronym" => text
            .split_whitespace()
            .map(|word| word.chars().next().unwrap())
            .collect::<String>()
            .to_uppercase(),
        _ => {
            eprintln!("Inavalid Operation: {}", op);
            exit(1);
        }
    };

    println!("{}", res);
}

fn invert(c: char) -> String {
    if c.is_lowercase() {
        c.to_uppercase().to_string()
    } else {
        c.to_lowercase().to_string()
    }
}
