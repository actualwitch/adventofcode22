use std::io;

fn main() {
    let mut input = String::new();
    let mut last_input_was_empty = false;
    let mut outputs = Vec::new();
    let mut partial_sum = 0;
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                input = input.trim().to_string();

                if last_input_was_empty && input.is_empty() {
                    break;
                }

                if input.is_empty() {
                    last_input_was_empty = true;
                    outputs.push(partial_sum);
                    partial_sum = 0;
                    continue;
                }
                last_input_was_empty = false;

                let calories: u32 = input.parse().unwrap();
                partial_sum += calories;
                input.clear();
            }
            Err(error) => {
                println!("error: {error}");
                break;
            }
        }
    }
    outputs.sort_by(|a, b| b.cmp(a));
    println!("{}", outputs[0]);
    println!("{}", outputs[0] + outputs[1] + outputs[2]);
}
