use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let points = read_input(input)?;

    solve1(&points);

    solve2(&points);

    Ok(())
}

fn solve1(points: &Vec<Point>) {

    let mut pair_2_surface: Vec<((usize, usize), i64)> =
        (0..points.len()).flat_map(move |i| {
            (i+1..points.len()).map(move |j|  ((i,j), points[i].surface(&points[j])  ) )
        }).collect();

    pair_2_surface.sort_by(|a, b| b.1.cmp(&a.1));

    let res = pair_2_surface[0].1;

    println!("{res} is the largest area of any rectangle you can make.");
}

fn solve2(points: &Vec<Point>) {
    let mut x_ranges: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    let mut y_ranges: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

    let mut new_points = points.clone();
    new_points.push(new_points[0].clone());

    for i in 1..new_points.len() {
        let previous = &new_points[i-1];
        let current = &new_points[i];
        if previous.x == current.x {
            let min_y = min(previous.y, current.y);
            let max_y = max(previous.y, current.y);
            x_ranges.entry(current.y).or_default().push((min_y, max_y));
        } else if previous.y == current.y {
            let min_x = min(previous.x, current.x);
            let max_x = max(previous.x, current.x);
            y_ranges.entry(current.y).or_default().push((min_x, max_x));
        }
    }

    let min_y = points.iter().map(|point| point.y).min().unwrap();
    let max_y = points.iter().map(|point| point.y).max().unwrap();

    let mut max_square: i64 = 0;

    // Scanning from top to bottom
    let mut active_ranges:  Vec<((i64,i64),i64)> = Vec::new();

    for y in min_y..(max_y + 1) {
        let new_ranges = x_ranges.get(&y);
        if new_ranges.is_none() { continue };

        let edges = new_ranges.unwrap();
        for edge in edges {
            let to_remove = find_overlapping_indices_reversed(edge, &active_ranges);
            let affected_ranges: Vec<((i64,i64),i64)>
                = to_remove.iter().map(|index| active_ranges.remove(*index)).collect();
            for affected_range in affected_ranges {
                let range_edge = affected_range.0;
                let intersection = intersection(&range_edge, edge).unwrap();
                let remaining = remove_range_from_range(&range_edge, &intersection);
            }
        }

        // Find all ranges we overlap with 2 and remove those active parts
        // After calculating their sizes

        // Find all ranges without overlap and create new active ranges from them

        // Find all ranges we overlap with 1 and merge those as new

    }
    assert_eq!(active_ranges.len(), 0);

    println!("{max_square} is the largest area of any rectangle you can make using only red and green tiles?")
    // Scanning for left to right
}

fn find_overlapping_indices_reversed(edge: &(i64,i64),
                                     active_squares: &Vec<((i64,i64),i64)>) -> Vec<usize> {
    active_squares.iter()
                  .enumerate()
                  .filter(|(_, (square_edge, _))| {
                      let r = intersection(edge, square_edge);
                      if r.is_none() {
                          return false;
                      }
                      let r = r.unwrap();
                      return r.0 != r.1
                  })
                 .map(|(index, _)| index)
                 .rev()
                 .collect()
}

fn intersection(left: &(i64, i64), right: &(i64, i64)) -> Option<(i64, i64)> {
        if left.0 >= right.0 && left.0 <= right.1 {
            let l = max(left.0, right.0);
            let r = min(left.1, right.1);
            Some((l,r))
        } else if (right.0 >= left.0 && right.0 <= left.1) {
            let l = max(right.0, left.0);
            let r = min(right.1, left.1);
            Some((l,r))
        } else {
            None
        }
}

fn remove_range_from_range(original: &(i64, i64), to_delete:  &(i64, i64)) -> Vec<(i64, i64)> {
    let mut rvalue = Vec::new();
    rvalue
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
