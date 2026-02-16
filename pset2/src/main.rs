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
        "J" => p_j(scanner),
        _ => (),
    }
}
fn p_a(scanner: &mut Scanner) {
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n: usize = scanner.read_usize();
        let a: Vec<usize> = scanner.read_vec_usize();
        let mut max: usize = 0;
        let mut max_index: usize = 0;
        for i in 0..n {
            if a[i] > max {
                max = a[i];
                max_index = i;
            } else if a[i]  == max {
                if i % 2 == 0 {
                    max_index = i;
                }
            }
        }
        if max_index % 2 == 0 {
            answer.push_str(&(max + (n + 1)/2).to_string());
        } else {
            answer.push_str(&(max + (n)/2).to_string());
        }
        
        answer.push('\n');
    }
    print!("{answer}");
}
fn p_b(scanner: &mut Scanner) {
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n: usize = scanner.read_usize();
        let the_string = scanner.read_string();
        let the_bytes = the_string.as_bytes();
        let mut counter: usize = 0;
        let mut i = 0;
        if n < 3 {
            answer.push('0');
            answer.push('\n');
            continue;
        }
        while i < n - 2 {
            if the_bytes[i] == 109 {
                if the_bytes[i+1] == 97 && the_bytes[i+2] == 112 {
                    counter = counter + 1;
                    i = i + 2;
                }
            } else if the_bytes[i] == 112 {
                if the_bytes[i+1] == 105 && the_bytes[i+2] == 101 {
                    counter = counter + 1;
                    i = i + 2;
                }
            } 
            i = i + 1;
        }
        answer.push_str(&counter.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
fn p_c(scanner: &mut Scanner) {
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let mut grid: Vec<Vec<u8>> = Vec::new();
        let nm: Vec<usize> = scanner.read_vec_usize();
        let n = nm[0];
        let m = nm[1];

        for _ in 0..n {
            let the_string = scanner.read_string();
            grid.push(the_string.as_bytes().to_vec());
        }

        let mut works: bool = true;
        'outer: for a in 0..n {
            for b in 0..m {
                if grid[a][b] == 49 {
                    let mut cr = 0;
                    for l in 0..=a {
                        if grid[l][b] == 49 { cr += 1; }
                    }
                    if cr != (a + 1) {
                        let mut cc = 0;
                        for k in 0..=b {
                            if grid[a][k] == 49 { cc += 1; }
                        }
                        
                        if cc != (b + 1) {
                            works = false;
                            break 'outer;
                        }
                    }
                }
            }
        }
        
        if works {
            answer.push_str("YES\n");
        } else {
            answer.push_str("NO\n");
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
    
}
fn p_h(scanner: &mut Scanner) {
    
}
fn p_i(scanner: &mut Scanner) {
    
}
fn p_j(scanner: &mut Scanner) {
    
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