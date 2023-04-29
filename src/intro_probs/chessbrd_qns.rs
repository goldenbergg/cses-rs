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

#[allow(dead_code)]
fn next_permutation(v: &mut Vec<u8>) -> bool {
    if let Some((i, k)) = v
        .windows(2)
        .enumerate()
        .rfind(|(_, w)| w[0] < w[1])
        .map(|(j, w)| {
            let k = v[j + 1..].iter().rposition(|x| w[0] < *x).unwrap_or(0);
            (j, k)
        })
    {
        v.swap(i, k + i + 1);
        v[i + 1..].reverse();
        return true;
    }

    false
}

fn place_qns(row: usize, slns: &mut u32, ranks: &mut [Vec<u8>; 8], anti: &mut [bool; 15], 
    main: &mut [bool; 15], files: &mut [bool; 8]) {
    if row == 8 {
        *slns += 1;
        return;
    }
    
    for sq in 0..=7 {
        let diff: i32 = (sq as i32) - (row as i32);
        if ranks[row][sq] == b'.' && !files[sq] && !anti[sq + row] && 
            !main[(diff + 7) as usize] {
            files[sq] = true;
            anti[sq + row] = true;
            main[(diff + 7) as usize] = true;
            
            place_qns(row + 1, slns, ranks, anti, main, files);
            
            files[sq] = false;
            anti[sq + row] = false;
            main[(diff + 7) as usize] = false;
        }
    }
}

fn main() {
    let mut ranks: [Vec<u8>; 8] = Default::default();
    let mut anti: [bool; 15] = [false; 15];
    let mut main: [bool; 15] = [false; 15];
    let mut files: [bool; 8] = [false; 8];
    let mut slns: u32 = 0;

    for row in 0..=7 {
        ranks[row] = input!().into_bytes();
    }

    place_qns(0, &mut slns, &mut ranks, &mut anti, &mut main, &mut files);

    println!("{}", slns);
}
    