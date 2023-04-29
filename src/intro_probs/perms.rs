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

use std::collections::HashSet;

fn main() {
    let n: i64 = input!().parse().unwrap();

    for i in 1..(n + 1) {
        let mut inc = 2;
        let mut seen: HashSet<i64> = HashSet::new();
        let mut perm: Vec<i64> = Vec::new();
        let mut sum: i64 = 0;

        let mut curr_num: i64 = i;
        seen.insert(curr_num);
        sum += curr_num;
        perm.push(curr_num);

        while sum < (n * (n + 1) / 2) {
            let mut temp = curr_num - inc;

            if inc == n {
                break
            } else if temp <= 0 || seen.contains(&temp) == true {
                temp = curr_num + inc;

                if temp > n || seen.contains(&temp) == true {
                    inc += 1;
                } else {
                    curr_num += inc;
                    sum += curr_num;
                    seen.insert(curr_num);
                    perm.push(curr_num);
                    inc = 2;
                }
            } else {
                curr_num -= inc;
                sum += curr_num;
                seen.insert(curr_num);
                perm.push(curr_num);
                inc = 2;
            }
        }

        if sum == (n * (n + 1) / 2) {
            for x in perm.iter() {
                print!("{} ", x);
            }

            return
        }
    }

    print!("NO SOLUTION");
}