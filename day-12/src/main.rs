use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::collections::{HashSet, HashMap};
use std::fmt;

use rayon::prelude::*;
use indicatif::{ProgressBar, ParallelProgressIterator};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;

    solve1(&input);

    Ok(())
}

fn solve1(problem: &Problem) {
    let piece_index_to_possible_rotations: Vec<Vec<Piece>> =
        problem.pieces.iter()
                      .map(|piece| piece.find_unique_transformation_results())
                      .collect();

    let bar = ProgressBar::new(problem.puzzles.len() as u64);

    let res =
        problem.puzzles
            .par_iter()                    // 🔥 parallel iterator
            .progress_with(bar.clone())    // tie progress bar to rayon
            .filter(|puzzle| can_fit(&piece_index_to_possible_rotations, puzzle))
            .count();                        // parallel count

    bar.finish_with_message("done");

    println!("{res} of the regions can fit all of the presents listed.")
}

fn can_fit(piece_index_to_possible_rotations: &Vec<Vec<Piece>>,
           puzzle: &Puzzle) -> bool {

    let grid = Grid::new(&puzzle);

    let mut index_of_pieces_to_place: Vec<usize> = Vec::new();

    for (index, amount) in puzzle.amount_of_pieces_to_place.iter().enumerate() {
        for _ in 0..*amount {
            index_of_pieces_to_place.push(index);
        }
    }

    let mut cache: HashMap<Grid, usize> = HashMap::new();
    can_fit_piece_on_grid(0,
                          &index_of_pieces_to_place,
                          &piece_index_to_possible_rotations,
                          &grid,
                          &mut cache).is_some()
}

fn calculate_required_space(index: usize,
                            index_of_pieces_to_place: &Vec<usize>,
                            piece_index_to_possible_rotations: &Vec<Vec<Piece>>) -> usize {
    let mut total = 0;
    for i in index..index_of_pieces_to_place.len() {
        let piece_type = index_of_pieces_to_place[i];
        // Get the first shape from the possible rotations
        if let Some(first_shape) = piece_index_to_possible_rotations.get(piece_type)
            .and_then(|shapes| shapes.first()) {
            total += piece_area(first_shape);
        }
    }
    total
}

fn piece_area(piece: &Piece) -> usize {
    piece.shape.iter()
        .flat_map(|row| row.iter())
        .filter(|&&ch| ch == '#')
        .count()
}

fn can_fit_piece_on_grid(index: usize,
                         index_of_pieces_to_place: &Vec<usize>,
                         piece_index_to_possible_rotations: &Vec<Vec<Piece>>,
                         grid: &Grid,
                         cache: &mut HashMap<Grid, usize>) -> Option<Grid> {

    if index >= index_of_pieces_to_place.len() {
        return Some(grid.clone())
    }

    if let Some(&cached_fail_index) = cache.get(grid) {
        if index >= cached_fail_index {
            return None;
        }
    }

    let empty_spaces = grid.empty_spaces();
    let required_space = calculate_required_space(index, index_of_pieces_to_place, piece_index_to_possible_rotations);
    if empty_spaces < required_space {
        cache.insert(grid.clone(), index);
        return None;
    }

    let index_of_piece = index_of_pieces_to_place[index];

    let possible_formations = &piece_index_to_possible_rotations[index_of_piece];

    for possible_formation in possible_formations {
        for point in grid.points() {
            let x = grid.place_piece_at(possible_formation, &point);
            if x.is_some() {
                let next_grid = x.unwrap();
                let next = can_fit_piece_on_grid(index  + 1,
                                                 index_of_pieces_to_place,
                                                 piece_index_to_possible_rotations,
                                                 &next_grid,
                                                 cache);
                if next.is_some() {
                    return next;
                }
            }
        }
    }

    cache.insert(grid.clone(), index);
    None
}

