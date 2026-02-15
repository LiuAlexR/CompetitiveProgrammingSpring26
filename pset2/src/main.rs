fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "A");
}
/// The main problem solution archive
#[allow(dead_code, unused_variables)]
fn problems(scanner: &mut Scanner, num: &str) {
    match num {
        "A" => p_a(scanner),
        "B" => p_b(scanner),
        "C" => p_c(scanner),
        "D" => p_d(scanner),
        "E" => p_e(scanner),
        "F" => p_f(scanner),
        "G" => p_g(scanner),
        "H" => p_h(scanner),
        "J" => p_j(scanner),
        _ => (),
    }
}
fn p_a(scanner: &mut Scanner) {
    let t: usize = scanner.read_usize();
    for _ in 0..t {
        let n: usize = scanner.read_usize();
        let a: Vec<usize> = scanner.read_vec_usize();
        let mut solution: Vec<usize> = Vec::with_capacity(n);
        let mut max: usize = 0;
        for i in 0..n {
            if a[i] > max {
                max = a[i];
            }
            solution[i] = 
        }
    }
}
fn p_b(scanner: &mut Scanner) {
    
}
fn p_c(scanner: &mut Scanner) {
    
}
fn p_d(scanner: &mut Scanner) {
    
}
fn p_e(scanner: &mut Scanner) {
    
}
fn p_f(scanner: &mut Scanner) {
    
}
fn p_g(scanner: &mut Scanner) {
    
}
fn p_h(scanner: &mut Scanner) {
    
}
fn p_i(scanner: &mut Scanner) {
    
}
fn p_j(scanner: &mut Scanner) {
    
}
/// The scanner. It allows for readings of generic numbers, strings, and vectors of numbers
use std::{io::Stdin, str::FromStr};
struct Scanner {
    reader: Stdin,
    buffer: String,
}
#[allow(dead_code)]
impl Scanner {
    pub fn new() -> Self {
        Self {
            reader: std::io::stdin(),
            buffer: String::new(),
        }
    }
    fn read_line(&mut self) -> usize {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
            Ok(val) => {
                val
            },
            Err(_) => panic!("Buffer error"),
        }
    }
    pub fn read_string(&mut self) -> String {
        self.read_line();
        self.buffer.retain(|c| !(c == '\n' || c == '\r'));
        return self.buffer.clone();
    }
    pub fn read_i32 (&mut self) -> i32 {
        return self.read_number_generic::<i32>();
    }
    pub fn read_isize (&mut self) -> isize {
        return self.read_number_generic::<isize>();
    }
    pub fn read_usize (&mut self) -> usize {
        return self.read_number_generic::<usize>();
    }
    pub fn read_number_generic<T: FromStr> (&mut self) -> T {
        self.read_line();
        self.buffer.retain(|c| !c.is_whitespace());
        let val = self.buffer.parse::<T>();
        match val {
            Ok(value) => {
                return value;
            },
            Err(_) => panic!("Parsing Error"),
        }
    }
    pub fn read_vec_generic<T: FromStr> (&mut self) -> Vec<T> {
        self.read_line();
        let try_to_split = self.buffer.split_whitespace().map(|val| match val.parse::<T>() {
            Ok(val) => val,
            Err(_) => panic!("Parsing Error")
        }).collect();
        return try_to_split;
    }
    pub fn read_vec_usize(&mut self) -> Vec<usize> {
        return self.read_vec_generic::<usize>();
    }
}
// Last Line comment marker