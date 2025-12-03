use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashSet};
use indicatif::ProgressBar;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;
    solve1(&input);
    solve2(&input);
    Ok(())
}

fn solve1(input: &Vec<Vec<u64>>) {
    let res = input.iter().map(|x|find_max_voltage_for_bank(x)).sum::<u64>();
    println!("{res} is the total output joltage");
}

fn solve2(input: &Vec<Vec<u64>>) {
    let bar = ProgressBar::new(input.len() as u64);

    let res = input
        .iter()
        .map(|x| {
            let r = find_max_voltage_for_bank_with_12_lamps(x);
            bar.inc(1);          // advance the bar
            r
        })
        .sum::<u64>();

    bar.finish(); // optionally .finish_with_message("done");

    println!("{res} is the new total output joltage");
}

fn find_max_voltage_for_bank_with_12_lamps(bank: &Vec<u64>) -> u64 {
    let mut heap: BinaryHeap<SelectionState> = BinaryHeap::new();

    let mut lower_bound: u64 = 0;

    // Start with an empty selection
    heap.push(SelectionState {
        current_index: 0,
        selected_indices: Vec::new(),
        bank,
    });

    while let Some(state) = heap.pop() {

        if state.current_upper_bound() < lower_bound {
            continue;
        }

        if state.full() {
            lower_bound = max(lower_bound, state.current_value());
            continue
        }

        // Decision to take next digit
        let next_state = state.take_current_digit();
        if next_state.can_complete() && next_state.current_upper_bound() > lower_bound {
            heap.push(next_state)
        }

        // Decision to not take next digit
        let next_state = state.skip_current_digit();
        if next_state.can_complete() && next_state.current_upper_bound() > lower_bound {
            heap.push(next_state)
        }

    }

    lower_bound
}

struct SelectionState<'a> {
    current_index: usize,
    selected_indices: Vec<usize>,
    bank: &'a Vec<u64>,
}

impl<'a> SelectionState<'a> {

    fn full(&self) -> bool {
        self.selected_indices.len() == 12
    }

    fn can_complete(&self) -> bool {
        let remaining_digits = self.bank.len() - self.current_index;
        remaining_digits + self.selected_indices.len() >= 12
    }

    fn current_value(&self) -> u64 {
        let mut radix: i32 = 11;
        let mut value: u64 = 0;
        for selected_index in self.selected_indices.iter() {
            value += self.bank.get(*selected_index).unwrap()
                        * 10u64.pow(radix as u32);
            radix -= 1;
        }
        value
    }

    fn current_upper_bound(&self) -> u64 {
        let mut radix: i32 = 11;
        let mut value: u64 = 0;
        for selected_index in self.selected_indices.iter() {
            value += self.bank.get(*selected_index).unwrap()
                * 10u64.pow(radix as u32);
            radix -= 1;
        }

        // TODO Highest remaining digits
        // find the remaining (radix + 1) digits from bank, with an index higher than or equal to
        // current_index and add them sorted lowest to highest in a tmp Vec<u64> name best_candidates

        if radix < 0 {
            return value;
        }

        // How many digits still need to be chosen
        let slots_left = (radix + 1) as usize;

        // Highest remaining digits:
        // find the remaining digits from bank, with an index >= current_index
        let mut remaining: Vec<u64> = self
            .bank
            .iter()
            .enumerate()
            .skip(self.current_index)
            .map(|(_, &v)| v)
            .collect();

        // sort ascending
        remaining.sort_unstable();

        // Take the largest `slots_left` digits.
        // They will still be in ascending order (lowest -> highest) in best_candidates.
        let len = remaining.len();
        let start = len - slots_left;
        let best_candidates: Vec<u64> = remaining[start..].to_vec();

        while radix >= 0 {
            value += best_candidates.get(radix as usize).unwrap()
                * 10u64.pow(radix as u32);
            radix -= 1;
        }

        value
    }

    fn take_current_digit(&self) -> Self {
        let mut next_selected_indices = self.selected_indices.clone();
        next_selected_indices.push(self.current_index);
        SelectionState {
            current_index: self.current_index + 1,
            selected_indices: next_selected_indices,
            bank: self.bank,
        }
    }

    fn skip_current_digit(&self) -> Self {
        SelectionState {
            current_index: self.current_index + 1,
            selected_indices: self.selected_indices.clone(),
            bank: self.bank,
        }
    }
}

impl<'a> Eq for SelectionState<'a> {}

impl<'a> PartialEq for SelectionState<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.current_upper_bound() == other.current_upper_bound()
    }
}

impl<'a> Ord for SelectionState<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.current_upper_bound().cmp(&other.current_upper_bound())
    }
}

impl<'a> PartialOrd for SelectionState<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_max_voltage_for_bank(bank: &Vec<u64>) -> u64 {
    index_pairs(&bank).map(|(i,j)| {
        bank.get(j).unwrap() * 10 +
        bank.get(i).unwrap()
    }).max().unwrap()
}

fn index_pairs(bank: &Vec<u64>) -> impl Iterator<Item = (usize, usize)> {
    let n = bank.len();

    (1..n).flat_map(move |i| {
        (0..i).map(move |j| (i, j))
    })
}

fn read_input(filename: &String) -> io::Result<Vec<Vec<u64>>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|line| {
        line.unwrap()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>()
    }) .collect())
}