fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "D");
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
        _ => (),
    }
}
// Iskander and Drawings
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let st = scanner.read_string();
        let s = st.split("*");
        let mut max_size: isize = 0;
        for string in s {
            // println!("{string}");
            if string.len() as isize > max_size {
                max_size = string.len() as isize;
            }
        }
        let mut num_app = (max_size ) / 2;
        if max_size % 2 == 1 {
            num_app += 1;
        }
        answer.push_str(&format!("{num_app}\n"));
    }
    print!("{answer}");
}
// Nikita and Books
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut no = false;
        let mut running: usize = 0;
        let mut expected: usize = 0;
        for i in 0..(n) {
            running = running + a[i];
            expected = expected + i + 1;
            if running < expected {
                no = true;
                break;
            }
        }
        if no {
            answer.push_str(&"NO\n");
        } else {
            answer.push_str(&"YES\n");
        }
    }
    print!("{answer}");
}
// Stepan and Permutation
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let nxy = scanner.read_vec_usize();
        let p = scanner.read_vec_usize();
        let xy_gcd = gcd(nxy[1], nxy[2]);
        if xy_gcd == 1 {
            answer.push_str(&"YES\n");
        } else {
            let mut sortable = true;
            for i in 0..nxy[0] {
                let mut num = p[i];
                if (num - 1).abs_diff(i) % xy_gcd != 0 {
                    sortable = false;
                    break;
                }
            }
            if sortable {
                answer.push_str(&"YES\n");
            } else {
                answer.push_str(&"NO\n");
            }
        }
    }
    print!("{answer}");
}
// Yaroslav and Productivity
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let nm = scanner.read_vec_usize();
        let n = nm[0];
        let m = nm[1];
        let a = scanner.read_vec_usize();
        let b = scanner.read_vec_usize();
    }
    print!("{answer}");
}
fn p_e(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {

    }
    print!("{answer}");
}
use core::num;
use std::{collections::HashSet, f64::consts::SQRT_2, process::id};
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
#[allow(dead_code)]
fn pow(base: usize, power: usize, modulo: usize) -> usize {
    let mut val: usize = 1;
    let mut the_power = power;
    let mut the_base = base;
    while the_power > 0 {
        if the_power % 2 == 1 {
            val = (val * the_base) % modulo;
        }
        the_base = (the_base * the_base) % modulo;
        the_power = the_power / 2;
    }
    return val;
}
fn mmax(n1: usize, n2: usize) -> usize {
    if n1 >= n2 {
        return n1;
    } else {
        return n2;
    }
}
fn mmin(n1: usize, n2: usize) -> usize {
    if n1 <= n2 {
        return n1;
    } else {
        return n2;
    }
}
fn subset_sum(arr: Vec<usize>, target: usize) -> Option<Vec<usize>> {
    let mut dp: Vec<Vec<usize>> = vec![vec![0; target + 1]; arr.len() + 1];
    let mut solution: Vec<usize> = Vec::new();
    return Some(solution);
}
// Last Line comment marker