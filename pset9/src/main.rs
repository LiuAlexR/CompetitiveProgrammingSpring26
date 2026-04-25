fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "F");
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
/// Minority
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let s = scanner.read_string();
        let num_chars = s.len();
        let mut zeroes = 0;
        let mut ones = 0;
        for i in 0..num_chars {
            if s.as_bytes()[i] == b'0' {
                zeroes = zeroes + 1;
            } else {
                ones = ones + 1;
            }
        }
        if num_chars < 3 {
            answer.push_str(&"0\n");
            continue;
        }
        if ones == zeroes {
            answer.push_str(&mmin(zeroes - 1, ones).to_string());
            answer.push('\n');
            continue;
        }
        answer.push_str(&mmin(zeroes, ones).to_string());
        answer.push('\n');
        
    }
    print!("{answer}");
}
/// Luntik and Subsequences
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut count_zeroes = 0;
        let mut count_ones = 0;
        for i in 0..n {
            if a[i] == 0 {
                count_zeroes += 1;
            } else if a[i] == 1 {
                count_ones += 1;
            }
        }
        let ans: usize = (count_ones * (1 << count_zeroes));
        answer.push_str(&ans.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Consecutive Points Segment
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let x = scanner.read_vec_usize();
        if x[n - 1] - x[0] <= (n + 1) {
            answer.push_str("YES\n");
        } else {
            answer.push_str("NO\n");
        }
    }
    print!("{answer}");
}
/// Mainak and Interesting Sequence
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let nm = scanner.read_vec_usize();
        let n = nm[0];
        let m = nm[1];
        if m < n {
            answer.push_str(&"No\n");
        } else if m == n {
            answer.push_str(&"Yes\n");
            for _ in 0..n {

                answer.push('1');
                answer.push(' ');
            }
            answer.push('\n');
        } else {
            if m % n == 0 {
                let val = m / n;
                answer.push_str(&"Yes\n");
                for _ in 0..n {
                    answer.push_str(&val.to_string());
                    answer.push(' ');
                }
                answer.push('\n');
            } else {
                if n % 2 == 0 {
                    if m % 2 == 0 {
                        answer.push_str(&"Yes\n");
                        for _ in 0..(n-2) {
                            answer.push_str("1 ");
                        }
                        let val = m - n + 2;
                        answer.push_str(&(val / 2).to_string());
                        answer.push(' ');
                        answer.push_str(&(val / 2).to_string());
                        answer.push('\n');
                    } else {
                        answer.push_str(&"No\n");
                    }
                } else {
                    answer.push_str(&"Yes\n");
                    for _ in 0..(n-1) {
                        answer.push_str("1 ");
                    }
                    answer.push_str(&(m - n + 1).to_string());
                    answer.push('\n');
                }
            }
        }
    }
    print!("{answer}");
}
/// Mocha and Hiking
fn p_e(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut idx = 0;
        if a[0] != 0 {
            answer.push_str(&(n + 1).to_string());
            answer.push(' ');
            for i in 1..(n + 1) {
                answer.push_str(&(i).to_string());
                answer.push(' ');
            }
            answer.push('\n');
            continue;
        } else if a[n - 1] == 0 {
            for i in 1..(n + 2) {
                answer.push_str(&(i).to_string());
                answer.push(' ');
            }
            answer.push('\n');
            continue;
        }
        else {
            let mut found = false;
            for i in 0..(n - 1) {
                if a[i] == 0 && a[i + 1] == 1 {
                    for j in 1..=(i + 1) {
                        answer.push_str(&j.to_string());
                        answer.push(' ');
                    }
                    answer.push_str(&(n + 1).to_string());
                    for j in (i + 2)..=n {
                        answer.push(' ');
                        answer.push_str(&j.to_string());
                    }
                    answer.push('\n');
                    found = true;
                    break;
                }
            }
            
            if !found {
                answer.push_str("-1\n");
            }
        }
    }
    print!("{answer}");
}
/// Game of Ball Passing
fn p_f(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        
        let mut max_val = 0;
        let mut sum_val = 0;
        
        for i in 0..n {
            if a[i] > max_val {
                max_val = a[i];
            }
            sum_val += a[i];
        }
        if max_val == 0 {
            answer.push_str("0\n");
            continue;
        }
    
        let sum_others = sum_val - max_val;
        
        if max_val <= sum_others + 1 {
            answer.push_str("1\n");
        } else {
            let diff = max_val - sum_others;
            answer.push_str(&diff.to_string());
            answer.push('\n');
        }
    }
    print!("{answer}");
}
fn p_g(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        
    }
    print!("{answer}");
}
fn p_h(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        
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