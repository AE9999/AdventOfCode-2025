use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use rayon::prelude::*;
use indicatif::{ProgressBar, ParallelProgressIterator};
use good_lp::{
    variable, Expression, ProblemVariables, Solution, SolverModel,
    solvers::highs::highs,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let machines = read_input(input)?;

    solve1(&machines);

    solve2(&machines);

    Ok(())
}

fn solve1(machines: &Vec<Machine>) {
    let bar = ProgressBar::new(machines.len() as u64);

    let res: i64 = machines
        .par_iter()                    // 🔥 parallel iterator
        .progress_with(bar.clone())    // tie progress bar to rayon
        .map(|machine| {
            solve_toggle_ilp(machine).unwrap()
        })
        .sum();                        // parallel sum

    bar.finish_with_message("done");

    println!("{res} is the fewest button presses required to correctly configure the indicator lights on all of the machines?");
}

fn solve2(machines: &Vec<Machine>) {
    let bar = ProgressBar::new(machines.len() as u64);
    let res: i64 = machines
        .par_iter()                    // 🔥 parallel iterator
        .progress_with(bar.clone())    // tie progress bar to rayon
        .map(|machine| {
            solve_exact(machine).unwrap()

        })
        .sum();                        // parallel sum
    bar.finish_with_message("done");

    println!("{res} is the fewest button presses required to correctly configure the joltage level counters on all of the machines?")
}

fn solve_toggle_ilp(machine: &Machine) -> Option<i64> {
    let n = machine.desired_end_state.len();
    let b = machine.button_2_switches.len();

    // Build reverse map: switch i -> buttons that toggle it
    let mut switch_to_buttons: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (bi, sws) in machine.button_2_switches.iter().enumerate() {
        for &i in sws {
            if i >= n {
                return None; // invalid input
            }
            switch_to_buttons[i].push(bi);
        }
    }

    let mut vars = ProblemVariables::new();

    // x_b ∈ {0,1}
    let x: Vec<_> = (0..b)
        .map(|bi| vars.add(variable().binary().name(format!("x_{bi}"))))
        .collect();

    // For each switch i: integer k_i >= 0
    // Optional but helpful: k_i <= deg_i/2 because sum a_{i,b} x_b <= deg_i
    let k: Vec<_> = (0..n)
        .map(|i| {
            let deg_i = switch_to_buttons[i].len() as f64;
            vars.add(variable().integer().min(0.0).max((deg_i / 2.0).floor()).name(format!("k_{i}")))
        })
        .collect();

    // Objective: minimize total presses
    let objective: Expression = x.iter().copied().sum();
    let mut model = vars.minimise(objective).using(highs); // <-- NOTE: highs, not highs()

    for i in 0..n {
        let mut sum_i = Expression::from(0.0);
        for &bi in &switch_to_buttons[i] {
            sum_i += x[bi];
        }
        let t_i = if machine.desired_end_state[i] { 1.0 } else { 0.0 };

        // sum_i - 2*k_i == t_i  (enforces parity/XOR)
        model = model.with((sum_i - 2.0 * k[i]).eq(t_i));
    }

    let solution = model.solve().ok()?;

    // Minimum #presses is sum of x_b (they're 0/1)
    let total: i64 = x.iter().map(|&v| solution.value(v).round() as i64).sum();
    Some(total)
}

fn solve_exact(machine: &Machine) -> Option<i64> {
    let n = machine.joltage.len();
    let b = machine.button_2_switches.len();

    // Precompute: for each switch i, which buttons affect it?
    let mut switch_to_buttons: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (bi, sws) in machine.button_2_switches.iter().enumerate() {
        for &i in sws {
            switch_to_buttons[i].push(bi);
        }
    }

    // Decision variables: x[b] are nonnegative integers
    let mut vars = ProblemVariables::new();
    let x: Vec<_> = (0..b)
        .map(|bi| vars.add(variable().integer().min(0.0).name(format!("x_{bi}"))))
        .collect();

    // Objective: minimize total presses
    let objective: Expression = x.iter().copied().sum();
    let mut model = vars.minimise(objective).using(highs);

    // Exact constraints: for each i, sum_{b affects i} x_b == joltage[i]
    for i in 0..n {
        let mut s_i = Expression::from(0.0);
        for &bi in &switch_to_buttons[i] {
            s_i += x[bi];
        }
        model = model.with(s_i.eq(machine.joltage[i] as f64));
    }

    // Solve
    let solution = model.solve().ok()?;

    // Minimal total presses is the objective value = sum_b x_b
    // (we recompute from variable values to avoid relying on solver APIs)
    let total: i64 = x
        .iter()
        .map(|&v| solution.value(v).round() as i64)
        .sum();

    Some(total)
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
