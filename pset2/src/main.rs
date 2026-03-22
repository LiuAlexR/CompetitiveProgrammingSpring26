fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "F");
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
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let qd: Vec<usize> = scanner.read_vec_usize();
        let numbers: Vec<usize> = scanner.read_vec_usize();
        for a in numbers {
            if a >= qd[1] * 10 {
                answer.push_str("YES\n");
            } else {
                let mut found = false;
                for k in 1..10 {
                    if a >= k * qd[1] && (a - k * qd[1]) % 10 == 0 {
                        answer.push_str("YES\n");
                        found = true;
                        break;
                    }
                }
                if !found {
                    answer.push_str("NO\n");
                }
            }
        }
    }
    print!("{answer}");
}
fn p_e(scanner: &mut Scanner) {
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let mut a = scanner.read_vec_usize();
        a.sort();
        a.reverse();
        let mut aliceturn: bool = true;
        let mut alice_score = 0;
        let mut bob_score = 0;
        for num in a {
            if aliceturn {
                if num % 2 == 0 {
                    alice_score = alice_score + num;
                }
            } else {
                if num % 2 == 1 {
                    bob_score = bob_score + num;
                }
            }
            aliceturn = !aliceturn;
        }
        if alice_score > bob_score {
            answer.push_str("Alice\n");
        } else if alice_score < bob_score {
            answer.push_str("Bob\n");
        } else {
            answer.push_str("Tie\n");
        }
    }
    print!("{answer}");
}
/// DFS
fn p_f(scanner: &mut Scanner) {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    struct state {
        length_of_parsed: usize,
        current_index: usize,
        direction: usize, // 1 for Right, 0 for Left
    }

    let t_cases: usize = scanner.read_usize();
    let mut answer: String = String::new();

    for _ in 0..t_cases {
        let s = scanner.read_string().as_bytes().to_vec();
        let t = scanner.read_string().as_bytes().to_vec();
        let mut stack: VecDeque<state> = VecDeque::new();
        let mut done = false;

        // Initial placements
        'adder: for item in 0..s.len() {
            if s[item] == t[0] {
                let x = state {
                    length_of_parsed: 1,
                    current_index: item,
                    direction: 1,
                };
                if x.length_of_parsed == t.len() {
                    done = true;
                    break 'adder;
                }
                stack.push_front(x);
            }
        }

        if done {
            answer.push_str("YES\n");
            continue;
        }

        // Standard DFS using the stack
        while let Some(x_u) = stack.pop_front() {
            let next_char_index = x_u.length_of_parsed;
            if next_char_index == t.len() {
                done = true;
                break;
            }

            if x_u.direction == 1 {
                if x_u.current_index + 1 < s.len() && s[x_u.current_index + 1] == t[next_char_index] {
                    stack.push_front(state {
                        length_of_parsed: next_char_index + 1,
                        current_index: x_u.current_index + 1,
                        direction: 1,
                    });
                }
                if x_u.current_index > 0 && s[x_u.current_index - 1] == t[next_char_index] {
                    stack.push_front(state {
                        length_of_parsed: next_char_index + 1,
                        current_index: x_u.current_index - 1,
                        direction: 0,
                    });
                }
            } 
            else {
                if x_u.current_index > 0 && s[x_u.current_index - 1] == t[next_char_index] {
                    stack.push_front(state {
                        length_of_parsed: next_char_index + 1,
                        current_index: x_u.current_index - 1,
                        direction: 0,
                    });
                }
            }
        }

        if done {
            answer.push_str("YES\n");
        } else {
            answer.push_str("NO\n");
        }
    }
    print!("{answer}");
}
fn p_g(scanner: &mut Scanner) {
    let n: usize = scanner.read_usize();
    let b: Vec<i64> = scanner.read_vec_generic::<i64>();
    let mut groups: HashMap<i64, i64> = HashMap::new();

    for i in 0..n {
        let city_index = (i + 1) as i64;
        let beauty = b[i];
        let key = city_index - beauty;
        
        let current_sum = groups.entry(key).or_insert(0);
        *current_sum += beauty;
    }

    let mut max_beauty: i64 = 0;
    for (&key, &sum) in groups.iter() {
        if sum > max_beauty {
            max_beauty = sum;
        }
    }

    println!("{}", max_beauty);
}
fn p_h(scanner: &mut Scanner) {
    let t: usize = scanner.read_usize();
    let mut answer: String = String::new();
    
    let inf: usize = 2_000_000_000;

    for _ in 0..t {
        let _ = scanner.read_string();
        let nk = scanner.read_vec_usize();
        let n: usize = nk[0];
        let k: usize = nk[1];
        
        let a: Vec<usize> = scanner.read_vec_usize(); 
        let t_values: Vec<usize> = scanner.read_vec_usize(); 
        
        let mut grid: Vec<usize> = vec![inf; n];
        for i in 0..k {
            grid[a[i] - 1] = t_values[i];
        }
        let mut left: Vec<usize> = vec![inf; n];
        let mut last_temp: usize = inf;
        for i in 0..n {
            last_temp = std::cmp::min(last_temp + 1, grid[i]);
            left[i] = last_temp;
        }

        let mut right: Vec<usize> = vec![inf; n];
        last_temp = inf;
        for i in (0..n).rev() {
            last_temp = std::cmp::min(last_temp + 1, grid[i]);
            right[i] = last_temp;
        }

        for i in 0..n {
            let res = std::cmp::min(left[i], right[i]);
            answer.push_str(&res.to_string());
            if i < n - 1 {
                answer.push_str(" ");
            }
        }
        answer.push_str("\n");
    }
    print!("{answer}");
}
fn p_i(scanner: &mut Scanner) {
    
}
fn p_j(scanner: &mut Scanner) {
    
}
/// The scanner. It allows for readings of generic numbers, strings, and vectors of numbers
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
}
// Last Line comment marker