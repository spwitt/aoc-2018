use std::io::{self, Read};

pub fn stdin_to_line_vec() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.lines().map(|s| s.to_string()).collect()
}
