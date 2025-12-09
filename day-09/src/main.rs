use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let points = read_input(input)?;

    solve(&points);

    Ok(())
}

fn solve(points: &Vec<Point>) {

    let mut pair_2_surface: Vec<((usize, usize), i64)> =
        (0..points.len()).flat_map(move |i| {
            (i+1..points.len()).map(move |j|  ((i,j), points[i].surface(&points[j])  ) )
        }).collect();

    pair_2_surface.sort_by(|a, b| b.1.cmp(&a.1));

    let res = pair_2_surface[0].1;
    
    println!("{res} is the largest area of any rectangle you can make.");
}

fn read_input(filename: &String) -> io::Result<Vec<Point>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|x| {
        let line = x.unwrap();
        let mut it = line.split(',');
        Point::new(it.next().unwrap().parse::<i64>().unwrap(),
                   it.next().unwrap().parse::<i64>().unwrap())
    } ).collect())
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn surface(&self, other: &Point)  -> i64 {
        let min_x = min(self.x, other.x);
        let max_x = max(self.x, other.x);
        let min_y = min(self.y, other.y);
        let max_y = max(self.y, other.y);

        ((max_x -  min_x) + 1) * ((max_y - min_y) + 1)
    }
}
