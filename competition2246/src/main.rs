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
        _ => (),
    }
}
// farmpiggie and Subset Sum
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        for i in (1..(n+1)).rev() {
            answer.push_str(&format!("{i}"));
            answer.push(' ');
        }
        answer.push('\n');
    }
    print!("{answer}");
}
// ezraft and Array
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        if n == 1 {
            answer.push_str(&"1");
        } else if n == 2 {
            answer.push_str(&"-1");
        } else {
            answer.push_str(&"1 2 3");
            let mut cur: u64 = 3;
            for _ in 0..(n-3) {
                cur = cur * 2;
                answer.push_str(&format!(" {cur}"));
            }
        }
        answer.push('\n');
    }
    print!("{answer}");
}
// 0mar and Alternating Sums
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_i64();
        let mut number_counts: HashMap<i64, usize> = HashMap::new();
        for i in &a {
            if number_counts.contains_key(i) {
                number_counts.insert(*i, number_counts[&i] + 1);
            } else {
                number_counts.insert(*i, 1);
            }
        }
        let mut count: usize = 0;
        let mut total_sum: usize = 0;
        for key in &number_counts {
            count = count + (*key.1 / 2);
        }
        total_sum += pow(2, count, 1000000007);
        count = 0;
        if number_counts.contains_key(&-1) {
            for i in 0..&a.len()-1 {
                if a[i] + 1 == a[i + 1] {
                    number_counts.insert(-1, number_counts[&-1] - 1);
                    number_counts.insert(a[i], number_counts[&a[i]] - 1);
                    number_counts.insert(a[i + 1], number_counts[&a[i + 1]] - 1);
                    for key in &number_counts {
                        count = count + (*key.1 / 2);
                    }
                    total_sum += pow(2, count, 1000000007);
                    count = 0;
                    number_counts.insert(-1, number_counts[&-1] + 1);
                    number_counts.insert(a[i], number_counts[&a[i]] + 1);
                    number_counts.insert(a[i + 1], number_counts[&a[i + 1]] + 1);
                }
            }
        }
        answer.push_str(&format!("{total_sum}"));
        answer.push('\n');
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