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

fn main() {
    let n: i64 = input!().parse().unwrap();
    let nums: Vec<i64> = ls2vec!(input!());

    let ms_num: i64 = (n * (n + 1)) / 2 - nums.iter().sum::<i64>();
    
    print!("{}", ms_num);
}