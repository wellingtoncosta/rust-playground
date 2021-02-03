use std::io;

fn main() {
    let mut input = String::new();
    println!("Input a char sequence to be evaluated...");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input = input.trim().to_string();
            let result = is_palindrome(trimmed_input);
            println!("Is palindrome? {}", result);
        }
        Err(error) => println!("Error: {}", error),
    }
}

fn is_palindrome(input: String) -> bool {
    let length = input.len();

    if length <= 1 {
        return false;
    }
    
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut j = length - 1;

    while i <= j {
        if chars[i] != chars[j] {
            return false;
        }

        i = i + 1;
        j = j - 1;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_palindrome() {
        let input = String::from("ana");
        let result = is_palindrome(input);
        println!("{:?}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn invalid_palindrome() {
        let input = String::from("abcd");
        let result = is_palindrome(input);
        assert_eq!(result, false);
    }
}
