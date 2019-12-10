// parses the puzzle input
fn parse_input(inp: &str) -> (u32, u32) {
    let toks: Vec<&str> = inp.split("-").collect();
    let lim_low = toks[0];
    let lim_up = toks[1];
    let lim_low = lim_low.parse::<u32>().unwrap();
    let lim_up = lim_up.parse::<u32>().unwrap();
    return (lim_low, lim_up);
}

// splits an int into a vector of digits, least significant to most
fn split_int(mut i: u32) -> Vec<u32> {
    let mut res = Vec::new();
    while i > 0 {
        res.push(i % 10);
        i /= 10;
    }
    return res;
}

// checks if an int is 6 digits
fn is_valid_6digit(pw: u32) -> bool {
    // It is a six-digit number.
    return pw > 99999 && pw < 1000000;
}

fn is_increasing(digits: &Vec<u32>) -> bool {
    let mut last_digit = &digits[0];
    for d in digits {
        // if d is > than last digit than we have a decrease
        if d > last_digit {
            return false;
        }
        last_digit = d;
    }
    // if we find no decreases we are only increasing
    return true;
}

fn has_2adj(digits: &Vec<u32>) -> bool {
    let mut last_digit = &digits[0];
    let mut adj_count = 1;
    for i in 1..digits.len() {
        let digit = &digits[i];
        if digit == last_digit { // check if the digit is the same as the last digit
            adj_count += 1;
        } else {
            // we can return early if we find an adj count of 2
            if adj_count == 2 {
                return true;
            }
            adj_count = 1; // we found a different digit so reset adj count
        }
        last_digit = digit;
    }
    // make sure we dont miss 2 adj digits at the end of the vector
    if adj_count == 2 {
        return true;
    }
    return false;
}

fn main() {
    let (lim_low, lim_up) = parse_input("357253-892942");

    let mut valid_count = 0;
    // The value is within the range given in your puzzle input.
    for n in lim_low + 1..lim_up {
        let digits = split_int(n);
        if is_valid_6digit(n) && is_increasing(&digits) && has_2adj(&digits) {
            valid_count += 1;
        }
    }
    println!("Valid Count: {}", valid_count);
}
