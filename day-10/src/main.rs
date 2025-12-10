use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use rayon::prelude::*;
use indicatif::{ProgressBar, ParallelProgressIterator};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let machines = read_input(input)?;

    // solve1(&machines);

    solve2(&machines);

    Ok(())
}

fn solve1(machines: &Vec<Machine>) {
    let bar = ProgressBar::new(machines.len() as u64);

    let res: usize = machines
        .par_iter()                    // 🔥 parallel iterator
        .progress_with(bar.clone())    // tie progress bar to rayon
        .map(|machine| {
            let start_state = vec![false; machine.desired_end_state.len()];
            let mut state_to_min: HashMap<Vec<bool>, usize> = HashMap::new();
            do_solve1(machine, start_state, &mut state_to_min, 0).unwrap()
        })
        .sum();                        // parallel sum

    bar.finish_with_message("done");

    println!("{res} is the fewest button presses required to correctly configure the indicator lights on all of the machines?");
}

fn solve2(machines: &Vec<Machine>) {
    let bar = ProgressBar::new(machines.len() as u64);
    let res: usize = machines
        .par_iter()                    // 🔥 parallel iterator
        .progress_with(bar.clone())    // tie progress bar to rayon
        .map(|machine| {

            let mut state: Vec<i64> = vec![0; machine.joltage.len()];

            let mut presses = 0usize;

            let mut state_to_min: HashMap<Vec<i64>, usize> = HashMap::new();

            // Pre process pushes we know that need to happen
            let mut jolt_to_buttons: HashMap<usize, Vec<usize>> = HashMap::new();

            for (button, switches) in machine.button_2_switches.iter().enumerate() {
                for jolt in switches {
                    jolt_to_buttons.entry(*jolt).or_default().push(button);
                }
            }

            let forced_buttons=
                jolt_to_buttons.iter()
                    .filter(|(jolt, buttons)| buttons.len() == 1);

            for (jolt, buttons) in forced_buttons {
                let amount = state[*jolt];
                for _ in 0..amount {
                    state = press_jolt_button(state, machine, buttons[0]);
                    presses += 1
                }
            }

            let mut best: Option<usize> = None;
            // &mut state_to_min,
            do_solve2(machine, state,  presses, &mut best, 0).unwrap()
        })
        .sum();                        // parallel sum
    bar.finish_with_message("done");

    println!("{res} is the fewest button presses required to correctly configure the joltage level counters on all of the machines?")
}


fn do_solve2(
    machine: &Machine,
    state: Vec<i64>,
    // state_to_min: &mut HashMap<Vec<i64>, usize>,
    presses: usize,
    best: &mut Option<usize>,
    min_index: usize,
) -> Option<usize> {
    // 1. Global upper bound: prune branches that can't beat current best
    if let Some(best_so_far) = *best {
        if presses >= best_so_far {
            return None;
        }
    }

    // 2. Per-state pruning: we've been here cheaper or equal before
    // if let Some(&known) = state_to_min.get(&state) {
    //     if presses >= known {
    //         return None;
    //     }
    // }

    // 3. Goal check: reached target joltage
    if state == machine.joltage {
        match best {
            Some(b) if presses < *b => *b = presses,
            None => *best = Some(presses),
            _ => {}
        }
        return Some(presses);
    }

    // record best known cost for this state
    // state_to_min.insert(state.clone(), presses);

    // 4. Generate successors, only using buttons with index >= min_index
    let mut options: Vec<((usize, Vec<i64>), i64)> =
        (min_index..machine.button_2_switches.len())
            .filter_map(|index_to_press| {
                let mut next_state = state.clone();
                for &to_increase in &machine.button_2_switches[index_to_press] {
                    next_state[to_increase] += 1;
                }

                target_distance(&next_state, &machine.joltage)
                    .map(|dist| ((index_to_press, next_state), dist))
            })
            .collect();

    // 5. Best-first by heuristic distance
    options.sort_by_key(|&(_, dist)| dist);

    // 6. Recurse; enforce non-decreasing indices via `min_index`
    options
        .into_iter()
        .filter_map(|((index_to_press, next_state), _)| {
            do_solve2(
                machine,
                next_state,
                // state_to_min,
                presses + 1,
                best,
                index_to_press, // allow same or larger indices
            )
        })
        .min()
}

fn press_jolt_button(state: Vec<i64>, machine: &Machine, button: usize) -> Vec<i64> {
    let mut next_state = state.clone();
    for joltage_index  in &machine.button_2_switches[button] {
        next_state[*joltage_index] = next_state[*joltage_index] + 1
    }
    next_state
}

fn do_solve1(machine: &Machine,
             state: Vec<bool>,
             state_to_min: &mut HashMap<Vec<bool>, usize>,
             presses: usize
) -> Option<usize> {

    // println!("{index_to_press}, {presses}");

    if state == machine.desired_end_state {
        return Some(presses)
    }

    let known_optimum = state_to_min.get(&state);
    if known_optimum.is_some() && known_optimum.unwrap() < &presses {
        return None
    }
    // state_to_min.insert(state.clone(), presses);


    let mut options: Vec<((usize, Vec<bool>), usize)> = (0..machine.button_2_switches.len()).map( |index_to_press| {
        let mut next_state = state.clone();
        for to_flip in &(machine.button_2_switches[index_to_press]) {
            next_state[*to_flip] = !next_state[*to_flip];
        }
        let hamming_distance = hamming_distance(&next_state, &(machine.desired_end_state));
        ((index_to_press, next_state), hamming_distance)
    }).collect();

    options.sort_by_key(|&(_, dist)| dist);

    options
        .into_iter()
        .filter_map(|((_, next_state), _)| {
            do_solve1(machine, next_state, state_to_min, presses + 1)
        })
        .min()
}

fn hamming_distance(a: &[bool], b: &[bool]) -> usize {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .filter(|(x, y)| x != y)
        .count()
}

fn target_distance(state: &[i64], target: &[i64]) -> Option<i64> {
    assert_eq!(state.len(), target.len());

    state
        .iter()
        .zip(target.iter())
        .try_fold(0_i64, |acc, (&s, &t)| {
            // if any state[i] + 1 > target[i] → impossible
            if s  > t {
                None
            } else {
                Some(acc + (t - s))
            }
        })
}

fn read_input(filename: &String) -> io::Result<Vec<Machine>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|x| {
        let line = x.unwrap();
        let mut it = line.split_whitespace();
        let desired_end_state = it.next()
                                                       .unwrap()
                                                       .chars().filter(|c| c != &'['
                                                                                  && c != &']')
                                                       .map(|c| {
                                                           match c  {
                                                               '.' => false,
                                                               '#' => true,
                                                               _ => panic!("Unexpected output")

                                                           }
                                                       }).collect::<Vec<bool>>();
        let mut button_2_switches : Vec<Vec<usize>> = Vec::new();
        let mut joltage: Vec<i64> = Vec::new();

        for s in it {
            let char = s.chars().next().unwrap();
            if char == '(' {
                let raw: String = s.chars().filter(|c| c != &'(' && c != &')').collect();
                let r = raw.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                button_2_switches.push(r);
            } else {
                let raw: String = s.chars().filter(|c| c != &'{' && c != &'}').collect();
                joltage = raw.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
            }
        }

        Machine {
            desired_end_state,
            button_2_switches,
            joltage
        }

    }).collect())
}


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Machine {
    desired_end_state: Vec<bool>,
    button_2_switches: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}
