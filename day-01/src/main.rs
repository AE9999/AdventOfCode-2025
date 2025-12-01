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

fn solve(input: &Vec<String>) {
    let min = 0;
    let max = 99;

    let mut password = 0;
    let mut dial = 50;
    for line in input {
        let direction = if line.chars().next().unwrap() == 'L' { -1  } else { 1 };
        let raw_amount = &line[1..].parse::<i32>().unwrap();
        let amount = (raw_amount % 100) * direction;
        dial = dial + amount;
        if dial < min {
            dial =  dial + 100;
        } else if dial > max {
            dial = min + (dial - max)  - 1;
        }
        if dial == 0 {
            password += 1;
        }
    }

    println!("{} the actual password to open the door", password);
}

fn solve2(input: &Vec<String>) {
    let min = 0;
    let max = 99;
    
    let mut password = 0;
    let mut dial = 50;
    for line in input {
        let direction = if line.chars().next().unwrap() == 'L' { -1  } else { 1 };
        let raw_amount = &line[1..].parse::<i32>().unwrap();

        if raw_amount == &0 { continue }

        let amount = (raw_amount % 100) * direction;
        let mut rotations = raw_amount / 100;
        let org_dial = dial;

        let mut already_hit = false;

        dial = dial + amount;
        if dial < min {
            if org_dial != 0 {
                rotations += 1;
            }
            dial =  dial + 100;
        } else if dial > max {
            rotations += 1;
            already_hit = true;
            dial = min + (dial - max)  - 1;
        }

        if dial == 0 && !already_hit {
            rotations += 1;
        }

        password += rotations;

        println!("amount: {}, rotations {}, dial {} ..", amount, rotations, dial)

    }

    println!("{} the 0x434C49434B password to open the door", password);
}

fn read_input(filename: &String) -> io::Result<Vec<String>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|x| x.unwrap()).collect())
}

