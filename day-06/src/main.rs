use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let puzzle = read_input(input)?;
    solve1(&puzzle);

    let puzzle_part2 = read_input_part2(input)?;
    solve2(&puzzle_part2);

    Ok(())
}

fn solve1(puzzle: &Puzzle) {
    let res : u64 =
        puzzle.problems.iter().map(|problem| {
            match problem.1 {
                '*' => { problem.0.iter().fold(1, |acc, value| {acc * value}) }
                '+' => { problem.0.iter().sum() }
                _ => panic!("Unexpected input")
            }
        }).sum();

    println!("{} is the grand total found by adding together all of the answers to the individual problems", res)
}

fn solve2(puzzle_part2: &PuzzlePart2) {
    let mut res: u64 = 0;

    let operator_indices: Vec<usize> =
        puzzle_part2.problem.last().unwrap().iter().enumerate()
            .filter(|(_, value)| {
                match value {
                    &&'+' => true,
                    &&'*' => true,
                    _ => false,
                }
            }).map(|(index, _)| index).collect();

    let number_vertical_length = puzzle_part2.problem.len() - 1;

    for i in 0..operator_indices.len() {
        let start_index = operator_indices[i];
        let mut end_index=
            if i == operator_indices.len() - 1 {
                puzzle_part2.problem[0].len() - 1
            } else {
                operator_indices[i + 1] - 2
            };

        let mut numbers:  Vec<u64> = Vec::new();

        while end_index >= start_index {
            let number_chars: String =
                (0..number_vertical_length).map(|y| puzzle_part2.problem[y][end_index])
                                              .filter(|x| x.is_digit(10))
                                              .collect();

            numbers.push(number_chars.parse::<u64>().unwrap());
            if end_index == 0 { break }; // corner case
            end_index -= 1;
        }

        let operator = puzzle_part2.problem.last().unwrap()[start_index];

        res +=
            match operator {
                '*' => { numbers.iter().fold(1, |acc, value| {acc * value}) }
                '+' => { numbers.iter().sum() }
                _ => panic!("Unexpected input")
            }

    }

    println!("{} is the grand total found by adding together all of the answers to the individual problems", res)
}

fn read_input(filename: &String) -> io::Result<Puzzle> {
    let file_in = File::open(filename)?;

    let mut raw_numbers: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for line in BufReader::new(file_in).lines().map(|x| x.unwrap()) {
        let mut it = line.split_whitespace().peekable();

        let count = it.clone().collect::<Vec<&str>>().len();
        if raw_numbers.len() == 0 {
            for _ in 0..count {
                raw_numbers.push(Vec::new());
            }
        }

        let is_number = it
            .peek()
            .map(|s| s.parse::<u64>().is_ok())
            .unwrap_or(false);

        if is_number {

            it.enumerate().for_each(|(index, value)| {
                raw_numbers[index].push(value.parse::<u64>().unwrap());
            });
        } else {
            it.for_each(|operator| operators.push(operator.chars().next().unwrap()))
        }
    }


    let problems =
        operators.iter()
                 .enumerate()
                 .map(|(index, operator)| {
                     (raw_numbers[index].clone(),
                      *operator)
                 })
                 .collect();

    Ok(Puzzle {
        problems,
    })
}

fn read_input_part2(filename: &String) -> io::Result<PuzzlePart2> {
    let file_in = File::open(filename)?;
    let mut problem: Vec<Vec<char>> =
        BufReader::new(file_in).lines()
                               .map(|x| x.unwrap().chars().collect())
                               .collect();

    let max_length = problem.iter().map(|line| line.len()).max().unwrap();

    for line_index in 0..problem.len() {
        let line = problem.get_mut(line_index).unwrap();
        let amount_needed = max_length - line.len();
        (0..amount_needed).for_each(|_| line.push(' '));
    }

    Ok(PuzzlePart2 {
        problem
    })
}

struct Puzzle {
    problems: Vec<(Vec<u64>, char)>
}

struct PuzzlePart2 {
    problem: Vec<Vec<char>>
}