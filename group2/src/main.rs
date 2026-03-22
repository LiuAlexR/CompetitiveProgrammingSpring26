fn main() {
    let scanner: Scanner = Scanner::new();
}
/// Diameter is n/2 for cycle graph
/// m * 2 / n is degree of every vertex assuming optimality
/// Diameter + degree of every node = number of nodes
fn p1(scanner: &mut Scanner) {
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let nmk = scanner.read_vec_usize();
        let n = nmk[0];
        let m = nmk[1];
        let k = nmk[2];
        if m < n -1 {
            answer.push_str("NO\n");
            continue;
        }
        if m > n * (n-1) / 2 {
            answer.push_str("NO\n");
            continue;
        }
        if k > n {
            answer.push_str("NO\n");
            continue;
        }
        if k < 2 {
            answer.push_str("NO\n");
            continue;
        }
    }
}
/// The scanner. It allows for readings of generic numbers, strings, and vectors of numbers
#[allow(unused_imports)]
use std::{collections::{HashMap, VecDeque}, io::Stdin, str::FromStr};
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