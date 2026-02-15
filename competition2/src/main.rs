fn main() {
    let mut scanner = Scanner::new();
    p4(&mut scanner);
}
fn p1(scanner:&mut Scanner) {
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n: usize = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut found: bool = false;
        for i in 0..n {
            if a[i] == 67 {
                answer.push_str("YES\n");
                found = true;
                break;
            }
        }
        if !found {
            answer.push_str("NO\n");
        }
    }
    print!("{answer}");
}
fn p2(scanner: &mut Scanner) {
    fn get_base_odd(mut n: usize) -> usize {
        if n == 0 { return 0; }
        while n % 2 == 0 {
            n /= 2;
        }
        n
    }
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n: usize = scanner.read_usize();
        let mut a = scanner.read_vec_usize();
        let mut found_vec: Vec<bool> = Vec::new();
        for _ in 0..n+1{
            found_vec.push(false);
        }
        a.insert(0, 0 as usize);
        let mut possible = true;
        for i in 1..n+1 {
            if get_base_odd(i) == get_base_odd(a[i]) {
                continue;
            } else {
                possible = false;
                break;
            }
        }
        if possible {
            answer.push_str("YES\n");
        } else {
            answer.push_str("NO\n");
        }
    }
    print!("{answer}"); 
}
fn p3(scanner:&mut Scanner) {
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n: usize = scanner.read_usize();
        let mut a = scanner.read_vec_usize();
        let mut count: usize = 0;
        let mut i = 1;
        while i < n {
            if a[i] == a[i-1] || a[i] == 7 - a[i-1] {
                count = count + 1;
                i = i + 2;
                // if a[i] == 1 {
                //     if a[i + 1] == 2 || a[i+1] == 5 {
                //         a[i] = 3;
                //     } else {
                //         a[i] = 2;
                //     }
                // } else if a[i] == 2 {
                //     if a[i + 1] == 1 || a[i+1] == 6 {
                //         a[i] = 5;
                //     } else {
                //         a[i] = 1;
                //     }
                // } else {
                //     if a[i] - 1 == a[i+1] || a[i] - 1 == 7 - a[i+1] {
                //         a[i] = a[i] - 2;
                //     } else {
                //         a[i] = a[i] - 1;
                //     }
                // }
            } else {
                i = i + 1;
            }
        }
        answer.push_str(&count.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
fn p4(scanner:&mut Scanner) {
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n: usize = scanner.read_usize();
        let f = scanner.read_vec_generic::<isize>();
        let mut a: Vec<isize> = Vec::with_capacity(n);
        a.push(0);
        for i in 1..n-1 {
            a.push((f[i-1] - 2*f[i] + f[i+1])/2);
        }
        let d1 = f[0] - f[1];
        let dn = f[n-2] - f[n-1];
        let s = (f[0] + f[n-1]) / (n as isize - 1);
        
        a[0] = (s - d1) / 2;
        a.push((s + dn) / 2);
        for i in 0..n {
            answer.push_str(&a[i].to_string());
            if i < n - 1 { answer.push(' '); }
        }
        answer.push('\n');
    }
    print!("{answer}");
}
/// The scanner. It allows for readings of generic numbers, strings, and vectors of numbers
use std::{io::Stdin, str::FromStr};
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
}
// Last Line comment marker