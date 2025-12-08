use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let points = read_input(input)?;

    let amount = if input == "input/test.dat" {
        10
    } else {
        1000
    };

    solve(&points, amount);


    Ok(())
}

fn solve(points: &Vec<Point>, max_amount: usize) {

    let mut pair_2_distances: Vec<((usize, usize), f64)> =
        (0..points.len()).flat_map(move |i| {
            (i+1..points.len()).map(move |j|  ((i,j), points[i].distance(&points[j])  ) )
        }).collect();
    
    pair_2_distances.sort_by(|a, b| a.1.total_cmp(&b.1));

    let mut clusters: Vec<HashSet<usize>> = Vec::new();

    let mut index_2_cluster: HashMap<usize, usize> = HashMap::new();

    for (index, pair_2_distance) in pair_2_distances.iter().enumerate() {
        let l_id = pair_2_distance.0.0;
        let r_id = pair_2_distance.0.1;

        let l_cluster = index_2_cluster.get(&l_id).copied();
        let r_cluster = index_2_cluster.get(&r_id).copied();

        if l_cluster.is_none() && r_cluster.is_none() {
            let cluster_id = clusters.len();
            clusters.push(HashSet::new());
            clusters[cluster_id].insert(l_id);
            clusters[cluster_id].insert(r_id);
            index_2_cluster.insert(l_id, cluster_id);
            index_2_cluster.insert(r_id, cluster_id);
        } else if l_cluster == r_cluster {
            // do nothing
        } else if l_cluster.is_some() && r_cluster.is_none() {
            let cluster_id = l_cluster.unwrap();
            clusters[cluster_id].insert(r_id);
            index_2_cluster.insert(r_id, cluster_id);
        } else if l_cluster.is_none() && r_cluster.is_some() {
            let cluster_id = r_cluster.unwrap();
            clusters[cluster_id].insert(l_id);
            index_2_cluster.insert(l_id, cluster_id);
        } else {
            // merge
            let l_cluster_id = l_cluster.unwrap();
            let r_cluster_id = r_cluster.unwrap();

            let r_cluster_copy =  clusters.get(r_cluster_id).unwrap().clone();

            let l_cluster = clusters.get_mut(l_cluster_id).unwrap();

            for index_to_move in r_cluster_copy {
                l_cluster.insert(index_to_move);
                index_2_cluster.insert(index_to_move, l_cluster_id.clone());
            }

            clusters.get_mut(r_cluster_id).unwrap().clear();
        }

        if index == max_amount -1 {
            let mut cluster_copy = clusters.clone();
            cluster_copy.sort_by(|a, b| b.len().cmp(&a.len()));

            let res =
                cluster_copy.iter().take(3).fold(1, |acc, value| {acc * value.len()});

            println!("{res} you get if you multiply together the sizes of the three largest circuits?")
        }

        if clusters.iter().find(|x| x.len() == points.len()).is_some() {
            let res = points[l_id].x * points[r_id].x;
            println!("{} is what you get if you multiply together the X coordinates of the last two junction boxes you need to connect", res);
            break;
        }
    }
}

fn read_input(filename: &String) -> io::Result<Vec<Point>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|x| {
        let line = x.unwrap();
        let mut it = line.split(',');
        Point::new(
            it.next().unwrap().parse::<i64>().unwrap(),
            it.next().unwrap().parse::<i64>().unwrap(),
            it.next().unwrap().parse::<i64>().unwrap()
        )
    }).collect())
}

struct Point {
    x :i64,
    y :i64,
    z :i64,
}

impl Point {
    fn distance(&self, other: &Self) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let dz = self.z as f64 - other.z as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn new(x: i64, y: i64, z: i64) -> Self {
        Point {
            x,
            y,
            z
        }
    }
}