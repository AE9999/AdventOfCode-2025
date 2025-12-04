use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main()  -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;

    solve1(&input);

    solve2(input);

    Ok(())
}

fn solve1(input: &Grid) {
    let res = input.accessible().count();

    println!("{}  many rolls of paper can be accessed by a forklift", res);
}

fn solve2(mut input: Grid) {
    let mut removed = 0;
    loop {
        let res = input.accessible().count();
        if res == 0 { break };
        removed += res;
        input = input.clear_toilet_rolls(input.accessible());
    }
    println!("{} many rolls of paper in total can be removed by the Elves and their forklifts",
             removed);
}


fn read_input(filename: &String) -> io::Result<Grid> {
    let file_in = File::open(filename)?;
    Ok(Grid {
        grid: BufReader::new(file_in).lines().map(|x| x.unwrap().chars().collect()).collect()
    })
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

    fn neighbours(&self) -> impl Iterator<Item = Point>+  '_  {
        let dxdys =
            vec![
                Point::new(-1, 0),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(0, 1),

                Point::new(-1, -1),
                Point::new(-1, 1),
                Point::new(1, -1),
                Point::new(1, 1),
            ];

        dxdys.into_iter().map(move |p| self.add(&p))
    }
}

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

    fn points(&self) -> impl Iterator<Item = Point> {
        let w = self.width();
        let h = self.height();

        (0..w).flat_map(move |x| {
            (0..h).map(move |y| Point::new(x, y))
        })
    }

    fn accessible(&self) -> impl Iterator<Item = Point> + '_ {
        self.points()
            .filter(|p| self.char_at(p) == Some(&'@'))
            .filter(|p| {
                let neighbour_at_count = p.neighbours()
                    .filter(|n| self.char_at(n) == Some(&'@'))
                    .count();
                neighbour_at_count < 4
            })
    }

    fn clear_toilet_roll(&mut self, point: &Point) {
        let to_update =
            self.grid.get_mut(point.y as usize).unwrap().get_mut(point.x as usize).unwrap();
        *to_update = '.';
    }

    fn clear_toilet_rolls(&self, to_clear: impl Iterator<Item = Point>) ->  Self {
        let new_grid = self.grid.clone();
        let mut new_self = Grid {
            grid: new_grid
        };
        for clearing in to_clear {
            new_self.clear_toilet_roll(&clearing);
        }
        new_self

    }
}
