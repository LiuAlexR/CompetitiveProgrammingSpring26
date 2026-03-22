fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "A");
}
/// The main problem solution archive
#[allow(dead_code, unused_variables)]
fn problems(scanner: &mut Scanner, num: &str) {
    match num {
        "A" => p_a(scanner),
        _ => (),
    }
}

fn p_a(scanner: &mut Scanner) {
    fn is_narek(c: u8) -> u8{
        match c {
            b'n' => 0,
            b'a' => 1,
            b'r' => 2,
            b'e' => 3,
            b'k' => 4,
            _ => 5,
        }
    }
    struct StringEssentials {
        start: u8,
        c_score: usize,
        n_score: usize,
        end: u8,
    }
    impl StringEssentials {
        fn merge(&self, other: &StringEssentials) -> StringEssentials {
            let mut new_struct = StringEssentials { start: self.start, c_score: self.c_score + other.c_score, n_score: self.n_score + other.n_score, end: other.end };
            if (self.end + 1) % 5 == 0
            new_struct
        }
    }
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let nm = scanner.read_vec_usize();
        let mut structs: Vec<StringEssentials> = Vec::new();
        for _ in 0..nm[0] {
            let incoming = scanner.read_string().as_bytes().to_vec();
            let mut cur_find = 5;
            let mut starting_n = false;
            let mut incoming_struct: StringEssentials  = StringEssentials { start: 5, c_score: 0, n_score: 0, end: 5 };
            for i in 0..nm[1] {
                if cur_find == 5 {
                    cur_find = is_narek(incoming[i]);
                    if cur_find != 5 {
                        incoming_struct.start = cur_find;
                    }
                }
                if cur_find == 0 {
                    starting_n = true;
                    if is_narek(incoming[i]) == 0 {
                        cur_find = 1;
                    } else if is_narek(incoming[i]) != 5 {
                        incoming_struct.c_score += 1;
                    }
                } else if cur_find == 1 {
                    if is_narek(incoming[i]) == 1 {
                        cur_find = 2;
                    } else if is_narek(incoming[i]) != 5 {
                        incoming_struct.c_score += 1;
                    }
                } else if cur_find == 2 {
                    if is_narek(incoming[i]) == 2 {
                        cur_find = 3;
                    } else if is_narek(incoming[i]) != 5 {
                        incoming_struct.c_score += 1;
                    }
                }else if cur_find == 3 {
                    if is_narek(incoming[i]) == 3 {
                        cur_find = 4;
                    } else if is_narek(incoming[i]) != 5 {
                        incoming_struct.c_score += 1;
                    }
                } else if cur_find == 4 {
                    if is_narek(incoming[i]) == 4 {
                        cur_find = 0;
                        if starting_n {
                            incoming_struct.n_score += 5;
                        }
                    } else if is_narek(incoming[i]) != 5 {
                        incoming_struct.c_score += 1;
                    }
                }
            }
            if cur_find == 5 || incoming_struct.c_score > incoming_struct.n_score + 9 {
                continue;
            } else if cur_find == 0 {
                incoming_struct.end = 4;
            } else {
                incoming_struct.end = cur_find - 1;
            }
            structs.push(incoming_struct);
        }
        for i in structs {
            println!("{},{},{},{}", i.start,i.c_score,i.n_score,i.end);
        }
    }
    print!("{answer}");
}
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