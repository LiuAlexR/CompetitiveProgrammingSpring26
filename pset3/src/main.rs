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
        // "J" => p_j(scanner),
        _ => (),
    }
}
/// Isn't it just remove bridges leading to 1?
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let nk = scanner.read_vec_usize();
        if nk[1] >= nk[0] - 1 {
            answer.push_str("1\n");
        } else {
            answer.push_str(&nk[0].to_string());
            answer.push('\n');
        }
    }
    print!("{answer}");
}
/// Step 1 is to check easy cases. 
fn p_b(scanner: &mut Scanner) {
    let ns = scanner.read_vec_usize();
    let n = ns[0];
    let s = ns[1] - 1;
    let first = scanner.read_vec_usize();
    let second = scanner.read_vec_usize();
    if first[0] == 0 {
        print!("NO");
        return ();
    } else if first[s] == 0 && second[s] == 0 {
        print!("NO");
        return ();
    } else if first[s] == 1 {
        print!("YES");
        return ();
    } else {
        for i in s..n {
            if first[i] == 1 && second[i] == 1 {
                print!("YES");
                return ();
            }
        }
        print!("NO");
        return();
    }
}
fn p_c(scanner: &mut Scanner) {
    let n = scanner.read_usize();
    let mut answer: String = String::new();
    let p: Vec<usize> = scanner.read_vec_usize();
    for i in 0..n {
        let mut visited = vec![false; n];
        if visited[i] {
            answer.push_str(&(i+1).to_string());
            answer.push(' ');
        } else {
            let mut cur = i;
            while !visited[cur] {
                visited[cur] = true;
                cur = p[cur] - 1;
            }
            answer.push_str(&(cur+1).to_string());
            answer.push(' ');
        }
    }
    print!("{answer}");
}
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
     for _ in 0..t {
        let n =scanner.read_usize();
        let p = scanner.read_vec_usize();
        let c = scanner.read_string().as_bytes().to_vec();
        let mut num_of_black = vec![-2; n];
        for i in 0..n {
            if num_of_black[i] == -2 {
                let mut cur_pos = i;
                let mut cur_num_of_black = 0;
                while num_of_black[cur_pos] == -2 {
                    num_of_black[cur_pos] = -1;
                    if c[cur_pos] == b'0' {
                        cur_num_of_black = cur_num_of_black + 1;
                    }
                    cur_pos = p[cur_pos] - 1;
                }
                while num_of_black[cur_pos] == -1 {
                    num_of_black[cur_pos] = cur_num_of_black;
                    cur_pos = p[cur_pos] - 1;
                }
            }
        }
        answer.push_str(&num_of_black.iter().map(|x| x.to_string() + " ").collect::<String>());
        answer.push('\n');
     }
     print!("{answer}");
}
fn p_e(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let nm = scanner.read_vec_usize();
        let mut grid: Vec<Vec<u8>> = Vec::new();
        for _ in 0..nm[0] {
            grid.push(scanner.read_string().as_bytes().to_vec());
        }
        let mut found = false;
        'outer: for i in 0..nm[0] - 1 {
            for j in 0..nm[1] - 1 {
                if grid[i][j] + grid[i+1][j] + grid[i][j+1] + grid[i+1][j+1] - b'0' * 4 == 3 {
                    found = true;
                    answer.push_str(&"NO\n");
                    break 'outer; 
                }
            }
        }
        if !found {
            answer.push_str(&"YES\n");
        }
    }
    print!("{answer}");
}
fn p_f(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let s = scanner.read_string().as_bytes().to_vec();
        let mut s_vals: Vec<isize> =  Vec::new();
        for i in s {
            if i == b'W' {
                s_vals.push(-1);
            } else {
                s_vals.push(1);
            }
        }
        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 0..(n-1) {
            adj_list[a[i] - 1].push(i + 1);
        }
        let mut scores: Vec<isize> = vec![80000; n];
        f_rec(&adj_list, &mut scores, &s_vals, 0);
        let mut counter = 0;
        for i in scores {
            if i == 0 {
                counter = counter + 1;
            }
        }
        answer.push_str(&counter.to_string());
        answer.push('\n');

    }
    print!("{answer}");
    fn f_rec(adj_list: &Vec<Vec<usize>>, scores: &mut Vec<isize>, values: &Vec<isize>, node: usize) -> isize {
        if scores[node] != 80000 {
            return scores[node];
        }
        if adj_list[node].len() == 0 {
            scores[node] = values[node];
            return values[node];
        }
        let mut cur = values[node];
        for i in &adj_list[node] {
            cur = cur + f_rec(adj_list, scores, values, *i);
        }
        scores[node] = cur;
        return cur;
    }
}
fn p_g(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let abk = scanner.read_vec_usize();
        let a = scanner.read_vec_usize();
        let b = scanner.read_vec_usize();
        let mut boy_count: Vec<usize> = vec![0; abk[0]];
        let mut girl_count: Vec<usize> = vec![0; abk[1]];
        for i in 0..abk[2] {
            boy_count[a[i] - 1] += 1;
        }
        for i in 0..abk[2] {
            girl_count[b[i] - 1] += 1;
        }
        let mut total: isize = abk[2] as isize * (abk[2] as isize - 1) / 2 as isize;
        for i in boy_count {
            if i != 0 {
                let overlap = i * (i - 1) / 2;
                total = total - overlap as isize;
            }
        }
        for i in girl_count {
            if i != 0 {
                let overlap = i * (i - 1) / 2;
                total = total - overlap as isize;
            }
        }
        answer.push_str(&total.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
fn p_h(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();

    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();

        let inf = n as i32 + 7;
        let mut dp = vec![vec![inf; 2]; n + 1];

        dp[0][0] = 0;

        for i in 0..n {
            if dp[i][0] != inf {
                let cost = a[i] as i32;
                if dp[i+1][1] > dp[i][0] + cost {
                    dp[i+1][1] = dp[i][0] + cost;
                }
                if i + 1 < n {
                    let cost2 = (a[i] + a[i+1]) as i32;
                    if dp[i+2][1] > dp[i][0] + cost2 {
                        dp[i+2][1] = dp[i][0] + cost2;
                    }
                }
            }

            if dp[i][1] != inf {
                if dp[i+1][0] > dp[i][1] {
                    dp[i+1][0] = dp[i][1];
                }
                if i + 1 < n {
                    if dp[i+2][0] > dp[i][1] {
                        dp[i+2][0] = dp[i][1];
                    }
                }
            }
        }

        let result = dp[n][0].min(dp[n][1]);
        answer.push_str(&result.to_string());
        answer.push('\n');
    }
    print!("{}", answer);
}
fn p_i(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
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
}
// Last Line comment marker