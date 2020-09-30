use clap::{Arg, App};

// input samples:
// 0 4 3 2 8 7 6 5
// 0 3 2 11 10 9 7 6 5 4
// 23 31 45 47 51 60 64 72 75
// 10,14,12,15,27,30,7,5,4,3,33,48,60,49,41,35,26,17

fn main() {
    let matches = App::new("Longest Decreasing Subsequence Problem")
        .version("1.0.0")
        .arg(Arg::new("sequence")
            .long("sequence")
            .about("The sequence of values to be calculated.")
            .takes_value(true)
            .required(true))
        .get_matches();

    let sequence: Vec<i32> = matches.value_of("sequence").unwrap()
                .split(",")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

    println!("Sequence: {:?}", sequence);

    calculate_longest_decreasing_subsequence(sequence);
}

fn calculate_longest_decreasing_subsequence(sequence: Vec<i32>) {
    let size = sequence.len();

    let mut matrix = create_matrix(size);

    matrix[0].push(sequence[0]);

    for i in 1..size {
        for j in 0..i {
            if sequence[j] > sequence[i] && matrix[j].len() > matrix[i].len() {
                matrix[i] = matrix[j].to_vec();
            }
        }

        matrix[i].push(sequence[i]);
    }

    let subsequence: Vec<i32> = get_longest_vector(matrix);

    println!("Longest decreasing subsequence found: {:?}", subsequence);
}

fn create_matrix(size: usize) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for _ in 0..size {
        matrix.push(Vec::new());
    }

    return matrix
}

fn get_longest_vector(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut j: usize = 0;

    for i in 0..matrix.len() {
        if matrix[j].len() < matrix[i].len() {
            j = i;
        }
    }

    return matrix[j].to_vec();
}
