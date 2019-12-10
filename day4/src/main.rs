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

fn is_pw_valid(pw: u32) -> bool {
    // It is a six-digit number.
    if !is_valid_6digit(pw) { 
        return false 
    }
    
    let digits = split_int(pw);
    let mut last_digit: &u32 = &(0);
    let mut has_adj_digits = false;
    for digit in digits.iter().rev() { // digits is in reverse order
        // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
        if digit < last_digit {
            return false;
        }
        // Two adjacent digits are the same (like 22 in 122345).
        if digit == last_digit {
            has_adj_digits = true;
        }
        last_digit = digit;
    }
    if !has_adj_digits {
        return false;
    }

    return true;
}

fn main() {
    let (lim_low, lim_up) = parse_input("357253-892942");
    println!("lower: {}; upper: {}", lim_low, lim_up);

    let mut valid_count = 0;
    // The value is within the range given in your puzzle input.
    for n in lim_low + 1..lim_up {
        if is_pw_valid(n) {
            //println!("{} is valid!", n);
            valid_count += 1;
        }
    }
    println!("Valid Count: {}", valid_count);
}
