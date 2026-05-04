fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "J");
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
/// Parallel Projection
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let wdh = scanner.read_vec_i64();
        let w = wdh[0];
        let d = wdh[1];
        let h = wdh[2];
        let abfg = scanner.read_vec_i64();
        let a = abfg[0];
        let b = abfg[1];
        let f = abfg[2];
        let g = abfg[3];
        let ans1 = a + f + h + (b - g).abs();
        let ans2 = b + g + h + (a - f).abs();
        let ans3 = h + (w - a) + (w - f) + (b - g).abs();
        let ans4 = h + (d - b) + (d - g) + (a - f).abs();
        let ans = std::cmp::min(ans1, std::cmp::min(ans2, std::cmp::min(ans3, ans4)));
        answer.push_str(&format!("{ans}\n"));
    }
    print!("{answer}");
}
/// Come Together
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let a = scanner.read_vec_i64();
        let b = scanner.read_vec_i64();
        let c = scanner.read_vec_i64();
        let mut sum = 1;
        if (a[0] - b[0]).signum() == (a[0] - c[0]).signum() {
            sum = sum + std::cmp::min((a[0] - b[0]).abs(), (a[0] - c[0]).abs()).abs();
        }
        if (a[1] - b[1]).signum() == (a[1] - c[1]).signum() {
            sum = sum + std::cmp::min((a[1] - b[1]).abs(), (a[1] - c[1]).abs()).abs();
        }
        answer.push_str(&format!("{sum}\n"));
    }
    print!("{answer}");
}
/// Integer Points
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let p = scanner.read_vec_usize();
        let m = scanner.read_usize();
        let q = scanner.read_vec_usize();
        let mut even_p: i64 = 0;
        let mut odd_p: i64 = 0;
        for i in 0..n {
            if p[i] % 2 == 0 {
                even_p += 1;
            } else {
                odd_p += 1;
            }
        }
        let mut even_q: i64 = 0;
        let mut odd_q: i64 = 0;
        for i in 0..m {
            if q[i] % 2 == 0 {
                even_q += 1;
            } else {
                odd_q += 1;
            }
        }
        let ans = (even_p ) * (even_q ) + (odd_p ) * (odd_q );
        answer.push_str(&format!("{ans}\n"));
    }
    print!("{answer}");
}
/// Vasya and Cornfield
fn p_d(scanner: &mut Scanner) {
    let nd = scanner.read_vec_i64();
    let n = nd[0];
    let d = nd[1];
    let m = scanner.read_usize();
    
    let mut answer = String::new();
    
    for _ in 0..m {
        let xy = scanner.read_vec_i64();
        let x = xy[0];
        let y = xy[1];
        
        let sum = x + y;
        let cond1 = sum >= d && sum <= 2 * n - d;
        
        let diff = y - x;
        let cond2 = diff >= -d && diff <= d;
        
        if cond1 && cond2 {
            answer.push_str("YES\n");
        } else {
            answer.push_str("NO\n");
        }
    }
    
    print!("{answer}");
}
/// NN and the Optical Illusion
fn p_e(scanner: &mut Scanner) {
    let nr = scanner.read_vec_usize();
    let n = nr[0];
    let r = nr[1];
    let ans: f64 = (f64::consts::PI / n as f64).sin() * r as f64 / (1.0 - (f64::consts::PI / n as f64).sin());
    println!("{ans}");
}
/// Shawarma Tent
fn p_f(scanner: &mut Scanner) {
    let nxy = scanner.read_vec_usize();
    let n = nxy[0];
    let x = nxy[1];
    let y = nxy[2];
    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;
    for i in 0..n {
        let xy = scanner.read_vec_usize();
        let sx = xy[0];
        let sy = xy[1];
        if x > sx {
            left += 1;
        }
        if x < sx {
            right += 1;
        }
        if y > sy {
            down += 1;
        }
        if y < sy {
            up += 1;
        }
    }
    if up >= down && up >= left && up >= right {
        let yn = y + 1;
        println!("{up}\n{x} {yn}");
    } else if down >= left && down >= right {
        let yn = y - 1;
        println!("{down}\n{x} {yn}");
    } else if left >= right {
        let xn = x - 1;
        println!("{left}\n{xn} {y}");
    } else {
        let xn = x + 1;
        println!("{right}\n{xn} {y}");
    }
}
/// Inscribed Figures
fn p_g(scanner: &mut Scanner) {
    let n = scanner.read_usize();
    let a = scanner.read_vec_usize();
    let mut sum = 0;
    for i in 1..n {
        let prev = a[i - 1];
        let curr = a[i];

        if (prev == 2 && curr == 3) || (prev == 3 && curr == 2) {
            println!("Infinite");
            return;
        }

        if (prev == 1 && curr == 2) || (prev == 2 && curr == 1) {
            sum += 3;
        }
        else if (prev == 1 && curr == 3) || (prev == 3 && curr == 1) {
            sum += 4;
        }
        if i > 1 && a[i-2] == 3 && a[i-1] == 1 && a[i] == 2 {
            sum -= 1;
        }
    }

    println!("Finite\n{}", sum);
    
}
/// The Morning Star
fn p_h(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let mut x_to_count: HashMap<i64, usize> = HashMap::new();
        let mut y_to_count: HashMap<i64, usize> = HashMap::new();
        let mut d1_to_count: HashMap<i64, usize> = HashMap::new();
        let mut d2_to_count: HashMap<i64, usize> = HashMap::new();
        for i in 0..n {
            let xy = scanner.read_vec_i64();
            let x = xy[0];
            let y = xy[1];
            let _ = match x_to_count.get(&x) {
                Some(val) => x_to_count.insert(x, *val + 1),
                None => x_to_count.insert(x, 1),
            };
            let _ = match y_to_count.get(&y) {
                Some(val) => y_to_count.insert(y, *val + 1),
                None => y_to_count.insert(y, 1),
            };
            let _ = match d1_to_count.get(&(x - y)) {
                Some(val) => d1_to_count.insert(x - y, *val + 1),
                None => d1_to_count.insert(x - y, 1),
            };
            let _ = match d2_to_count.get(&(x + y)) {
                Some(val) => d2_to_count.insert(x + y, *val + 1),
                None => d2_to_count.insert(x + y, 1),
            };
        }
        let mut sum = 0;
        for i in x_to_count.iter() {
            let val = *i.1;
            sum = sum + val * (val - 1);
        }
        for i in y_to_count.iter() {
            let val = *i.1;
            sum = sum + val * (val - 1);
        }
        for i in d1_to_count.iter() {
            let val = *i.1;
            sum = sum + val * (val - 1);
        }
        for i in d2_to_count.iter() {
            let val = *i.1;
            sum = sum + val * (val - 1);
        }
        answer.push_str(&format!("{sum}\n"));
    }
    print!("{answer}");
}
/// Polygon for the Angle
fn p_i(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let ang = scanner.read_usize();
        
        let g = gcd(ang, 180);
        let mut k = ang / g;
        let mut n = 180 / g;
        if k == n - 1 {
            k *= 2; 
            n *= 2; 
        }
        
        answer.push_str(&format!("{n}\n"));
    }
    print!("{answer}");
}
/// Very Suspicious
fn p_j(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut max_triangles = Vec::new();
    let mut current_l = 0;
    loop {
        let n1 = current_l / 3;
        let n2 = (current_l - n1) / 2;
        let n3 = current_l - n1 - n2;
        
        let t_count = 2 * (n1 * n2 + n1 * n3 + n2 * n3);
        max_triangles.push(t_count);
        
        if t_count >= 1_000_000_000 {
            break;
        }
        current_l += 1;
    }

    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_i32();
        
        let res = match max_triangles.binary_search(&n) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        
        answer.push_str(&format!("{res}\n"));
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
use core::{f64, num};
use std::{collections::HashSet, f64::consts::SQRT_2, hash::Hash, process::id};
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