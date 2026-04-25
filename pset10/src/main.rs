fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "H");
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
/// Digits Sum
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        answer.push_str(&((n + 1) / 10).to_string());
        answer.push('\n');    
    }
    print!("{answer}");
}
/// Doremy's Perfect Math Class
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut running = 0;
        let mut max = 0;
        for i in a {
            running = gcd(running, i);
            if i > max {
                max = i;
            }
        }
        answer.push_str(&(max / running).to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Assembly via Remainders
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let x = scanner.read_vec_usize();
        let mut val = 501;
        let mut a: Vec<usize> = vec![501; n];
        for i in 0..(n-1) {
            val = val + x[i];
            a[i + 1] = val;
        }
        for i in a {
            answer.push_str(&i.to_string());
            answer.push(' ');
        }
        answer.push('\n');
    }
    print!("{answer}");
}
/// Lunatic Never Content
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut running = 0;
        for i in 0..(n/2) {
            let diff = mmax(a[i], a[n - i - 1]) - mmin(a[i], a[n - i - 1]);
            running = gcd(running, diff);
        }
        answer.push_str(&format!("{running}\n"));
    }
    print!("{answer}");
}
/// Prefix Sum Primes
fn p_e(scanner: &mut Scanner) {
    let mut answer = String::new();
    let n = scanner.read_usize();
    let a = scanner.read_vec_usize();
    let mut num_1 = 0;
    let mut num_2 = 0;
    for i in a {
        if i == 1 {
            num_1 += 1;
        } else {
            num_2 += 1;
        }
    }
    if num_1 == 0 {
        for _ in 0..num_2 {
            answer.push_str(&"2 ");
        }
    } else if num_2 == 0 {
        for _ in 0..num_1 {
            answer.push_str(&"1 ");
        }
    } else {
        answer.push_str(&"2 ");
        answer.push_str(&"1 ");
        num_1 -= 1;
        num_2 -= 1;
        for _ in 0..num_2 {
            answer.push_str(&"2 ");
        }
        for _ in 0..num_1 {
            answer.push_str(&"1 ");
        }
    }
    print!("{answer}");
}
/// Alarm Clocks Everywhere
fn p_f(scanner: &mut Scanner) {
    let mut answer = String::new();
    let nm = scanner.read_vec_usize();
    let n = nm[0];
    let m = nm[1];
    let x = scanner.read_vec_usize();
    let p = scanner.read_vec_usize();
    let mut running_gcd = 0;
    for i in 1..n {
        running_gcd = gcd(running_gcd, x[i] - x[0]);
    }
    let mut found = false;
    let mut the_val = 0;
    for i in 0..m {
        if running_gcd % p[i] == 0 {
            found = true;
            the_val = i + 1;
            break;
        }
    }
    if found {
        let p_0 = x[0];
        answer.push_str(&format!("YES\n{p_0} {the_val}\n"));
    } else {
        answer.push_str(&"NO\n");
    }
    print!("{answer}");
}
/// I Hate 1111
fn p_g(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize() as isize;
        let mut found = false;
        for b in 0..11 {
            let remaining = n - (b * 111);
            if remaining >= 0 && remaining % 11 == 0 {
                found = true;
                break;
            }
        }
        if found {
            answer.push_str("YES\n");
        } else {
            answer.push_str("NO\n");
        }
    }
    print!("{answer}");
}
/// Diluc and Kaeya
fn p_h(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let s = scanner.read_string();
        
        // Map to store (D_ratio, K_ratio) -> count
        let mut ratio_counts = HashMap::new();
        let mut d_count = 0;
        let mut k_count = 0;
        
        let mut results = Vec::with_capacity(n);

        for c in s.chars() {
            if c == 'D' {
                d_count += 1;
            } else {
                k_count += 1;
            }

            // Simplify the ratio
            let g = gcd(d_count, k_count);
            let simplified_ratio = (d_count / g, k_count / g);

            // The answer for this prefix is the number of times 
            // we've seen this ratio before + 1
            let count = ratio_counts.entry(simplified_ratio).or_insert(0);
            *count += 1;
            results.push(*count);
        }
        for i in results {
            answer.push_str(&i.to_string());
            answer.push(' ');
        }
        answer.push('\n');
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