use std::io::{self, Read};
use std::str;

fn main() {
    let mut input = vec![0u8; 1];
    let quit_str = 'q'.to_string();

    while str::from_utf8_mut(&mut input).unwrap() != &quit_str {
        io::stdin().read_exact(&mut input).unwrap();
    }
}
