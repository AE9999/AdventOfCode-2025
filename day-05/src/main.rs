use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;
    solve1(&problem);
    solve2(&problem);

    Ok(())
}

fn solve1(problem: &Problem) {
    let res =
        problem.ingredients.iter()
                           .filter(|ingredient| {
                               problem.is_in_ranges(**ingredient)
                           }).count();
    println!("{} of the available ingredient IDs are fresh", res);
}

fn solve2(problem: &Problem) {
    let mut work_board = problem.ranges.clone();

    loop {
        let mut merged = false;

        for range in work_board.iter() {
            let overlapping_indices: Vec<usize> =
                find_overlapping_indices(range, &work_board);

            if overlapping_indices.len() > 1 {
                let overlapping_ranges: Vec<(u64, u64)> =
                    remove_overlapping_ranges(&overlapping_indices,
                                              &mut work_board);
                work_board.push(calculate_new_range(&overlapping_ranges));
                merged = true;
                break;
            }
        }

        if !merged {
            break;
        }
    }

    let res: u64 = work_board.iter().map(|(lb, ub)| { ub - lb + 1 }
    ).sum();
    println!("{} many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges",
             res);
}

fn find_overlapping_indices(range: &(u64, u64), distinct_ranges: &Vec<(u64, u64)>) -> Vec<usize> {
    distinct_ranges.iter()
                   .enumerate()
                   .filter(|(_, other)| {
                       (range.0 >= other.0 && range.0 <= other.1)
                       || (range.1 >= other.0 && range.1 <= other.1)
                   })
                   .map(|(index, _)| index)
                   .collect()
}

fn remove_overlapping_ranges(overlapping_indices: &Vec<usize>,
                             distinct_ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut indices = overlapping_indices.clone();
    indices.sort_unstable(); // ascending
    let mut removed = Vec::with_capacity(indices.len());

    for i in indices.into_iter().rev() {
        removed.push(distinct_ranges.remove(i));
    }

    removed
}

fn calculate_new_range(overlapping_ranges: &Vec<(u64, u64)>) -> (u64, u64) {
    (overlapping_ranges.iter().map(|range| range.0).min().unwrap(),
     overlapping_ranges.iter().map(|range| range.1).max().unwrap())
}

fn read_input(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    let mut parsing_ranges = true;

    for line in BufReader::new(file_in).lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            parsing_ranges = false;
            continue
        }
        if parsing_ranges {
            let mut it = line.split('-');
            ranges.push((it.next().unwrap().parse::<u64>().unwrap(),
                               it.next().unwrap().parse::<u64>().unwrap()))
        } else {
            ingredients.push(line.parse::<u64>().unwrap())
        }
    }

    Ok(Problem {
        ranges,
        ingredients
    })
}

struct Problem {
    ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>
}

impl Problem {
    fn is_in_ranges(&self, value: u64) -> bool {
        self.ranges.iter().any(|range| value >= range.0 && value <= range.1)
    }
}

