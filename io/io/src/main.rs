use std::io::{Stdout, Write, stdout};

fn main() {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    // let x: usize = buffer.strip_suffix().parse().unwrap();
    let mut stdout: Stdout = std::io::stdout();
    for x in 0..2 {
        stdin.read_line(&mut buffer).unwrap();
        stdout.write(buffer.as_bytes());
        buffer.clear();
    // let x: usize = buffer.strip_suffix().parse().unwrap();
    }
}
