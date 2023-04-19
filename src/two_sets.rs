#[macro_export]
macro_rules! input {
    () => {
        {   
            use std::io;
            let mut buf = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buf).expect("unable to read line");
            buf.strip_suffix("\r\n")
                .or(buf.strip_suffix("\n"))
                .unwrap_or(&buf)
                .to_string()
        }   
    };
}
 
#[macro_export]
macro_rules! ls2vec {
    ($x:expr) => {
        {
            let ls = $x.to_string();
            ls.split(" ")
                .map(|x| x.parse().unwrap())
                .collect()
        }
    };
}
 
#[allow(dead_code)]
fn nck(n: u64, k: u64) -> u64 {
    if k == 0 {
        return 1
    } else if k > n / 2 {
        return nck(n, n - k)
    } else {
        return n * nck(n - 1, k - 1) / k
    }
}
 
use std::collections::HashMap;
 
fn check(set: &Vec<i64>, idx: usize, sum: i64, visited: &mut HashMap<(usize, i64), i64>) -> i64 {
    if idx >= set.len() {
        if sum == 0 {
            return 1
        } else {
            return 0
        }
    }
 
    if visited.contains_key(&(idx, sum)) == false {
        let mut cnt = check(set, idx + 1, sum, visited);
        cnt += check(set, idx + 1, sum - set[idx], visited);
        visited.insert((idx, sum), cnt);
    }
 
    visited[&(idx, sum)]
}
 
fn get_subset(set: &Vec<i64>, sum: i64, visited: &mut HashMap<(usize, i64), i64>) -> Vec<i64> {
    let mut subset: Vec<i64> = Vec::new();
    let mut my_sum = sum.clone();
 
    for (idx, val) in set.iter().enumerate() {
        if check(set, idx + 1, my_sum - val, visited) > 0 {
            subset.push(val.clone());
            my_sum -= val;
        }
    }
 
    return subset
}
 
fn main() {
    let n: i64 = input!().parse().unwrap();
 
    if n * (n + 1) % 4 != 0 {
        println!("NO");
        return
    }
 
    let set: Vec<i64> = (1..=n).collect();
    let sum = n * (n + 1) / 4;
    let mut visited: HashMap<(usize, i64), i64> = HashMap::new();
 
    if check(&set, 0, sum, &mut visited) == 0 {
        println!("NO");
    } else {
        let subset_0 = get_subset(&set, sum, &mut visited);
        let subset_1: Vec<&i64> = set.iter().filter(|x| subset_0.contains(x) == false).collect();
 
        println!("YES");
        println!("{}", subset_0.len());
        for elem in subset_0.iter() {
            print!("{} ", elem);
        }
        println!("\n{}", subset_1.len());
        for elem in subset_1 {
            print!("{} ", elem);
        }
    }
}
