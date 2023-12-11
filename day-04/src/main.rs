use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("input.txt");
    let output = part2(input);

    dbg!(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut results: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let left: Vec<i32> = parts[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let right: Vec<i32> = parts[1]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        println!("Left: {:?}, Right: {:?}", left, right);

        let common_count: usize = right.iter().filter(|&n| left.contains(n)).count();
        println!("Common count: {}", common_count);

        if common_count > 0 {
            let common_count = common_count.saturating_sub(1);

            let result = 2i32.pow(common_count as u32);
            println!("Card value: {}", result);
            results.push(result);
        }
    }

    return results.iter().sum();
}

fn part2(input: &str) -> i32 {
    let mut line_numbers: HashMap<usize, i32> = HashMap::new();

    // We have 1 card for each game at the start
    for iter in 0..input.lines().count() {
        line_numbers.insert(iter, 1);
    }

    for (line_index, line) in input.lines().enumerate() {
        let parts: Vec<&str> = line.split('|').collect();
        let left: Vec<i32> = parts[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let right: Vec<i32> = parts[1]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        println!("Left: {:?}, Right: {:?}", left, right);

        let common_count: usize = right.iter().filter(|&n| left.contains(n)).count();
        println!("Common count: {}", common_count);

        if common_count > 0 {
            let increment_value = *line_numbers.get(&line_index).unwrap_or(&0);
            for n in line_index + 1..line_index + 1 + common_count {
                if let Some(value) = line_numbers.get_mut(&n) {
                    *value += increment_value;
                }
            }
        }

        let mut entries: Vec<_> = line_numbers.iter().collect();
        entries.sort_by_key(|&(k, _)| *k);

        for (line_number, value) in entries {
            println!("Line {}: {}", line_number, value);
        }
    }

    return line_numbers.values().sum();
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn part1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let result = part2(input);
        assert_eq!(result, 30);
    }
}
