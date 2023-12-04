fn parse_game_string(game: &str) -> Vec<(&str, i32)> {
    let mut parts = game.split_terminator(&[';', ',', ':'][..]);
    let mut res = Vec::new();

    while let Some(part) = parts.next() {
        let tup: (&str, i32);
        let mut sub_parts = part.split_whitespace();

        let sub_part_1 = sub_parts.next().unwrap();
        let sub_part_2 = sub_parts.next().unwrap();

        if let Ok(num) = sub_part_1.parse::<i32>() {
            tup = (sub_part_2, num);
        } else {
            let num = sub_part_2.parse::<i32>().unwrap();
            tup = (sub_part_1, num);
        }
        res.push(tup);
    }
    res
}

fn check_game(game: &str) -> i32 {
    let parsed_game = parse_game_string(game);

    let red_max = ("red", 12);
    let green_max = ("green", 13);
    let blue_max = ("blue", 14);

    let mut game_id = 0;
    let mut impossible = false;
    for tup in parsed_game.iter() {
        match tup.0 {
            "Game" => {
                game_id = tup.1;
            }
            "red" => {
                if tup.1 > red_max.1 {
                    impossible = true;
                }
            }
            "green" => {
                if tup.1 > green_max.1 {
                    impossible = true;
                }
            }
            "blue" => {
                if tup.1 > blue_max.1 {
                    impossible = true;
                }
            }
            _ => {
                println!("Something has gone horribly wrong!");
            }
        }
    }

    if impossible {
        0
    } else {
        game_id
    }
}

fn main() {
    let f = include_str!("../../input.txt");
    let mut res_arr: Vec<i32> = Vec::new();

    for line in f.lines() {
        res_arr.push(check_game(line));
    }

    let res: i32 = res_arr.iter().sum();

    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 2)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        0
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        0
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 5)]
    fn test_games(#[case] game: &str, #[case] expected: i32) {
        assert_eq!(expected, check_game(game));
    }

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        [("Game", 1),("blue", 3), ("red", 4), ("red", 1), ("green", 2), ("blue", 6), ("green", 2)].to_vec()
    )]
    fn test_parse_game_string(#[case] game: &str, #[case] expected: Vec<(&str, i32)>) {
        assert_eq!(expected, parse_game_string(game));
    }

    //  #[test]
    //  fn test_1() {
    //      let f = include_str!("../../test.txt");
    //      assert_eq!(id_total(f), 8);
    //  }
}
