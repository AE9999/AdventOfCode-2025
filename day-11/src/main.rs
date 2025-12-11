use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;

    solve1(&input);

    solve2(&input);

    Ok(())
}

fn solve1(input: &HashMap<String, Vec<String>>) {
    let mut cache: HashMap<String, u128> = HashMap::new();
    let res = do_solve1(&"you".to_string(), input, &mut cache);
    println!("{res} different paths lead from you to out");
}

fn do_solve1(location: &String,
             input: &HashMap<String, Vec<String>>,
             cache: &mut HashMap<String, u128>) -> u128 {

    if cache.contains_key(location) {
        return *(cache.get(location).unwrap())
    }

    let answer =
        if location == "out" {
            1
        } else {
            input.get(location)
                 .map(|outputs| {
                     outputs.iter().map(|output| do_solve1(output, input, cache)).sum()
                 }).unwrap()
        };

    cache.insert(location.clone(), answer);

    answer
}

fn solve2(input: &HashMap<String, Vec<String>>) {
    let start = "svr".to_string();
    let end = "out".to_string();
    let dac = "dac".to_string();
    let fft = "fft".to_string();

    let mut cache: HashMap<String, u128> = HashMap::new();
    let from_start_to_dac_no_fft = calculate_from_to(&start, &dac, Some(&fft), &input, &mut cache);

    let mut cache: HashMap<String, u128> = HashMap::new();
    let from_dac_to_fft = calculate_from_to(&dac, &fft, None, &input, &mut cache);

    let mut cache: HashMap<String, u128> = HashMap::new();
    let from_fft_to_out_no_dac =  calculate_from_to(&fft, &end, Some(&dac), &input, &mut cache);

    let from_start_to_dac_to_fft_to_end =
        from_start_to_dac_no_fft * from_dac_to_fft * from_fft_to_out_no_dac;

    let mut cache: HashMap<String, u128> = HashMap::new();
    let from_start_to_fft_no_dac = calculate_from_to(&start, &fft, Some(&dac), &input, &mut cache);

    let mut cache: HashMap<String, u128> = HashMap::new();
    let from_fft_to_dac = calculate_from_to(&fft, &dac, None, &input, &mut cache);

    let mut cache: HashMap<String, u128> = HashMap::new();
    let from_dac_to_out_no_fft =  calculate_from_to(&dac, &end, Some(&fft), &input, &mut cache);

    let from_start_to_fft_to_dac_to_end =
        from_start_to_fft_no_dac * from_fft_to_dac * from_dac_to_out_no_fft;

    let res = from_start_to_dac_to_fft_to_end + from_start_to_fft_to_dac_to_end;

    println!("{res} of those paths visit both dac and fft");

}

fn calculate_from_to(location: &String,
                     end: &String,
                     forbidden: Option<&String>,
                     input: &HashMap<String, Vec<String>>,
                     cache: &mut HashMap<String, u128>) -> u128 {

    if cache.contains_key(location) {
        return *(cache.get(location).unwrap())
    }

    let answer =
        if location == end {
            1
        } else if forbidden.is_some() && forbidden.unwrap() == location {
            0
        } else {
            input.get(location)
                .map(|outputs| {
                    outputs.iter().map(|output|
                        calculate_from_to(output,
                                          end,
                                          forbidden,
                                          input,
                                          cache)).sum()
                }).unwrap_or(0)
        };

    cache.insert(location.clone(), answer);

    answer
 }

fn read_input(filename: &String) -> io::Result<HashMap<String, Vec<String>>> {
    let file_in = File::open(filename)?;

    Ok(BufReader::new(file_in).lines().map(|x| {
        let line = x.unwrap();
        let mut it = line.split(": ");
        let key = it.next().unwrap().to_string();
        let values = it.next().unwrap().split_whitespace().map(|x| x.to_string()).collect();
        (key, values)
    }).collect())
}