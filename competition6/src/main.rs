fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "C");
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
/// A Simple Sequence
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        for i in (1..(n+1)).rev() {
            answer.push_str(&i.to_string());
            answer.push(' ');
        }
        answer.push('\n');
    }
    print!("{answer}");
}
/// Simply Sitting on Chairs
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let p = scanner.read_vec_usize();
        let mut count = 0;
        for i in 0..n {
            if p[i] <= i + 1 {
                count = count + 1;
            }
        }
        answer.push_str(&count.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// A Simple GCD Problem (Easy Version)
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let b = scanner.read_vec_usize();
        let mut gcd_arr: Vec<usize> = vec![0; n + 1];
        for i in 1..(n) {
            gcd_arr[i] = gcd(a[i - 1], a[i]);
        }
        let mut count = 0;
        for i in 0..n {
            if a[i] != gcd_arr[i] && a[i] != gcd_arr[i + 1] && lcm(gcd_arr[i], gcd_arr[i + 1]) != a[i] {
                count = count + 1;
            }
        }
        answer.push_str(&count.to_string());
        answer.push('\n');

    }
    print!("{answer}");
}

/// A Simple GCD Problem (Hard Version)
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let b = scanner.read_vec_usize();
        let mut gcd_arr: Vec<usize> = vec![0; n + 1];
        for i in 1..(n) {
            gcd_arr[i] = gcd(a[i - 1], a[i]);
        }
        let mut count = 0;
        for i in 0..n {
            if a[i] != gcd_arr[i] && a[i] != gcd_arr[i + 1] && lcm(gcd_arr[i], gcd_arr[i + 1]) != a[i] {
                count = count + 1;
            }
        }
        answer.push_str(&count.to_string());
        answer.push('\n');

    }
    print!("{answer}");
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
#[allow(dead_code)]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
#[allow(dead_code)]
fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}
// Last Line comment marker