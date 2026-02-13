use std::{io::Stdin, str::FromStr};

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
        "J" => p_j(scanner),
        _ => (),
    }
    /// General solution: Sort the string, while retaining a copy of the original. Then iterate through both, and add one to the solution if the characters do not match up
    /// Time Complexity: O(n*log(n)). Sorting takes O(nlog(n)), iterating through the arrays takes O(n). I do not count turning the string to a vector of bytes, because I do not know how fast that is
    fn p_a(scanner: &mut Scanner) -> () {
        let num_of_cases = scanner.read_number_generic::<usize>();
        let mut answer: String = String::new();
        for _ in 0..num_of_cases {
            let length_of_string = scanner.read_number_generic::<usize>();
            let the_string = scanner.read_string();
            let the_string_arr = the_string.as_bytes().to_vec();
            let mut the_string_sorted = the_string_arr.clone();
            let _ = the_string_sorted.sort();
            let mut counter: usize = 0;
            for i in 0..length_of_string {
                if the_string_arr[i] != the_string_sorted[i] {
                    counter = counter + 1;
                }
            }
            answer.push_str(&counter.to_string());
            answer.push('\n');
        }
        print!("{answer}");
    }
    /// General solution: Start at the back, move backwards. Once one pair is done, nothing done to a subsequent pair can affect that pair
    /// Time Complexity: O(n*lg(max N))?
    fn p_b(scanner: &mut Scanner) -> () {
        let mut answer: String = String::new();
        let t: u16 = scanner.read_number_generic::<u16>();
        for _ in 0..t {
            let n: usize = scanner.read_number_generic::<usize>();
            
            let mut the_vec = scanner.read_vec_generic::<i32>();
            if n == 1 {
                answer.push_str("0\n");
                continue;
            }
            let mut solved: bool = false;
            let mut counter: usize = 0;
            for element in (1..(n)).rev() {
                while the_vec[element - 1] >= the_vec[element]  && the_vec[element - 1] != 0{
                    the_vec[element - 1] = the_vec[element - 1] / 2;
                    counter = counter + 1;
                }
                if the_vec[element - 1] == the_vec[element] {
                    answer.push_str("-1");
                    answer.push('\n');
                    solved = true;
                    break;
                }
            }
            if !solved {
                answer.push_str(&counter.to_string());
                answer.push('\n');
            }
        }
        print!("{answer}");
    }
    /// Goal it to reduce it to 3x1 blocks because those only need one tile of paint, then add 1 for the extras.
    /// Time Complexity: Constant time
    fn p_c(scanner: &mut Scanner) -> () {
        let mut answer = String::new();
        let t: usize = scanner.read_usize();
        for i in 0..t {
            let the_vec = scanner.read_vec_generic::<usize>();
            if the_vec[0] * the_vec[1] % 3 == 0 {
                answer.push_str(&(the_vec[0] * the_vec[1] / 3).to_string());
                answer.push('\n');
            } else {
                answer.push_str(&(the_vec[0] * the_vec[1] / 3 + 1).to_string());
                answer.push('\n');
            }
        }
        print!("{answer}");

    }
    /// Just choose the k that is furthest away from the current point
    #[allow(unused_assignments)]
    fn p_d(scanner: &mut Scanner) -> () {
        let mut answer: String = String::new();
        let t: usize = scanner.read_usize();
        for _ in 0..t {
            let nk = scanner.read_vec_generic::<usize>();
            let n = nk[0];
            
            let k = nk[1];
            let the_string = scanner.read_string().as_bytes().to_vec();
            
            let mut counter = 0;
            let mut star_pos: Vec<usize> = Vec::new();
            for i in 0..(n) {
                if the_string[i] == 42 {
                    star_pos.push(i);
                }
            }
            if star_pos.len() == 1 {
                answer.push_str("1\n");
                continue;
            }
            let mut i = 0;
            let mut prev_x_pos = 0;
            counter = counter + 1;
            while prev_x_pos < star_pos.len() - 1 {
                i = prev_x_pos;
                while i + 1 < star_pos.len() && star_pos[i + 1] - star_pos[prev_x_pos] <= k {
                    i = i + 1;
                }
                prev_x_pos = i;
                counter = counter + 1;
                // if i == star_pos.len() - 1 {
                //     prev_x_pos = i;
                //     counter = counter + 1;
                //     break;
                // } else {
                //     if star_pos[i] - star_pos[prev_x_pos] <= k {
                //         if i + 1 == star_pos.len() - 1 && i - prev_x_pos != 1 {
                //             counter = counter + 1;
                //         }
                //         continue;
                //     } else {
                //         i = i - 1;
                //         prev_x_pos = i;
                //         counter = counter + 1;
                //         continue;
                //     }
                // }
            }
            answer.push_str(&counter.to_string());
            answer.push('\n');
        }
        print!("{answer}");
    }
    /// Keep track of both the min and the max of what a team could have. If the difference is greater than what could be reached, stop there
    fn p_e(scanner: &mut Scanner) -> () {
        let mut answer: String = String::new();
        let t: usize = scanner.read_usize();
        for _ in 0..t {
            let score_string: Vec<u8> = scanner.read_string().as_bytes().to_vec();
            let mut a_min_games_won: isize = 0;
            let mut a_max_games_won: isize = 0;
            let mut b_min_games_won: isize = 0;
            let mut b_max_games_won: isize = 0;
            let mut solved = false;
            for counter in 0..score_string.len() {
                if counter % 2 == 0 {
                    if score_string[counter] == 48 {
                        // Check if it is garuenteed that B wins
                        if b_max_games_won - a_min_games_won > ((score_string.len() - 2 - counter) / 2) as isize {
                            answer.push_str(&(counter+1).to_string());
                            answer.push('\n');
                            solved = true;
                            break;
                        } 
                    } else if score_string[counter] == 49 {
                        a_max_games_won = a_max_games_won + 1;
                        a_min_games_won = a_min_games_won + 1;
                        // Check if it is guarenteed that A wins
                        if a_max_games_won - b_min_games_won > ((score_string.len() - counter) / 2) as isize {
                            answer.push_str(&(counter+1).to_string());
                            answer.push('\n');
                            solved = true;
                            break;
                        } 
                    } else if score_string[counter] == 63 {
                        a_max_games_won = a_max_games_won + 1;
                        if b_max_games_won - a_min_games_won > ((score_string.len() - 2 - counter) / 2) as isize {
                            answer.push_str(&(counter+1).to_string());
                            answer.push('\n');
                            solved = true;
                            break;
                        } 
                        if a_max_games_won - b_min_games_won > ((score_string.len() - counter) / 2) as isize {
                            answer.push_str(&(counter+1).to_string());
                            answer.push('\n');
                            solved = true;
                            break;
                        } 
                    }
                } else if counter % 2 == 1 {
                    if score_string[counter] == 48 {
                        // Check if it is garuenteed that A wins
                        if a_max_games_won - b_min_games_won > ((score_string.len() - 1 - counter) / 2) as isize {
                            answer.push_str(&(counter+1).to_string());
                            answer.push('\n');
                            solved = true;
                            break;
                        } 
                    } else if score_string[counter] == 49 {
                        b_max_games_won = b_max_games_won + 1;
                        b_min_games_won = b_min_games_won + 1;
                        // Check if it is guarenteed that B wins
                        if b_max_games_won - a_min_games_won > ((score_string.len() - 1 - counter) / 2) as isize {
                            answer.push_str(&(counter+1).to_string());
                            answer.push('\n');
                            solved = true;
                            break;
                        } 
                    } else if score_string[counter] == 63 {
                        b_max_games_won = b_max_games_won + 1;
                        if a_max_games_won - b_min_games_won > ((score_string.len() - 1 - counter) / 2) as isize {
                            answer.push_str(&(counter+1).to_string());
                            answer.push('\n');
                            solved = true;
                            break;
                        } 
                        if b_max_games_won - a_min_games_won > ((score_string.len() - 1 - counter) / 2) as isize {
                            answer.push_str(&(counter+1).to_string());
                            answer.push('\n');
                            solved = true;
                            break;
                        } 

                    }
                }
            }
            if !solved {
                answer.push_str("10\n");
            }
        }
        print!("{answer}");
    }
    /// General strategy - select all from the right until left < d+1. Implemented through sliding window
    fn p_f(scanner: &mut Scanner) {
        let mut answer = String::new();
        let t: usize = scanner.read_usize();
        
        for _ in 0..t {
            let n = scanner.read_usize();
            let a = scanner.read_vec_generic::<i32>();
            let mut left = 0;
            for right in 0..n {
                while a[left] < (right - left + 1) as i32 {
                    left += 1;
                }
                
                let cost = right - left + 1;
                answer.push_str(&cost.to_string());
                answer.push(' ');
            }
            answer.push('\n');
        }
        print!("{}", answer);
    }
    fn p_g(scanner: &mut Scanner) -> () {
        let mut answer = String::new();
        let t: usize = scanner.read_usize();
        for _ in 0..t {
            let n: i32 = scanner.read_i32();
            let a = scanner.read_vec_generic::<isize>();
            let mut oddsum: isize = 0;
            let mut evensum: isize = 0;
            for i in 0..n {
                if i % 2 == 0 {
                    evensum = evensum + a[i as usize];
                } else {
                    oddsum = oddsum + a[i as usize];
                }
            }
            if oddsum < evensum {
                for i in 0..n {
                    if i % 2 == 0 {
                        answer.push_str(&a[i as usize].to_string());
                        answer.push(' ');
                    } else {
                        answer.push_str("1 ");
                    }
                }
            } else {
                for i in 0..n {
                    if i % 2 == 1 {
                        answer.push_str(&a[i as usize].to_string());
                        answer.push(' ');
                    } else {
                        answer.push_str("1 ");
                    }
                }
            }
            answer.push('\n');
        }
        print!("{answer}");
    }
    fn p_h(scanner: &mut Scanner) -> () {
        fn verify(the_string: &str) -> bool {
            let mut found = false;
            for i in 0..the_string.len()-7 {
                if &the_string[i..i+7] == "abacaba" {
                    if found {
                        return false;
                    }
                    found = true;
                }
            }
            return false;
        }
        let mut answer = String::new();
        let t: usize = scanner.read_usize();
    }
    fn p_i(scanner: &mut Scanner) -> () {
        
    }
    fn p_j(scanner: &mut Scanner) -> () {
        
    }
}
/// The scanner. It allows for readings of generic numbers, strings, and vectors of numbers
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