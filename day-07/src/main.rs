use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::collections::{HashMap, VecDeque};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let grid = read_input(input)?;
    solve1(grid.clone());

    solve2(&grid);

    Ok(())
}

fn solve2(grid: &Grid) {

    let start =
        grid.points().find(|point| grid.char_at(point) == Some(&'S')).unwrap();

    let mut cache: HashMap<Point, u64> = HashMap::new();

    let res = do_solve2(grid, &start, &mut cache);

    println!("{} many timelines will a single tachyon particle end up on", res)
}

fn do_solve2(grid: &Grid, point: &Point, cache: &mut HashMap<Point, u64>) -> u64 {
    let down = Point::new(0, 1);
    let left = Point::new(-1, 0);
    let right = Point::new(1, 0);

    let current_location = grid.char_at(&point);

    if cache.contains_key(point) {
        return *cache.get(point).unwrap();
    }

    let rvalue =
        match current_location {
            Some('|') => {
                { panic!("This should never happen")};
            }
            Some('S') => {
                do_solve2(grid, &point.add(&down), cache)
            },
            Some('.') => {
                do_solve2(grid, &point.add(&down), cache)
            },
            Some('^') => {
                do_solve2(grid, &point.add(&left), cache) +
                do_solve2(grid, &point.add(&right), cache)
            }
            None => {
                1
            },
            _ => { panic!("Unexpected state")}
        };

    cache.insert(point.clone(), rvalue);

    rvalue
}

fn solve1(mut grid: Grid) {
    let mut dequeue: VecDeque<Point> = VecDeque::new();
    let mut res = 0;

    let start =
        grid.points().find(|point| grid.char_at(point) == Some(&'S')).unwrap();

    let down = Point::new(0, 1);
    let left = Point::new(-1, 0);
    let right = Point::new(1, 0);
    dequeue.push_back(start);

    while let Some(point) = dequeue.pop_front() {
        let current_location = grid.char_at(&point);
        match current_location  {
            Some('|') => {
                continue;
            }
            Some('S')  => {
                dequeue.push_back(point.add(&down))
            },
            Some('.')  => {
                grid.update_char_at(&point, '|');
                dequeue.push_back(point.add(&down))
            },
            Some('^') => {
                dequeue.push_back(point.add(&left));
                dequeue.push_back(point.add(&right));
                res += 1
            },
            None => {},
            _ => { panic!("Unexpected input")}
        }
    }

    println!("{} times will the beam be split", res);
}

fn read_input(filename: &String) -> io::Result<Grid> {
    let file_in = File::open(filename)?;
    let grid = BufReader::new(file_in).lines().map( | x| x.unwrap().chars().collect()).collect();
    Ok(
        Grid {
            grid
        }
    )
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Self {
        Point { x: self.x + other.x, y: self.y + other.y  }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct  Grid {
    grid: Vec<Vec<char>>
}

impl Grid {
    fn height(&self) -> i32 {
        self.grid.len() as i32
    }

    fn width(&self) -> i32  {
        self.grid.get(0).unwrap().len() as i32
    }

    fn char_at (&self, point: &Point) -> Option<&char> {
        if point.x < 0
            || point.x >= self.width()
            || point.y < 0
            || point.y >= self.height() {
            None
        } else {
            self.grid
                .get(point.y as usize)
                .and_then(|row| row.get(point.x as usize))
        }
    }

    fn update_char_at(&mut self, point: &Point, new_char: char) {
        self.grid[point.y as usize][point.x as  usize] = new_char;
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let w = self.width();
        let h = self.height();

        (0..w).flat_map(move |x| {
            (0..h).map(move |y| Point::new(x, y))
        })
    }
}
