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
        _ => (),
    }
}
/// Password
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_i32();
        let _ = scanner.read_string();
        let variations = (10 - n) * (9 - n) * 3;
        answer.push_str(&variations.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Minimise Oneness
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        answer.push('1');
        for _ in 0..n-1 {
            answer.push('0');
        }
        answer.push('\n');
    }
    print!("{answer}");
}
/// Longest Divisors Interval
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_number_generic::<u64>();
        let mut x = 1;
        loop {
            if n % x != 0 {
                break;
            }
            x += 1;
        }
        answer.push_str(&(x-1).to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Emordnilap
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let mut res: usize = n;
        let the_mod = 1000000007;
        res = (res * (res - 1)) % the_mod;
        for i in 1..(n + 1) {
            res = res * i;
            res = res % the_mod;
        }
        answer.push_str(&res.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Pasha and Stick
fn p_e(scanner: &mut Scanner) {
    let n = scanner.read_usize();
    if n % 2 == 1 {
        print!("{}", 0);
    } else {
        print!("{}", (((n / 2) - 1) / 2));
    }
}
/// JOE is on TV!
fn p_f(scanner: &mut Scanner) {
    let n = scanner.read_usize();
    let mut money: f64 = 0.0;
    for i in 1..(n+1) {
        money = money + 1 as f64 / i as f64;
    }
    print!("{money}");
}
/// Binary Cafe
fn p_g(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    let mut memo: HashMap<usize, usize> = HashMap::new();
    for _ in 0..t {
        let nk = scanner.read_vec_usize();
        let n = nk[0];
        let k = nk[1];
        let max_possible = count_options(&mut memo, &n);
        if k > 30 {
            answer.push_str(&max_possible.to_string());
            // answer.push_str(&(n+1).to_string());

            answer.push('\n');
        } else {
            // let ans: usize = std::cmp::min(1 << k, n+1);
            let ans: usize = std::cmp::min(1 << k, max_possible);
            answer.push_str(&ans.to_string());
            answer.push('\n');
        }
    }
    print!("{answer}");
    fn count_options(memo: &mut HashMap<usize, usize>, num: &usize) -> usize {
        match memo.get(num) {
            Some(value) => {
                return *value;
            },
            None => {
                if *num == 0 {
                    memo.insert(0, 1);
                    return 1;
                } else if *num == 1 {
                    memo.insert(1, 2);
                    return 2;
                } else {
                    let mut smallest_power: usize = 1;
                    let mut power: usize = 0;
                    while smallest_power < *num {
                        smallest_power = smallest_power << 1;
                        power = power + 1;
                    }
                    if *num == smallest_power - 1 {
                        memo.insert(*num, smallest_power);
                        return smallest_power;
                    } else {
                        let largest_power = smallest_power >> 1;
                        let val = count_options(memo, &(num - largest_power)) + largest_power;
                        memo.insert(*num, val);
                        return val;
                    }
                }
            },
        }
    }
}
/// Binomial Coefficients, Kind Of
fn p_h(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    let n = scanner.read_vec_usize();
    let k = scanner.read_vec_usize();
    let THE_MOD: usize = 1_000_000_007;
    for i in 0..t {
        let mut val: usize = 1;
        let mut power = k[i];
        let mut base = 2;
        while power > 0 {
            if power % 2 == 1 {
                val = (val * base) % THE_MOD;
            }
            base = (base * base) % THE_MOD;
            power = power / 2;
        }
        answer.push_str(&val.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Earning on Bets
fn p_i(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let k: Vec<usize> = scanner.read_vec_usize();

        let common_val: usize = 232792560; 
        
        let mut bets = Vec::new();
        let mut total_bet: usize = 0;

        for &multiplier in &k {
            let x_i = common_val / multiplier;
            bets.push(x_i);
            total_bet += x_i;
        }

        if total_bet < common_val {
            for i in 0..n {
                answer.push_str(&bets[i].to_string());
                answer.push(' ');
            }
            answer.push('\n');
        } else {
            answer.push_str(&"-1\n".to_string());
        }
    }
    print!("{answer}");
}
fn p_j(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
 
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