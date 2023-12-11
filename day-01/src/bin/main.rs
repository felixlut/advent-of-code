use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("input1.txt");
    let output = part2(input);
    // dbg!(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let lines: std::str::Lines<'_> = input.lines();

    let mut results: Vec<String> = Vec::new();

    for line in lines {
        dbg!(line);
        let chars = line.chars();

        let mut numbers: Vec<char> = Vec::new();
        for char in chars {
            if char.is_digit(10) {
                numbers.push(char)
            }
        }
        let first: &char = numbers
            .first()
            .expect("Expects to see at least 1 number in the line");
        let last: &char = numbers
            .last()
            .expect("Expects to see at least 1 number in the line");

        let v: Vec<char> = vec![*first, *last];
        let s: String = v.iter().collect();

        results.push(s)
    }

    let results_numbers: Vec<i32> = results.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut tot_sum = 0;
    for res_num in results_numbers {
        dbg!(res_num);
        tot_sum += res_num
    }

    return tot_sum;
}

fn is_alpha_digit(input: &str, start_index: usize) -> char {
    let alpha_numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    for num in alpha_numbers {
        let target_len = num.0.len();

        if input.len() >= start_index + target_len
            && input[start_index..start_index + target_len] == num.0.to_string()
        {
            return num.1;
        }
    }

    return ' ';
}

fn part2(input: &str) -> i32 {
    let lines: std::str::Lines<'_> = input.lines();

    let mut results: Vec<String> = Vec::new();

    for line in lines {
        dbg!(line);

        let mut numbers: Vec<char> = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                numbers.push(char)
            } else {
                let alpha_number = is_alpha_digit(line, i);
                if alpha_number != ' ' {
                    numbers.push(alpha_number)
                }
            }
        }
        let first: &char = numbers
            .first()
            .expect("Expects to see at least 1 number in the line");
        let last: &char = numbers
            .last()
            .expect("Expects to see at least 1 number in the line");

        let v: Vec<char> = vec![*first, *last];
        let s: String = v.iter().collect();

        results.push(s)
    }

    let results_numbers: Vec<i32> = results.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut tot_sum = 0;
    for res_num in results_numbers {
        dbg!(res_num);
        tot_sum += res_num
    }

    return tot_sum;
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let result = part1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn part2_test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let result = part2(input);
        assert_eq!(result, 281);
    }
}
