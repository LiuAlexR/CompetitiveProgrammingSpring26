fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "I");
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
        _ => (),
    }
}

fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    let codeforces = &"codeforces".to_string();
    for _ in 0..t {
        let char = scanner.read_string();
        if codeforces.contains(&char) {
            answer.push_str("YES\n");
        } else {
            answer.push_str("NO\n");
        }
    }
    print!("{answer}");
}
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let s = scanner.read_string().as_bytes().to_vec();
        let mut first = 11;
        let mut second = 11;
        for i in 0..n {
            if s[i] == b'B' {
                if first == 11 {
                    first = i;
                    second = i;
                } else {
                    second = i;
                }
            }
        }
        if second == 11 {
            answer.push_str("1\n");
        } else {
            answer.push_str(&(second + 1 -first).to_string());
            answer.push('\n');
        }
    }
    print!("{answer}");
}
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let mut s = scanner.read_string().as_bytes().to_vec();
        let mut swaps = 0;
        let mut a_counter = 0;
        let mut idx = 0;
        while idx < n {
            if s[idx] == b'A' {
                a_counter = a_counter + 1;
                idx = idx + 1;
            } else {
                if a_counter == 0 {
                    idx = idx + 1;
                } else {
                    swaps = swaps + a_counter;
                    a_counter = 0;
                    s.swap(idx, idx - 1);
                }
            }
        }
        answer.push_str(&swaps.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
fn p_d(scanner: &mut Scanner) {
    let q = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..q {
        let n = scanner.read_usize();
        let s_raw = scanner.read_string();
        let s: Vec<u8> = s_raw.bytes().map(|b| b-b'0').collect();
        if n == 2 {
            if s[0] >= s[1] {
                answer.push_str("NO\n");
                continue;
            } else {
                answer.push_str("YES\n2\n");
                answer.push_str(&s_raw[0..1]);
                answer.push(' ');
                answer.push_str(&s_raw[1..2]);
                answer.push('\n');
            }
        } else {
            answer.push_str("YES\n2\n");
            answer.push_str(&s_raw[0..1]);
            answer.push(' ');
            answer.push_str(&s_raw[1..n]);
            answer.push('\n');
        }
    }
    print!("{answer}");
}
fn p_e(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let a = scanner.read_string();
        let b = scanner.read_string();
        if a[0..1] == b[0..1] {
            answer.push_str("YES\n");
            answer.push_str(&a[0..1]);
            answer.push_str("*\n");
        } else if a[a.len()-1..a.len()] == b[b.len()-1..b.len()] {
            answer.push_str("YES\n*");
            answer.push_str(&a[a.len()-1..a.len()]);
            answer.push_str("\n");
        } else {
            let mut str: Option<&str> = None;
            'outer: for i in 0..a.len()-1 {
                for j in 0..b.len()-1 {
                    if a[i..i+2] == b[j..j+2] {
                        str = Some(&a[i..i+2]);
                        break 'outer;
                    }
                }
            }
            match str {
                Some(x) => {
                    answer.push_str("YES\n*");
                    answer.push_str(x);
                    answer.push_str("*\n");
                },
                None => {
                    answer.push_str("NO\n");
                },
            }
        }
    }
    print!("{answer}");
}
fn p_f(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let s: Vec<u8> = scanner.read_string().bytes().map(|b| 25 - (b-b'a')).collect();
        let t: Vec<u8> = scanner.read_string().bytes().map(|b| 25 - (b-b'a')).collect();
        let mut s_c: usize = 0;
        let mut t_c: usize = 0;
        for i in s {
            s_c = s_c | 1 << i;
        }
        for i in t {
            t_c = t_c | 1 << i;
        }
        if s_c & t_c > 0 {
            answer.push_str("YES\n");
        } else {
            answer.push_str("NO\n");
        }
    }
    print!("{answer}");
}
fn p_g(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let mut s: Vec<u8> = scanner.read_string().bytes().collect();
        let mut t: Vec<u8> = scanner.read_string().bytes().collect();
        let mut idx_s: usize = 0;
        let mut idx_t: usize = 0;
        let mut broken = false;
        while idx_t < t.len() {
            if idx_s < s.len() {
                if s[idx_s] == t[idx_t] {
                    idx_s = idx_s + 1;
                    idx_t = idx_t + 1;
                } else if s[idx_s] == b'?' {
                    s[idx_s] = t[idx_t];
                    idx_s = idx_s + 1;
                    idx_t = idx_t + 1;
                } else {
                    idx_s = idx_s + 1;
                }
            } else {
                answer.push_str("NO\n");
                broken = true;
                break;
            }
        }
        if !broken {
            answer.push_str("YES\n");
            for i in s {
                if i == b'?' {
                    answer.push('a');
                } else {
                    answer.push(i as char);
                }
            }
            answer.push('\n');
        }
    }
    print!("{answer}");
}
fn p_h(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let s = scanner.read_string();
        let mut s_c = String::new();
        s_c.push_str(&s);
        s_c.push_str(&s);
        let mut ones = 0;
        let mut c_ones = 0;
        for i in s_c.bytes() {
            if i == b'1' {
                c_ones = c_ones + 1;
            } else {
                ones = std::cmp::max(ones, c_ones);
                c_ones = 0;
            }
        }
        ones = std::cmp::max(ones, c_ones);
        if ones == s_c.len() {
            ones = ones / 2;
        }
        if ones == s.len() {
            answer.push_str(&(ones * ones).to_string());
            answer.push('\n');
        } else if ones == 0 {
            answer.push_str("0\n");
        } else {
            if ones % 2 == 0 {
                answer.push_str(&((ones + 1)/2 * (ones + 2)/2).to_string());
                answer.push('\n');
            } else {
                answer.push_str(&((ones + 1)/2 * (ones + 1)/2).to_string());
                answer.push('\n');
            }
        }
    }
    print!("{answer}");
}
fn p_i(scanner: &mut Scanner) {
    // let t = scanner.read_usize();
    let mut s: Vec<u8> = scanner.read_string().bytes().collect();
    let n = s.len();
    let mut i = 0;

    while i < n && s[i] == b'a' {
        i += 1;
    }

    if i == n {
        s[n - 1] = b'z';
    } else {
        while i < n && s[i] != b'a' {
            s[i] -= 1;
            i += 1;
        }
    }

    let mut answer = String::new();
    for byte in s {
        answer.push(byte as char);
    }
    println!("{}", answer);
}
fn p_j(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {

    }
    print!("{answer}");
}
use std::process::id;
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