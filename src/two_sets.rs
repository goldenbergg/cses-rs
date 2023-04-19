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
 
fn main() {
    let n: i64 = input!().parse().unwrap();
 
    if n * (n + 1) % 4 != 0 {
        println!("NO");
        return
    }
    
    let part: i64 = n * (n + 1) / 4;
    let set: Vec<i64> = (1..=n).rev().collect();

    for (outer_idx, val) in set.iter().enumerate() {
        let mut sum: i64 = *val;
        let mut subset_0: Vec<i64> = Vec::new();
        let mut subset_1: Vec<i64> = Vec::new(); 

        if sum > part {
            sum -= val;
            subset_1.push(*val);
            continue;
        }

        subset_0.push(*val);
        let mut inner_idx = outer_idx + 1;

        while inner_idx < set.len() {
            sum += set[inner_idx];

            if sum > part {
                sum -= set[inner_idx];
                subset_1.push(set[inner_idx]);
            } else {
                subset_0.push(set[inner_idx]);
            }

            inner_idx += 1;
        }

        if sum < part {
            continue;
        } else {
            println!("YES");
            println!("{}", subset_0.len());

            for elem in subset_0 {
                print!("{} ", elem);
            }

            println!("\n{}", subset_1.len());

            for elem in subset_1 {
                print!("{} ", elem);
            }

            break;
        }
    }
}
