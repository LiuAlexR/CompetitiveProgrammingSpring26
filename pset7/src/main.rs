fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "G");
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
/// Spell Check
fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let timur = scanner.read_string();
        if n != 5 {
            answer.push_str(&"NO\n".to_string());
        } else {
            if timur.contains('T') && timur.contains('i') && timur.contains('m') && timur.contains('u') && timur.contains('r') {
                answer.push_str(&"YES\n".to_string());
            } else {
                answer.push_str(&"NO\n".to_string());
            }
        }
    }
    print!("{answer}");
}
/// Orac and Factors
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let nk = scanner.read_vec_usize();
        let mut n = nk[0];
        let mut k = nk[1];
        let mut smallest = n;
        for i in 2..n {
            if n % i == 0 {
                smallest = i;
                n = n + smallest;
                k = k - 1;
                break;
            }
        }
        while k > 0 {
            if smallest == 2 {
                n = n + 2 * k;
                break;
            }
            for i in 2..(smallest + 1) {
                if n % i == 0 {
                    smallest = i;
                    n = n + smallest;
                    k = k - 1;
                    break;
                }
            }
        }
        answer.push_str(&n.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Traffic Light
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let nc_string = scanner.read_string();
        let mut nc = nc_string.split(' ');
        let n = match nc.next() {
            Some(x) => match x.parse::<usize>() {
                Ok(y) => y,
                Err(_) => 0,
            },
            None => {
                0
            },
        };
        let c = match nc.next() {
            Some(x) => x.as_bytes()[0],
            None => b'e',
        };
        let s = scanner.read_string().as_bytes().to_vec();
        if c == b'g' {
            answer.push_str("0\n");
            continue;
        }
        let mut s_d: Vec<u8> = Vec::new();
        for i in 0..n {
            s_d.push(s[i]);
        }
        for i in 0..n {
            s_d.push(s[i]);
        }
        
        let mut max_wait = 0;
        let mut last_g = 0;
        let mut found_g = false;
        for i in (0..2 * n).rev() {
            if s_d[i] == b'g' {
                last_g = i;
                found_g = true;
            }
            if i < n && s_d[i] == c && found_g {
                let wait = last_g - i;
                if wait > max_wait {
                    max_wait = wait;
                }
            }
        }
        answer.push_str(&format!("{}\n", max_wait));
    }
    print!("{answer}");
}
/// Ahahahahahahahaha
fn p_d(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut zero_count = 0;
        let mut one_count = 0;
        for i in &a {
            if i == &0 {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        if zero_count >= one_count {
            answer.push_str(&zero_count.to_string());
            answer.push('\n');
            for _ in 0..zero_count {
                answer.push('0');
                answer.push(' ');
            }
            answer.push('\n');
        } else {
            if one_count % 2 == 0 {
                answer.push_str(&one_count.to_string());
                answer.push('\n');
                for _ in 0..one_count {
                    answer.push('1');
                    answer.push(' ');
                }
                answer.push('\n');
            } else {
                answer.push_str(&(one_count - 1).to_string());
                answer.push('\n');
                for _ in 1..one_count {
                    answer.push('1');
                    answer.push(' ');
                }
                answer.push('\n');
            }
        }
    }
    print!("{answer}");
}
/// Diamond Miner
fn p_e(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let mut miners: Vec<u64> = Vec::new();
        let mut diamonds: Vec<u64> = Vec::new();
        for _ in 0..(2*n) {
            let input = scanner.read_vec_i64();
            if input[0] == 0 {
                miners.push(input[1].abs_diff(0));
            } else {
                diamonds.push(input[0].abs_diff(0));
            }
        }
        miners.sort();
        diamonds.sort();
        let mut distance: f64 = 0.0;
        for i in 0..n {
            let d_2 = (miners[i] * miners[i] + diamonds[i] * diamonds[i]) as f64;
            distance = distance + d_2.sqrt();
        }
        answer.push_str(&distance.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
/// Little Alawn's Puzzle
fn p_f(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let b = scanner.read_vec_usize();
        let mut first_to_second_cycle: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            first_to_second_cycle.insert(a[i], b[i]);
        }
        let mut count_cycles = 0;
        for i in 1..(n+1) {
            if first_to_second_cycle.get(&i) == Some(&0) {
                continue;
            } else {
                count_cycles = count_cycles + 1;
                let mut cur_index = i;
                let mut next_index = first_to_second_cycle[&cur_index];
                first_to_second_cycle.insert(cur_index, 0);
                while first_to_second_cycle[&next_index] != 0 {
                    cur_index = next_index;
                    next_index = first_to_second_cycle[&next_index];
                    first_to_second_cycle.insert(cur_index, 0);
                }
            }
        }
        let val = pow(2, count_cycles, 1_000_000_007);
        answer.push_str(&val.to_string());
        answer.push('\n');
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
use std::{collections::HashSet, f64::consts::SQRT_2};
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