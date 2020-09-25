use std::io::{self, BufRead};

// input samples:
// 0 4 3 2 8 7 6 5
// 0 3 2 11 10 9 7 6 5 4
// 23 31 45 47 51 60 64 72 75
// 10 14 12 15 27 30 7 5 4 3 33 48 60 49 41 35 26 17

fn main() {
    let reader = io::stdin();

    let input: Vec<i32> = 
        reader.lock()
            .lines().next().unwrap().unwrap()
            .split_whitespace().map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

    println!("input: {:?}", input);

    calculate_longest_decreasing_subsequence(input);
}

fn calculate_longest_decreasing_subsequence(v: Vec<i32>) {
    let l: usize = v.len();
    
    let mut s: Vec<Vec<i32>> = Vec::new();
    
    let mut si: usize = 0;

    s.push(Vec::new());

    for i in 0..l {
        if s[si].is_empty() {
            s[si].push(v[i]);
            continue
        }

        if s[si][s[si].len() - 1] > v[i] {
            s[si].push(v[i]);
        } else {
            si = si + 1;
            s.push(Vec::new());
            s[si].push(v[i]);
        }
    }

    let mut subsequence: Vec<i32> = Vec::new();

    for x in s {
        if x.len() == 1 {
            continue;
        }

        if x.len() > subsequence.len() {
            subsequence = x;
        }
    }

    println!("subsequence: {:?}", subsequence);
}
