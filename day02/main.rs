use std::io;

fn match_outcome(s: &str) -> i32 {
    let mut outcome = 0;
    match s {
        "C X" | "B Z" | "A Y" => outcome = 6,
        "A Z" | "C Y" | "B X" => outcome = 0,
        _ => outcome = 3,
    };
    outcome
}

fn match_value(s: &str) -> i32 {
    let foo: Vec<char> = s.chars().rev().take(1).collect();
    return 0;
}

fn main() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let value = match_outcome(&input.clone());
                println!("{value} {input}", value = value, input = input);
                input.clear();
            }
            Err(error) => {
                println!("error: {error}");
                break;
            }
        }
    }
}
