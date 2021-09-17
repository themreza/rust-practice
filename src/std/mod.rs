use std::io::{stdin, stdout, Write};

pub fn prompt(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid input.");
    return input.replace('\n', "");
}