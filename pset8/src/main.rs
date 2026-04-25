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
        "I" => p_i(scanner),
        "J" => p_j(scanner),
        "K" => p_k(scanner),
        "L" => p_l(scanner),
        _ => (),
    }
}
/// Journey
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let nabc = scanner.read_vec_usize();
        let n = nabc[0];
        let a = nabc[1];
        let b = nabc[2];
        let c = nabc[3];
        let full_cycles = n / (a + b + c);
        let kilos_left = n % (a + b + c);
        if kilos_left == 0 {
            answer.push_str(&(full_cycles * 3).to_string());
            answer.push('\n');
        } else if a >= kilos_left {
            answer.push_str(&(full_cycles * 3 + 1).to_string());
            answer.push('\n');
        } else if  (a + b) >= kilos_left {
            answer.push_str(&(full_cycles * 3 + 2).to_string());
            answer.push('\n');
        } else {
            answer.push_str(&(full_cycles * 3 + 3).to_string());
            answer.push('\n');
        }
    }
    print!("{answer}");
}
/// Sushi for Two
fn p_b(scanner: &mut Scanner) {
    let n = scanner.read_usize();
    let sushi = scanner.read_vec_usize();
    
    let mut current_block = 0;
    let mut prev_block = 0;
    let mut max_pieces = 0;
    let mut cur_type = sushi[0];

    for s in sushi {
        if s == cur_type {
            current_block += 1;
        } else {
            prev_block = current_block;
            current_block = 1;
            cur_type = s;
        }
        let valid_here = 2 * std::cmp::min(prev_block, current_block);
        if valid_here > max_pieces {
            max_pieces = valid_here;
        }
    }
    
    print!("{max_pieces}");
}
/// Save More Mice
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let nk = scanner.read_vec_usize();
        let x = scanner.read_vec_usize();
        let mut distance_from_hole = vec![0; nk[1]];
        for i in 0..nk[1] {
            distance_from_hole[i] = nk[0] - x[i];
        }
        distance_from_hole.sort();
        let mut cat_distance = nk[0];
        let mut idx = 0;
        let mut mice_num = 0;
        while cat_distance > distance_from_hole[mice_num] {
            cat_distance = cat_distance - distance_from_hole[mice_num];
            mice_num = mice_num + 1;
            if mice_num == nk[1] {
                break;
            }

        }
        answer.push_str(&(mice_num).to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Robin Hood in Town
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let mut a = scanner.read_vec_i64();
        
        if n <= 2 {
            answer.push_str("-1\n");
            continue;
        }

        a.sort();
        let sum: i64 = a.iter().sum();
    
        let target_wealth = a[n / 2];
        let x = 2 * (n as i64) * target_wealth - sum + 1;

        if x < 0 {
            answer.push_str("0\n");
        } else {
            answer.push_str(&format!("{}\n", x));
        }
    }
    print!("{answer}");
}
/// Brightness Begins
fn p_e(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let k = scanner.read_usize();
        let mut low = 1usize;
        let mut high = 2_000_000_000_000_000_000;
        let mut ans = high;

        while low <= high {
            let mid = low + (high - low) / 2;
            let s = (mid as f64).sqrt() as usize;

            let root = if (s + 1) * (s + 1) <= mid {
                s + 1
            } else if s * s > mid {
                s - 1
            } else {
                s
            };

            let on_bulbs = mid - root;

            if on_bulbs >= k {
                ans = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        answer.push_str(&ans.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Yet another Problem about Pairs Satisfying an Inequailty
fn p_f(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut working_idx: Vec<usize> = vec![0; 0];
        for i in 0..n {
            if a[i] < i + 1 {
                working_idx.push(i + 1);
            }
        }
        let mut total_pairs: i64 = 0;
        for &j_idx in &working_idx {
            let target = a[(j_idx - 1) as usize];
            let count = match working_idx.binary_search(&target) {
                Ok(pos) => pos,
                Err(pos) => pos,
            };
            
            total_pairs += count as i64;
        }
        answer.push_str(&format!("{}\n", total_pairs));
    }
    print!("{answer}");
}
/// Iva & Pav
fn p_g(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_generic::<usize>();
        let q = scanner.read_usize();
        let mut lookup: Vec<Vec<usize>> = vec![vec![0; (n.ilog2() + 1) as usize]; n + 1];
        for i in 0..n {
            lookup[i][0] = a[i];
        }
        let mut j: usize = 1;
        while (1 << j) <= n {
            let mut i = 0;
            while i + (1 << j) <= n {
                lookup[i][j] = lookup[i][j - 1] & lookup[i + (1 << (j - 1))][j - 1];
                i += 1;
            }
            j += 1;
        }
        for _ in 0..q {
            let lk = scanner.read_vec_usize();
            let l = lk[0] - 1;
            let k = lk[1];
            if a[l] < k {
                answer.push_str("-1 ");
                continue;
            }
            let mut low = l;
            let mut high = n - 1;
            let mut res = l;

            while low <= high {
                let mid = (low + high) / 2;
                
                let len = mid - l + 1;
                let log_len = len.ilog2() as usize;
                let range_and = lookup[l][log_len] & lookup[mid + 1 - (1 << log_len)][log_len];

                if range_and >= k {
                    res = mid;
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
            answer.push_str(&format!("{} ", res + 1));
        }
        answer.push('\n');
    }
    print!("{answer}");
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
/// Apple Tree
fn p_j(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let mut tree: Vec<Vec<usize>> = vec![];
        let mut num_descendants: Vec<usize> = vec![0; n];
        for _ in 0..n {
            let new_vec: Vec<usize> = vec![];
            tree.push(new_vec);
        }
        for _ in 0..(n-1) {
            let input = scanner.read_vec_usize();
            let u = input[0];
            let v = input[1];
            tree[u-1].push(v - 1);
            tree[v-1].push(u - 1);
        }
        calculate_leaves(0, usize::MAX, &tree, &mut num_descendants);
        let q = scanner.read_usize();
        for _ in 0..q {
            let xy = scanner.read_vec_usize();
            let x = xy[0] -1;
            let y = xy[1] - 1;
            let result = num_descendants[x] * num_descendants[y];
            answer.push_str(&result.to_string());
            answer.push('\n');
        }
    }
    print!("{answer}");
    fn calculate_leaves(u: usize, p: usize, adj: &Vec<Vec<usize>>, leaf_count: &mut Vec<usize>) {
        let mut is_leaf = true;
        for &v in &adj[u] {
            if v != p {
                is_leaf = false;
                calculate_leaves(v, u, adj, leaf_count);
                leaf_count[u] += leaf_count[v];
            }
        }
        if is_leaf {
            leaf_count[u] = 1;
        }
    }
}
/// Reorder the Array 
fn p_k(scanner: &mut Scanner) {
    let n = scanner.read_usize();
    let mut answer = String::new();
    // let a = scanner.read_vec_usize();
    // let mut counter: HashMap<usize, usize> = HashMap::new();
    // for i in &a {
    //     if counter.contains_key(i) {
    //         counter.insert(*i, counter[i] + 1);
    //     } else {
    //         counter.insert(*i, 1);
    //     }
    // }
    // let mut num = 0;
    // let mut temp = 0;
    // for i in counter {
    //     if i.1 > temp {
    //         num = num + temp;
    //         temp = i.1 - temp;
    //     } else {
    //         num = num + i.1;
    //         temp = temp - i.1;
    //     }
    // }
    // answer.push_str(&num.to_string());
    let mut a = scanner.read_vec_usize();
    a.sort();

    let mut count = 0;
    let mut i = 0;

    for j in 0..n {
        if a[j] > a[i] {
            count += 1;
            i += 1;
        }
    }
    answer.push_str(&count.to_string());
    print!("{answer}");
}
fn p_l(scanner: &mut Scanner) {
    let nm = scanner.read_vec_usize();
    let n = nm[0];
    let m = nm[1];
    let mut grid: Vec<Vec<usize>> = Vec::new();
    let mut count: usize = 0;
    for _ in 0..n {
        grid.push(scanner.read_vec_usize());
    }
    for i in 0..n {
        let mut black = 0;
        let mut white = 0;
        for j in 0..m {
            if grid[i][j] == 1 { black += 1; }
            else { white += 1; }
        }

        if black > 0 { count += (1usize << black) - 1; }
        if white > 0 { count += (1usize << white) - 1; }
    }

    for j in 0..m {
        let mut black = 0;
        let mut white = 0;
        for i in 0..n {
            if grid[i][j] == 1 { black += 1; }
            else { white += 1; }
        }
        if black > 0 { count += (1usize << black) - 1; }
        if white > 0 { count += (1usize << white) - 1; }
    }
    count = count - n * m;
    print!("{count}");
}
fn p_m(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        
    }
    print!("{answer}");
}
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
// Last Line comment marker