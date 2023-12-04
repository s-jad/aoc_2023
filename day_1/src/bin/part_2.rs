fn replace_digits(s: &str) -> String {
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let current_line = &s[index..];

        let result = if current_line.starts_with("one") {
            Some('1')
        } else if current_line.starts_with("two") {
            Some('2')
        } else if current_line.starts_with("three") {
            Some('3')
        } else if current_line.starts_with("four") {
            Some('4')
        } else if current_line.starts_with("five") {
            Some('5')
        } else if current_line.starts_with("six") {
            Some('6')
        } else if current_line.starts_with("seven") {
            Some('7')
        } else if current_line.starts_with("eight") {
            Some('8')
        } else if current_line.starts_with("nine") {
            Some('9')
        } else {
            let result = current_line.chars().next();
            result
        };
        index += 1;

        result
    });

    let new_str = line_iter.collect::<String>();

    new_str
}

fn find_digits(s: &str) -> Option<i32> {
    let line = replace_digits(s);
    let mut digits = String::new();

    for ld in line.chars() {
        if ld.is_digit(10) {
            if digits.len() == 0 {
                digits.push(ld);
            }
            for rd in line.chars().rev() {
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

fn part2(s: &str) -> i32 {
    let mut digits_arr = Vec::<i32>::new();

    for l in s.lines() {
        if let Some(d1) = find_digits(&l) {
            println!("digits => {:?}", d1);
            digits_arr.push(d1);
        }
    }

    digits_arr.iter().sum()
}

fn main() {
    let f = include_str!("../../input.text");
    let output = part2(f);
    dbg!(output);
}

#[cfg(test)]

mod tests {
    use super::*;

    use ::rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn line_test(#[case] line: &str, #[case] expected: i32) {
        assert_eq!(expected, find_digits(line).unwrap());
    }

    #[test]
    fn test_2() {
        let f = include_str!("../../test_part_2.text");
        assert_eq!(281, part2(f));
    }
}
