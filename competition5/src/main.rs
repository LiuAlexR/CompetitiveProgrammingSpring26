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
        "C" => p_c(scanner),
        "D" => p_d(scanner),

        _ => (),
    }
}
/// Antimedian Deletion
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let p = scanner.read_vec_usize();
        if n == 1 {
            answer.push_str(&"1\n".to_string());
            continue;
        } else if n == 2 {
            answer.push_str(&"2 2\n".to_string());
            continue;
        }
        for _ in 0..n {
            answer.push_str(&"2".to_string());
            answer.push(' ');
        }
        answer.push('\n');
    }
    print!("{answer}");
}
/// Mickey Mouse Constructive
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let xy = scanner.read_vec_generic::<isize>();
        let x = xy[0];
        let y = xy[1];
        let mut s = x-y;
        if s == 0 {
            answer.push_str(&"1\n".to_string());
        } else {
            if s < 0 {
                s = -s;
            }
            let mut total = 1;
            let mut the_mod = 676767677;
            let mut s_mut = s;
            let mut cur_prime_count = 0;
            let mut cur_prime = 2;
            while s_mut % cur_prime == 0 {
                s_mut = s_mut / cur_prime;
                cur_prime_count += 1;
            }
            total = (total * (cur_prime_count + 1)) % the_mod;
            cur_prime = 3;
            cur_prime_count = 0;
            while s_mut > 1 {
                while s_mut % cur_prime == 0 {
                    s_mut = s_mut / cur_prime;
                    cur_prime_count += 1;
                }
                total = (total * (cur_prime_count + 1)) % the_mod;
                cur_prime = cur_prime + 2;
                cur_prime_count = 0;
            }
            answer.push_str(&total.to_string());
            answer.push('\n');
        }
        for _ in 0..x {
            answer.push_str(&"1 ".to_string());
        }
        for _ in 0..y {
            answer.push_str(&"-1 ".to_string());
        }
        answer.push('\n');
    }
    print!("{answer}");
}
/// Longest Divisors Interval
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {

    }
    print!("{answer}");
}
/// Emordnilap
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let mut res: usize = n;
        let the_mod = 1000000007;
        res = (res * (res - 1)) % the_mod;
        for i in 1..(n + 1) {
            res = res * i;
            res = res % the_mod;
        }
        answer.push_str(&res.to_string());
        answer.push('\n');
        print!("{answer}");
    }
}
use std::collections::HashSet;
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
    pub fn read_vec_i64(&mut self) -> Vec<i64> {
        return self.read_vec_generic::<i64>();
    }
    
}
// Last Line comment marker