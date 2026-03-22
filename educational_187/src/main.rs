fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "B");
}
/// The main problem solution archive
#[allow(dead_code, unused_variables)]
fn problems(scanner: &mut Scanner, num: &str) {
    match num {
        "A" => p_a(scanner),
        "B" => p_b(scanner),
        // "C" => p_c(scanner),
        // "D" => p_d(scanner),
        // "E" => p_e(scanner),
        // "F" => p_f(scanner),
        // "G" => p_g(scanner),
        // "H" => p_h(scanner),
        // "I" => p_i(scanner),
        // "J" => p_j(scanner),
        _ => (),
    }
}
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let nmd = scanner.read_vec_usize();
        let n = nmd[0];
        let m = nmd[1];
        let d = nmd[2];
        let boxes_per_tower = d / m + 1;
        let num_of_towers = match n % boxes_per_tower {
            0 => n / boxes_per_tower,
            _ => n / boxes_per_tower + 1,
        };
        answer.push_str(&num_of_towers.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let x: Vec<u32> = scanner.read_string().as_bytes().iter().map(|x| (x-b'0') as u32).collect();
        let mut current_sum: u32 = x.iter().sum();
        
        if current_sum <= 9 {
            answer.push_str("0\n");
            continue;
        }

        let mut gains = Vec::new();
        for (i, &d) in x.iter().enumerate() {
            if i == 0 {
                if d > 1 { gains.push(d - 1); }
            } else {
                if d > 0 { gains.push(d); }
            }
        }

        gains.sort_by(|a, b| b.cmp(a));

        let mut moves = 0;
        let mut i = 0;
        while current_sum > 9 && i < gains.len() {
            current_sum -= gains[i];
            moves += 1;
            i += 1;
        }

        answer.push_str(&format!("{}\n", moves));
    }
    print!("{}", answer);
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