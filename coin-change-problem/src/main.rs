use clap::{Arg, App};

fn main() {
    let matches = App::new("Coin Change Problem")
        .version("1.0.0")
        .arg(Arg::new("change")
            .long("change")
            .about("The value of the change to be calculated.")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("coins")
            .long("coins")
            .about("The available coins to calculate the change value, separated by comma.")
            .takes_value(true)
            .required(true))
        .get_matches();

    let change: i32 = matches.value_of("change").unwrap().parse::<i32>().unwrap();
    let coins: Vec<i32> = matches.value_of("coins").unwrap()
                    .split(",")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect();

    println!("Change: {:?} | Coins: {:?}", change, coins);

    calculate_change(change, coins);
}

fn calculate_change(change: i32, coins: Vec<i32>) {
    let mut change_remainder: i32 = change;
    
    let mut coins_quantity: i32 = 0;
    
    let mut coins_occurrences: Vec<i32> = Vec::new();
    
    let coins_size: usize = coins.len();

    for i in (0..coins_size).rev() {
        let division = change_remainder / coins[i];

        if division > 0 {
            for _ in 0..division {
                coins_occurrences.push(coins[i]);
            }
        }

        change_remainder -= division * coins[i];

        coins_quantity += division;
    }

    println!("Result: {:?} | Size: {:?}", coins_occurrences, coins_quantity);
}