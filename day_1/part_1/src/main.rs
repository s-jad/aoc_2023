use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_digits(s: &str) -> Option<i32> {
    let mut digits = String::new();

    for ld in s.chars() {
        if ld.is_digit(10) {
            if digits.len() == 0 {
                digits.push(ld);
            }
            for rd in s.chars().rev() {
                if rd.is_digit(10) {
                    digits.push(rd);
                    break;
                }
            }
            break;
        }
    }

    if digits.len() == 2 {
        digits.parse::<i32>().ok()
    } else {
        None
    }
}

fn main() -> Result<(), std::io::Error> {
    let f = File::open("input.text")?;
    let reader = BufReader::new(f);
    let mut digits_arr = Vec::<i32>::new();

    for l in reader.lines() {
        let line = l?;
        if let Some(d1) = find_digits(&line) {
            digits_arr.push(d1);
        }
    }

    let res: i32 = digits_arr.iter().sum();

    println!("Result: {:?}", res);
    Ok(())
}
