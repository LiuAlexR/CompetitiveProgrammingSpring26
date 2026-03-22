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
/// Passing the Ball
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_i32();
        let s = scanner.read_string();
        let mut num_r = 0;
        for i in 0..s.len() {
            if &s[i..i+1] == "R" {
                num_r = num_r + 1;
            } else {
                break;
            }
        }
        let num_people = std::cmp::min(num_r + 1, n + 1);
        answer.push_str(&num_people.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Right Maximum
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut counter = 1;
        let mut max_val = a[0];
        max_val = a[0];
        for i in 1..n {
            if a[i] >= max_val {
                counter = counter + 1;
                max_val = a[i];
            }
        }

        answer.push_str(&counter.to_string());
        answer.push('\n');
    }
    print!("{answer}");

}
/// Spring
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let abcm = scanner.read_vec_usize();
        let a = abcm[0];
        let b = abcm[1];
        let c = abcm[2];
        let m = abcm[3];
        // let mut counter = 0;
        // counter = m / a;
        // counter = counter + (m / b);
        // counter = counter + (m / c);
        // counter = counter + (m / (a * b * c));
        // counter = counter - (m / (a * b));
        // counter = counter - (m / (b * c));
        // counter = counter - (m / (a * c));
        let mut alice_water = (m / a) * 6;
        alice_water = alice_water - (m / lcm(a, b)) * 3;
        alice_water = alice_water - (m / lcm(a, c)) * 3;
        alice_water = alice_water + (m / (lcm(a, lcm(c, b)))) * 2;
        let mut bob_water = (m / b) * 6;
        bob_water = bob_water - (m / lcm(a, b)) * 3;
        bob_water = bob_water - (m / lcm(b, c)) * 3;
        bob_water = bob_water + (m / (lcm(a, lcm(c, b)))) * 2;
        let mut carol_water = (m / c) * 6;
        carol_water = carol_water - (m / lcm(c, b)) * 3;
        carol_water = carol_water - (m / lcm(a, c)) * 3;
        carol_water = carol_water + (m / (lcm(a, lcm(c, b)))) * 2;
        answer.push_str(&alice_water.to_string());
        answer.push(' ');
        answer.push_str(&bob_water.to_string());
        answer.push(' ');
        answer.push_str(&carol_water.to_string());
        answer.push('\n');
    }
    print!("{answer}");
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        return gcd(b, a % b);
    }
    fn lcm(a: usize, b: usize) -> usize {
        return a * b / gcd(a, b);
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
    pub fn read_vec_i64(&mut self) -> Vec<i64> {
        return self.read_vec_generic::<i64>();
    }
}
// Last Line comment marker