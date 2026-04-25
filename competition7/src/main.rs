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
/// Blocked
/// Sort the array, then reverse it. If there are any duplicates, it doesn't work and cannot be unblocked
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let mut a = scanner.read_vec_usize();
        a.sort();
        a.reverse();
        let mut has_dupes = false;
        for i in 1..n {
            if a[i] == a[i - 1] {
                has_dupes = true;
                break;
            }
        }
        if has_dupes {
            answer.push_str(&"-1\n".to_string());
        } else {
            for i in 0..n {
                answer.push_str(&a[i].to_string());
                answer.push(' ');
            }
            answer.push('\n');
        }
        
    }
    print!("{answer}");
}
/// OIE Excursion
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let nm = scanner.read_vec_usize();
        let a = scanner.read_vec_usize();
        let n = nm[0];
        let m = nm[1];
        let mut running = 1;
        for i in 1..n {
            if a[i] == a[i - 1] {
                running = running + 1;
                if running == m {
                    break;
                }
            } else {
                running = 1;
            }
        }
        if running == m {
            answer.push_str(&"NO\n".to_string());
        } else {
            answer.push_str(&"YES\n".to_string());
        }
        
    }
    print!("{answer}");
}
/// Grid L
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let pq = scanner.read_vec_usize();
        let p = pq[0];
        let q = pq[1];
        let num_segs = p + 2 * q;
        let target = 2 * num_segs + 1;
        let max_check = ((target as f64).sqrt() as usize) + 1;
        let mut i = 3;
        let mut found = false;
        while i < max_check {
            if target % i == 0 {
                let j = target / i;
                if j % 2 == 1{
                    let m = (i - 1) / 2;
                    let n = (j - 1) / 2;
                    if q > n * (m + 1) || q > m * (n + 1) {
                        i = i + 2;
                        continue;
                    }
                    found = true;
                    break;
                }
            }
            i = i + 2;
        }
        if found {
            let new_i = (i - 1) / 2;
            let j = (target / i - 1) / 2;
            answer.push_str(&format!("{new_i} {j}\n"));
        } else {
            answer.push_str(&"-1\n");
        }
        
    }
    print!("{answer}");
}

fn p_d(scanner: &mut Scanner) {
    
}

fn p_e(scanner: &mut Scanner) {

}

fn p_f(scanner: &mut Scanner) {

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