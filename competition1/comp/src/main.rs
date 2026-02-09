use std::{io::Stdin, str::FromStr};

fn main() {
    let mut scanner = Scanner::new();
    p_a(&mut scanner);
}
fn p_a(scanner: &mut Scanner) -> () {
    let t: i32 = scanner.read_i32();
    let mut ans = String::new();
    for i in 0..t {
        let nw = scanner.read_vec_generic::<i32>();
        let n = nw[0];
        let w = nw[1];
        ans.push_str(&(n - n/w).to_string());
        ans.push('\n');
    }
    print!("{ans}");
}
fn p_b(scanner: &mut Scanner) -> () {
    let t: i32 = scanner.read_i32();
    let mut ans = String::new();
    for i in 0..t {
        let nxy = scanner.read_vec_generic::<i32>();
        let vals = scanner.read_vec_generic::<i32>();
        let mut max_index = 0;
        for i in 0..vals.len() {
            if vals[i] %
        }
        ans.push('\n');
    }
    print!("{ans}");
}
#[allow(dead_code)]
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
}