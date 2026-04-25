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
        "E" => p_e(scanner),
        "F" => p_f(scanner),
        "G" => p_g(scanner),
        "H" => p_h(scanner),
        "I" => p_i(scanner),
        "J" => p_j(scanner),
        "K" => p_k(scanner),
        "L" => p_l(scanner),
        _ => (),
    }
}
/// A Wonderful Contest
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        if a.contains(&(100 as usize)) {
            answer.push_str(&"YES\n");
        } else {
            answer.push_str(&"NO\n");
        }
    }
    print!("{answer}");
}
/// Artistic Balance Tree
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let nm = scanner.read_vec_usize();
        let n = nm[0];
        let m = nm[1];
        let mut a = scanner.read_vec_i64();
        let x = scanner.read_vec_usize();
        let mut a_e: Vec<i64> = vec![];
        let mut a_o: Vec<i64> = vec![];
        for i in 0..n {
            if i % 2 == 0 {
                a_e.push(a[i]);
            } else {
                a_o.push(a[i]);
            }
        }
        a_e.sort();
        a_e.reverse();
        a_o.sort();
        a_o.reverse();
        let mut sum = 0;
        let mut a_e_counter = 0;
        let mut a_o_counter = 0;
        for i in 0..m {
            if x[i] % 2 == 0 {
                a_e_counter += 1;
            } else {
                a_o_counter += 1;
            }
        }
        for i in 0..a_e.len() {
            if i >= a_o_counter {
                sum += a_e[i];
            } else {
                if a_e[i] <= 0 {
                    sum += a_e[i];
                }
            }
        }
        
        for i in 0..a_o.len() {
            if i >= a_e_counter {
                sum += a_o[i];
            } else {
                if a_o[i] <= 0 {
                    sum += a_o[i];
                }
            }
        }
        let mut sum = 0;
        for i in 0..n {
            sum += a[i];
        }
        answer.push_str(&format!("{sum}\n"));
    }
    print!("{answer}");
}
/// Red-Black Pairs
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();

    }
    print!("{answer}");
}
/// Exceptional Segments
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {

    }
    print!("{answer}");
}
/// Prefix Sum Primes
fn p_e(scanner: &mut Scanner) {
    let mut answer = String::new();

    print!("{answer}");
}
/// Alarm Clocks Everywhere
fn p_f(scanner: &mut Scanner) {
    let mut answer = String::new();

    print!("{answer}");
}
fn p_g(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize() as isize;
    }
    print!("{answer}");
}

fn p_h(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {

    }
    print!("{answer}");
}
fn p_i(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        
    }
    print!("{answer}");
}
fn p_j(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        
    }
    print!("{answer}");
}
fn p_k(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        
    }
    print!("{answer}");
}
fn p_l(scanner: &mut Scanner) {
        let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        
    }
    print!("{answer}");
}
fn p_m(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        
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
// Last Line comment marker