fn read_input(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;

    let mut pieces: Vec<Piece> = Vec::new();
    let mut puzzles: Vec<Puzzle> = Vec::new();

    let mut it = BufReader::new(file_in).lines().peekable();

    // parse pieces
    loop {
        let x = it.peek().unwrap();

        if x.as_ref().unwrap().contains('x') {
            break;
        }
        it.next(); // skip header
        let mut shape: Vec<Vec<char>> = Vec::new();

        while let Some(line_res) = it.next() {
            let line = line_res?;
            if line.trim().is_empty() {
                break; // blank separator line consumed, stop this piece
            }
            shape.push(line.chars().collect());
        }

        pieces.push(Piece {
            shape
        })
    }

    for raw_line in it {
        let line = raw_line.unwrap();
        let mut it = line.split(": ");
        let mut raw_grid_it = it.next().unwrap().split('x');

        let amount_of_pieces_to_place: Vec<usize> =
            it.next().unwrap().split_whitespace().map(|amount| amount.parse::<usize>().unwrap()).collect();

        let grid_x_size = raw_grid_it.next().unwrap().parse::<usize>().unwrap();
        let grid_y_size = raw_grid_it.next().unwrap().parse::<usize>().unwrap();
        puzzles.push(Puzzle {
            grid_x_size,
            grid_y_size,
            amount_of_pieces_to_place
        })
    }


    Ok(Problem {
        pieces,
        puzzles
    })
}


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Problem {
    pieces: Vec<Piece>,
    puzzles: Vec<Puzzle>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Piece {
    shape: Vec<Vec<char>>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Puzzle {
    grid_x_size: usize,
    grid_y_size: usize,
    amount_of_pieces_to_place: Vec<usize>
}


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Grid {
    grid: Vec<Vec<char>>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Rotation {
    Degree0,
    Degree90,
    Degree180,
    Degree270
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Flip {
    None,
    Horizontal,
    Vertical
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for &ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {

    fn new(puzzle: &Puzzle) -> Self {
        let grid: Vec<Vec<char>> =
            vec![vec!['.'; puzzle.grid_x_size]; puzzle.grid_y_size];

        Grid {
            grid
        }
    }

    fn height(&self) -> i32 {
        self.grid.len() as i32
    }

    fn width(&self) -> i32  {
        self.grid.get(0).unwrap().len() as i32
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let w = self.width();
        let h = self.height();

        (0..w).flat_map(move |x| {
            (0..h).map(move |y| Point::new(x, y))
        })
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

    fn place_piece_at(&self, piece: &Piece, position: &Point) -> Option<Self> {
        let mut next_state = self.clone();
        for point in piece.points() {
            let grid_pos = position.add(&point);
            let char_to_update = next_state.char_at(&grid_pos);
            if char_to_update != Some(&'.') {
                return None
            }
            next_state.update_position(&grid_pos);
        }
        Some(next_state)
    }

    fn update_position(&mut self, point: &Point) {
        let to_update =
            self.grid.get_mut(point.y as usize).unwrap().get_mut(point.x as usize).unwrap();
        *to_update = '#';
    }

    fn empty_spaces(&self) -> usize {
        self.grid.iter()
            .flat_map(|row| row.iter())
            .filter(|&&ch| ch == '.')
            .count()
    }

}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Self {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.shape {
            for &ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Piece {
    fn transform(&self, transformation: (Rotation, Flip)) -> Self {
        let (rot, flip) = transformation;

        let rotated = rotate_grid(&self.shape, rot);
        let flipped = flip_grid(&rotated, flip);

        Piece { shape: flipped }
    }

    fn find_unique_transformation_results(&self) -> Vec<Self> {
        let rotations = [
            Rotation::Degree0,
            Rotation::Degree90,
            Rotation::Degree180,
            Rotation::Degree270,
        ];
        let flips = [Flip::None, Flip::Horizontal, Flip::Vertical];

        let mut set: HashSet<Piece> = HashSet::new();

        for r in rotations.iter() {
            for f in flips.iter() {
                set.insert(self.transform((r.clone(), f.clone())));
            }
        }

        set.into_iter().collect()
    }

    fn height(&self) -> i32 {
        self.shape.len() as i32
    }

    fn width(&self) -> i32  {
        self.shape.get(0).unwrap().len() as i32
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let w = self.width();
        let h = self.height();
        let shape = &self.shape;

        (0..w).flat_map(move |x| {
            (0..h).filter_map(move |y| {
                if shape[y as usize][x as usize] == '#' {
                    Some(Point::new(x, y))
                } else {
                    None
                }
            })
        })
    }
}

fn rotate_grid(grid: &[Vec<char>], rot: Rotation) -> Vec<Vec<char>> {
    match rot {
        Rotation::Degree0 => grid.to_vec(),

        Rotation::Degree90 => {
            // new[y][x] = old[h-1-x][y]
            let h = grid.len();
            let w = grid[0].len();
            (0..w)
                .map(|y| (0..h).map(|x| grid[h - 1 - x][y]).collect())
                .collect()
        }

        Rotation::Degree180 => {
            let h = grid.len();
            let w = grid[0].len();
            (0..h)
                .map(|y| (0..w).map(|x| grid[h - 1 - y][w - 1 - x]).collect())
                .collect()
        }

        Rotation::Degree270 => {
            // new[y][x] = old[x][w-1-y]
            let h = grid.len();
            let w = grid[0].len();
            (0..w)
                .map(|y| (0..h).map(|x| grid[x][w - 1 - y]).collect())
                .collect()
        }
    }
}

fn flip_grid(grid: &[Vec<char>], flip: Flip) -> Vec<Vec<char>> {
    match flip {
        Flip::None => grid.to_vec(),

        Flip::Horizontal => {
            // mirror left-right: reverse each row
            grid.iter()
                .map(|row| row.iter().copied().rev().collect())
                .collect()
        }

        Flip::Vertical => {
            // mirror top-bottom: reverse row order
            grid.iter().cloned().rev().collect()
        }
    }
}