use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("input.txt");
    let output = part2(input);

    dbg!(input);
    dbg!(output);
}

fn get_num_from_string(input: &str) -> i32 {
    let number_as_string: String = input
        .chars()
        .filter(|c| c.is_numeric())
        .into_iter()
        .collect();

    return number_as_string.to_string().parse::<i32>().unwrap();
}

fn part1(input: &str) -> i32 {
    let colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let lines: std::str::Lines<'_> = input.lines();

    let mut results: Vec<i32> = Vec::new();

    'game_line: for line in lines {
        dbg!(line);

        let game_split: Vec<&str> = line.split(":").collect();

        let sets_split: Vec<&str> = game_split[1].split(";").collect();

        for set in sets_split {
            let color_in_sets: Vec<&str> = set.split(",").collect();
            for color_in_set in color_in_sets {
                for color in &colors {
                    if color_in_set.contains(color.0) {
                        let color_number = get_num_from_string(color_in_set);
                        if color_number > *color.1 {
                            dbg!("This game is not good");
                            continue 'game_line;
                        }
                    }
                }
            }
        }

        let game_number = get_num_from_string(game_split[0]);
        dbg!(game_number);

        results.push(game_number);
    }

    return results.iter().sum();
}

fn part2(input: &str) -> i32 {
    let lines: std::str::Lines<'_> = input.lines();

    let mut results: Vec<i32> = Vec::new();

    'game_line: for line in lines {
        dbg!(line);

        let mut colors_with_max = HashMap::new();
        colors_with_max.insert("red", 0);
        colors_with_max.insert("green", 0);
        colors_with_max.insert("blue", 0);

        let game_split: Vec<&str> = line.split(":").collect();

        let sets_split: Vec<&str> = game_split[1].split(";").collect();
        for set in sets_split {
            let color_in_sets: Vec<&str> = set.split(",").collect();
            for color_in_set in color_in_sets {
                for (color, color_max) in &mut colors_with_max {
                    if color_in_set.contains(color) {
                        let color_number = get_num_from_string(color_in_set);
                        if color_number > *color_max {
                            println!(
                                "Upping color: {} to new max: {}. Old max: {}",
                                color, color_number, color_max
                            );
                            *color_max = color_number;
                        }
                    }
                }
            }
        }

        results.push(colors_with_max.values().product());
    }

    return results.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn part1_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let result = part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let result = part2(input);
        assert_eq!(result, 2286);
    }
}
