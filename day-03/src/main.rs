fn main() {
    let input: &str = include_str!("input.txt");
    let output = part2(input);

    dbg!(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let engine: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = engine.len();
    let width = engine[0].len();

    let mut results: Vec<i32> = Vec::new();

    for row in &engine {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }

    let mut i = 0;
    let mut j = 0;
    while i < height {
        while j < width {
            let ch: char = engine[i][j];
            if ch.is_numeric() {
                // dbg!(format!("number found: {} at ({}, {})", ch, i, j));

                let start_index = j;
                let mut end_index: usize = j;
                'number_expansion: while end_index < width {
                    if engine[i][end_index].is_numeric() {
                        end_index += 1;
                    } else {
                        break 'number_expansion;
                    }
                }

                let number_as_string: String = engine[i][start_index..end_index].iter().collect();

                let mut saved = false;
                'adjecency_check_outer: for x in
                    i.saturating_sub(1)..=(i.saturating_add(1).min(height - 1))
                {
                    'adjecency_check_inner: for y in
                        start_index.saturating_sub(1)..(end_index.saturating_add(1).min(width - 1))
                    {
                        if x == i && start_index <= y && y < end_index {
                            continue 'adjecency_check_inner;
                        }

                        match engine[x][y] {
                            '.' => (),
                            _ => {
                                results.push(number_as_string.parse().ok().expect(
                                    "We have already checked that this should be a number",
                                ));
                                saved = true;
                                dbg!(format!(
                                    "Saving {}, saved because of {}({},{})",
                                    number_as_string, engine[x][y], x, y
                                ));
                                break 'adjecency_check_outer;
                            }
                        }
                    }
                }
                if !saved {
                    dbg!(format!("Discarding {}", number_as_string));
                }

                j = end_index;
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }

    return results.iter().sum();
}

fn expand_number(engine: Vec<Vec<char>>, i: usize, j: usize) -> (i32, usize, usize) {
    // Go left and right from the start_index and find all the chars that are numbers. Take them all and parse them as a number.
    // Return the result.
    let mut start_index: usize = j;
    let mut end_index: usize = j;
    while start_index > 0 && engine[i][start_index].is_numeric() {
        start_index = start_index.saturating_sub(1);
    }
    if !engine[i][start_index].is_numeric() {
        start_index = start_index.saturating_add(1);
    }

    while end_index < engine[i].len() && engine[i][end_index].is_numeric() {
        end_index = end_index.saturating_add(1);
    }
    if !engine[i][end_index].is_numeric() {
        end_index = end_index.saturating_sub(1);
    }

    let number_as_string: String = engine[i][start_index..=end_index].iter().collect();
    dbg!(format!("number_as_string: {}", number_as_string));

    return (
        number_as_string
            .parse()
            .ok()
            .expect("We have already checked that this should be a number"),
        start_index,
        end_index,
    );
}

fn part2(input: &str) -> i32 {
    let engine: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = engine.len();
    let width = engine[0].len();

    let mut results: Vec<i32> = Vec::new();

    for row in &engine {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }

    for (i, row) in engine.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '*' {
                dbg!(format!("gear found at ({}, {})", i, j));

                let mut gear_results: Vec<i32> = Vec::new();
                for x in i.saturating_sub(1)..=(i.saturating_add(1).min(height - 1)) {
                    let mut y = j.saturating_sub(1);
                    while y <= j.saturating_add(1).min(width - 1) {
                        if engine[x][y].is_numeric() {
                            let (num, _, end_index) = expand_number(engine.clone(), x, y);
                            gear_results.push(num);
                            y = end_index.saturating_add(1);
                        } else {
                            y += 1;
                        }
                    }
                }

                if gear_results.len() == 2 {
                    results.push(gear_results.iter().product());
                }
            }
        }
    }

    return results.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn part1_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part1_test_2() {
        let input = "....758..........................*......=.............@................................273......911...#....@666...+193......................
.............604....483..&144.859......807...-.........995..-218.770............37.512.*.........*.........................215...........117
......354..........*...............$........849.*.................................*.....242....469.&764.........................959*128.$...
";
        let result = part1(input);
        assert_eq!(result, 9626);
    }

    #[test]
    fn part1_test_3() {
        let input = "....................177......220...........500......111.46.........................413....*.....*................................/..........
500............+....../.167-....................959.............231..................=..192...........334.....781.....&..........122..151...
...*............969..........+878..136.../........*.#.............*.&...........226......................*...*......855..296..........%.....
";
        let result = part1(input);
        assert_eq!(result, 6729);
    }

    #[test]
    fn part2_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let result = part2(input);
        assert_eq!(result, 467835);
    }

    //     #[test]
    //     fn part2_test_2() {
    //         let input = "....758..........................*......=.............@................................273......911...#....@666...+193......................
    // .............604....483..&144.859......807...-.........995..-218.770............37.512.*.........*.........................215...........117
    // ......354..........*...............$........849.*.................................*.....242....469.&764.........................959*128.$...
    // ";
    //         let result = part2(input);
    //         assert_eq!(result, 9626);
    //     }

    //     #[test]
    //     fn part2_test_3() {
    //         let input = "....................177......220...........500......111.46.........................413....*.....*................................/..........
    // 500............+....../.167-....................959.............231..................=..192...........334.....781.....&..........122..151...
    // ...*............969..........+878..136.../........*.#.............*.&...........226......................*...*......855..296..........%.....
    // ";
    //         let result = part2(input);
    //         assert_eq!(result, 6729);
    //     }
}
