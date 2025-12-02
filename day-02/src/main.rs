use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;

    solve(&input);
    solve2(&input);

    Ok(())
}

fn solve2(input: &Vec<(i64, i64)>) {
    let res =
        input.iter().map(|x| { handle_range2(x).iter().sum::<i64>() } ).sum::<i64>();
    println!("{} do you get if you add up all of the invalid IDs", res);
}


fn solve(input: &Vec<(i64, i64)>) {
    let res =
        input.iter().map(|x| { handle_range(x).iter().sum::<i64>() } ).sum::<i64>();
    println!("{} do you get if you add up all of the invalid IDs", res);
}

fn handle_range2(range: &(i64, i64)) -> Vec<i64> {
    let mut rvalue = Vec::new();

    for id in range.0..range.1+1 {
        if is_invalid2(id) {
            rvalue.push(id)
        }
    }

    rvalue
}

fn handle_range(range: &(i64, i64)) -> Vec<i64> {
    let mut rvalue = Vec::new();

    for id in range.0..range.1+1 {
        if is_invalid(id) {
            rvalue.push(id)
        }
    }
    rvalue
}

fn is_invalid(id: i64) -> bool {
    let s = id.to_string();
    if s.len() % 2 == 1 {
        return false
    }

    let window_size = s.len() / 2;

    let s1 = &s[0..window_size];
    let s2 = &s[window_size..];

    s1 == s2
}

fn is_invalid2(id: i64) -> bool {
    let s = id.to_string();

    let max_window_size = s.len() / 2;

    for window_size in 1..=max_window_size {
        if s.len() % window_size != 0 { continue }
        let n = s.len() / window_size;
        let chunks: Vec<&str> = (0..n)
            .map(|i| {
                let start = i * window_size;
                let end = start + window_size;
                &s[start..end]
            })
            .collect();

        let first = chunks.first().unwrap();

        if chunks.iter().all(|c| c == first) {
            return true;
        }
    }
    false
}

fn read_input(filename: &String) -> io::Result<Vec<(i64, i64)>> {
    let file_in = File::open(filename)?;
    let line = BufReader::new(file_in).lines()
                                             .next().map(|x| x.unwrap())
                                             .unwrap();
    let rvalue = line.split(',').map(|entry| {
        let mut x = entry.split('-');

        (x.next().unwrap().parse::<i64>().unwrap(),
         x.next().unwrap().parse::<i64>().unwrap())
    }).collect();
    Ok(rvalue)
}

