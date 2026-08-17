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
        _ => (),
    }
}
// house robber
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let x = scanner.read_vec_usize();
        let mut cummulative_sum = vec![0; n];
        let mut solution_base = vec![0; n];
        if n > 0 {
            cummulative_sum[0] = x[0];
        }
        if n > 1 {
            if x[1] > x[0] {
                cummulative_sum[1] = x[1];
                solution_base[1] = 1;
            } else {
                cummulative_sum[1] = x[0];
                solution_base[1] = 0;
            }
        }
        for i in 2..n {
            let rob_cur_val = cummulative_sum[i - 2] + x[i];
            let not_rob = cummulative_sum[i - 1];
            if rob_cur_val > not_rob {
                cummulative_sum[i] = rob_cur_val;
                solution_base[i] = i - 2;
            } else {
                cummulative_sum[i] = not_rob;
                solution_base[i] = i - 1;
            }
        }
        answer.push_str(&format!("{}\n", cummulative_sum[n - 1]));
    }
    print!("{answer}");
}
// house robber 2
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let x = scanner.read_vec_usize();
        let mut cummulative_sum = vec![0; n];
        let mut solution_base = vec![0; n];
        if n > 0 {
            cummulative_sum[0] = x[0];
        }
        if n > 1 {
            if x[1] > x[0] {
                cummulative_sum[1] = x[1];
                solution_base[1] = 1;
            } else {
                cummulative_sum[1] = x[0];
                solution_base[1] = 0;
            }
        }
        for i in 2..(n-1) {
            let rob_cur_val = cummulative_sum[i - 2] + x[i];
            let not_rob = cummulative_sum[i - 1];
            if rob_cur_val > not_rob {
                cummulative_sum[i] = rob_cur_val;
                solution_base[i] = i - 2;
            } else {
                cummulative_sum[i] = not_rob;
                solution_base[i] = i - 1;
            }
        }
        let include_first = cummulative_sum[n - 2];
        let mut cummulative_sum = vec![0; n];
        let mut solution_base = vec![0; n];
        if n > 0 {
            cummulative_sum[0] = 0;
        }
        if n > 1 {
            cummulative_sum[1] = x[1];
        }
        if n > 2 {
            if x[2] > x[1] {
                cummulative_sum[2] = x[2];
                solution_base[2] = 2;
            } else {
                cummulative_sum[2] = x[1];
                solution_base[2] = 0;
            }
        }
        for i in 3..(n) {
            let rob_cur_val = cummulative_sum[i - 2] + x[i];
            let not_rob = cummulative_sum[i - 1];
            if rob_cur_val > not_rob {
                cummulative_sum[i] = rob_cur_val;
                solution_base[i] = i - 2;
            } else {
                cummulative_sum[i] = not_rob;
                solution_base[i] = i - 1;
            }
        }
        let include_last = cummulative_sum[n - 1];
        answer.push_str(&format!("{}\n", mmax(include_first, include_last)));
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