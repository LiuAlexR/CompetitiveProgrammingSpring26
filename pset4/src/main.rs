fn main() {
    let mut scanner = Scanner::new();
    problems(&mut scanner, "I");
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

fn p_a(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let s = scanner.read_vec_usize();
        let mut max = 0;
        let mut second_max = 0;
        for i in &s {
            if *i > max {
                let temp = max;
                max = *i;
                second_max = temp;
            } else if *i > second_max {
                second_max = *i;
            }
        }
        for i in &s {
            if *i == max {
                let dif = *i as i32- second_max as i32;
                answer.push_str(&dif.to_string());
                answer.push(' ');
            } else {
                let dif = *i as i32 - max as i32;
                answer.push_str(&dif.to_string());
                answer.push(' ');
            }
            
        }
        answer.push('\n');
        
    }
    print!("{answer}");
}
fn p_b(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer: String = String::new();
    for _ in 0..t {
        let nc = scanner.read_vec_usize();
        let a = scanner.read_vec_usize();
        let mut orbit_to_number_of_planets: HashMap<usize, usize> = HashMap::new();
        for i in &a {
            if orbit_to_number_of_planets.contains_key(i) {
                orbit_to_number_of_planets.insert(*i, orbit_to_number_of_planets.get(i).unwrap() + 1);
            } else {
                orbit_to_number_of_planets.insert(*i, 1);
            }
        }
        let mut total = 0;
        for i in orbit_to_number_of_planets {
            if i.1 < nc[1] {
                total = total + i.1;
            } else {
                total = total + nc[1];
            }

        }
        answer.push_str(&total.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
fn p_c(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let nq = scanner.read_vec_usize();
        let n = nq[0];
        let q = nq[1];
        let a = scanner.read_vec_usize();
        
        let mut a_parity = vec![0; n]; 
        
        a_parity[0] = a[0] % 2;
        for i in 1..n {
            a_parity[i] = (a[i] + a_parity[i-1]) % 2;
        }

        for _ in 0..q {
            let lrk = scanner.read_vec_usize();
            let l = lrk[0];
            let r = lrk[1];
            let k = lrk[2];

            let mut parity_of_section = 0;
            if l == 1 {
                parity_of_section = a_parity[r - 1];
            } else {
                parity_of_section = (a_parity[r - 1] + 2 - a_parity[l - 2]) % 2;
            }

            let num_of_elements = (r - l + 1) % 2;
            let new_section_parity = (num_of_elements * (k % 2)) % 2;
            
            let total_parity = a_parity[n - 1];
            let final_parity = (total_parity + 2 - parity_of_section + new_section_parity) % 2;

            if final_parity == 1 {
                answer.push_str("YES\n");
            } else {
                answer.push_str("NO\n");
            }
        }
    }
    print!("{answer}");
}
fn p_d(scanner: &mut Scanner) {
    let nm = scanner.read_vec_usize();
    let mut answer: String = String::new();
    let a = scanner.read_vec_usize();
    let mut forward_damage: Vec<usize> = vec![0; nm[0]]; // item in 1 is fall damage moving from pillar 0 to pillar 1
    let mut backward_damage: Vec<usize> = vec![0; nm[0]];
    for i in 1..nm[0] {
        if a[i - 1] >= a[i] {
            forward_damage[i] = a[i - 1] - a[i];
        } else {
            backward_damage[i] = a[i] - a[i - 1];
        }
    }
    let mut forward_prefix: Vec<usize> = vec![0; nm[0]];
    let mut backward_prefix: Vec<usize> = vec![0; nm[0]];
    backward_prefix[nm[0] - 1] = backward_damage[nm[0] - 1];
    for i in 1..nm[0] {
        forward_prefix[i] = forward_prefix[i - 1] + forward_damage[i];
    }
    for i in 1..nm[0] {
        backward_prefix[i] = backward_prefix[i - 1] + backward_damage[i];
    }
    for i in 0..nm[1] {
        let st = scanner.read_vec_usize();
        let s = st[0] - 1;
        let t = st[1] - 1;
        if t > s {
            let final_answer = forward_prefix[t] - forward_prefix[s];
            answer.push_str(&final_answer.to_string());
            answer.push('\n');
        } else if s == t {
            answer.push_str(&"0\n");
        } else {
            let final_answer = backward_prefix[s] - backward_prefix[t];
            answer.push_str(&final_answer.to_string());
            answer.push('\n');
        }
    }
    print!("{answer}");
}
fn p_e(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let nm = scanner.read_vec_usize();
        let mut arr: Vec<Vec<usize>> = Vec::new();
        for _ in 0..nm[0] {
            arr.push(scanner.read_vec_usize());
        }
        if nm[0] == 1 && nm[1] == 1 {
            answer.push_str(&arr[0][0].to_string());
            answer.push('\n');
        } else {
            for i in 0..nm[0] {
                for j in 0..nm[1] {

                    let top = if i == 0 {0 as usize} else {
                        match arr.get(i - 1) {
                            Some(x) => x[j],
                            None => 0,
                        }
                    };
                    let bottom = match arr.get(i + 1) {
                        Some(x) => x[j],
                        None => 0,
                    };
                    let left = if j == 0 {0 as usize} else {
                        match arr[i].get(j-1) {
                            Some(x) => *x,
                            None => 0,
                        }
                    };
                    let right = match arr[i].get(j+1) {
                        Some(x) => *x,
                        None => 0,
                    };
                    let num = arr[i][j];
                    let mut max = 0;
                    if top > max {
                        max = top;
                    }
                    if bottom > max {
                        max = bottom;
                    }
                    if left > max {
                        max = left;
                    }
                    if right > max {
                        max = right;
                    }
                    if num > max {
                        arr[i][j] = max;
                    }
                }
            }
            for i in arr {
                answer.push_str(&i.iter().map(|x| x.to_string() + " ").collect::<String>());
                answer.push('\n');
            }
        }
    }
    print!("{answer}");
}
fn p_f(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        // let n = scanner.read_usize();
        // if n % 2 == 1 {
        //     answer.push_str(&((n + 1) / 2).to_string());
        //     answer.push('\n');
        //     for i in 1..(n) {
        //         if i <= (n - 1) / 2 {
        //             answer.push_str(&i.to_string());
        //             answer.push(' ');
        //             answer.push_str(&(n - i + 1).to_string());
        //             answer.push('\n');
        //         } else {
        //             answer.push_str(&((n + 1) / 2).to_string());
        //             answer.push(' ');
        //             answer.push_str(&((n + 1) / 2).to_string());
        //             answer.push('\n');
        //         }
        //     }
        // } else {
        //     answer.push_str(&((n) / 2).to_string());
        //     answer.push('\n');
        //     for i in 2..(n) {
        //         if i <= (n) / 2 {
        //             answer.push_str(&i.to_string());
        //             answer.push(' ');
        //             answer.push_str(&(n - i + 2).to_string());
        //             answer.push('\n');
        //         } else {
        //             answer.push_str(&((n + 2) / 2).to_string());
        //             answer.push(' ');
        //             answer.push_str(&((n + 2) / 2).to_string());
        //             answer.push('\n');
        //         }
        //     }
        //     answer.push_str(&(1).to_string());
        //     answer.push(' ');
        //     answer.push_str(&((n + 2) / 2).to_string());
        //     answer.push('\n');
        // }
        let n = scanner.read_usize();
        let mut v: Vec<usize> = (1..=n).collect();
        answer.push_str(&"2\n");

        for _ in 0..n-1 {
            let a = v.pop().unwrap();
            let b = v.pop().unwrap();
            answer.push_str(&a.to_string());
            answer.push(' ');
            answer.push_str(&b.to_string());
            answer.push('\n');
            if (a + b) % 2 == 0 {
                v.push((a + b) / 2);
            } else {
                v.push((a + b + 1) / 2);
            }
        }
    }
    print!("{answer}");
}
fn p_g(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_usize();
        let mut min_arr = vec![0 as usize; n];
        min_arr[n-1] = a[n-1];
        for i in (0..n - 1).rev() {
            min_arr[i] = std::cmp::min(min_arr[i+1], a[i]);
        }
        let mut counter = 0;
        for i in 0..(n-1) {
            if a[i] > min_arr[i+1] {
                counter = counter + 1;
            }
        }
        answer.push_str(&counter.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
fn p_h(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let nm = scanner.read_vec_i64();
        let n = nm[0] as usize;
        let m = nm[1] as usize;
        let mut a = scanner.read_vec_i64();
        let mut b = scanner.read_vec_i64();
        
        a.sort();
        b.sort();

        let mut a_left = 0;
        let mut a_right = n - 1;
        let mut b_left = 0;
        let mut b_right = m - 1;
        
        let mut total_difference: i64 = 0;

        for _ in 0..n {
            let diff1 = (a[a_left] - b[b_right]).abs();
            let diff2 = (a[a_right] - b[b_left]).abs();

            if diff1 >= diff2 {
                total_difference += diff1;
                a_left += 1;
                if b_right > 0 { b_right -= 1; }
            } else {
                total_difference += diff2;
                if a_right > 0 { a_right -= 1; }
                b_left += 1;
            }
        }
        answer.push_str(&total_difference.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
fn p_i(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    for _ in 0..t {
        let n = scanner.read_usize();
        let s = scanner.read_string().as_bytes().to_vec();
        let mut counter = 1;
        for i in 1..(n-1) {
            if s[i-1] != s[i+1] {
                counter = counter + 1;
            }
        }
        answer.push_str(&counter.to_string());
        answer.push('\n');
    }
    print!("{answer}");
}
fn p_j(scanner: &mut Scanner) {
    let t = scanner.read_usize();
    let mut answer = String::new();
    
    for _ in 0..t {
        let n = scanner.read_usize();
        let a = scanner.read_vec_i64();
        
        let sum: i64 = a.iter().sum();
        
        if (2 * sum) % (n as i64) != 0 {
            answer.push_str("0\n");
            continue;
        }
        
        let target = (2 * sum) / (n as i64);
        let mut map = HashMap::new();
        let mut count: i64 = 0;
        
        for &x in &a {
            let complement = target - x;
            if let Some(&occ) = map.get(&complement) {
                count += occ as i64;
            }
            *map.entry(x).or_insert(0) += 1;
        }
        
        answer.push_str(&count.to_string());
        answer.push('\n');
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