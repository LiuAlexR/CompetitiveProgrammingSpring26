
fn main() {
    // Rational - Base case of leftmost number and the number 1 - both of these are instant wins for the current player
    let mut scanner: Scanner = Scanner::new();
    // let mut answer: String = String::new();
    // let t = scanner.read_usize();
    // for _ in 0..t {
    //     let n = scanner.read_usize();
    //     let permuation = scanner.read_vec_generic::<usize>();

    // }
    sasha(&mut scanner);
}
fn sasha(scanner: &mut Scanner) -> () {
    let a = scanner.read_usize();
    let nums = scanner.read_vec_generic::<usize>();
    let mut xor_vec: Vec<usize> = vec![0; a]; 
    xor_vec[0] = nums[0];
    for i in 1..a {
        xor_vec[i] = xor_vec[i-1] ^ nums[i];
    }
    let mut count = 0;
    for l in 0..a {
        for r in l..a {
            // Change 2: Correct parity check for even length (r-l+1)
            if (r - l + 1) % 2 != 0 {
                continue;
            }
            // Change 3: Correct mid calculation for 0-based indexing
            let mid = l + (r - l + 1) / 2 - 1;
            
            if l == 0 {
                let sum_left = xor_vec[mid];
                let sum_right = xor_vec[mid] ^ xor_vec[r];
                if sum_left == sum_right {
                    count += 1;
                }
            } else {
                let sum_left = xor_vec[l-1] ^ xor_vec[mid];
                let sum_right = xor_vec[mid] ^ xor_vec[r];
                if sum_left == sum_right {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");
}
/// The scanner. It allows for readigs of generic numbers, strings, and vectors of numbers
use std::{io::Stdin, str::FromStr};
#[allow(dead_code)]
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
}
// Last Line comment marker