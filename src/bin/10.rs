use std::collections::HashMap;
use std::cmp::max;

#[derive(Hash, Debug, Clone)]
struct Point { x: i32, y: i32 }

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[derive(Debug)]
struct Angle { rx: i32, ry: i32 }

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        let self_diag = get_diagnoal(self.rx, self.ry);
        let other_diag = get_diagnoal(other.rx, other.ry);

        let self_sin = (self.ry as f64 / self_diag).asin();
        let self_cos = (self.rx as f64 / self_diag).acos();
        let other_sin = (other.ry as f64 / other_diag).asin();
        let other_cos = (other.rx as f64 / other_diag).acos();

        // println!("{:?} {:?} {}", self, other, self_sin == other_sin && self_cos == other_cos);

        self_sin == other_sin && self_cos == other_cos
    }
}

fn get_diagnoal(a: i32, b: i32) -> f64 {
    let comp: f64 = (a * a + b * b) as f64;
    comp.sqrt()
}

fn main() {

//     let input = ".#..#
// .....
// #####
// ....#
// ...##";

//     let input = "......#.#.
// #..#.#....
// ..#######.
// .#.#.###..
// .#..#.....
// ..#....#.#
// #..#....#.
// .##.#..###
// ##...#..#.
// .#....####";

//     let input = "#.#...#.#.
// .###....#.
// .#....#...
// ##.#.#.#.#
// ....#.#.#.
// .##..###.#
// ..#...##..
// ..##....##
// ......#...
// .####.###.";

    let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

//     let input = ".#..##.###...#######
// ##.############..##.
// .#.######.########.#
// .###.#######.####.#.
// #####.##.#.##.###.##
// ..#####..#.#########
// ####################
// #.####....###.#.#.##
// ##.#################
// #####.##.###..####..
// ..######..##.#######
// ####.##.####...##..#
// .#####..#.######.###
// ##...#.##########...
// #.##########.#######
// .####.#.###.###.#.##
// ....##.##.###..#####
// .#.#.###########.###
// #.#.#.#####.####.###
// ###.##.####.##.#..##";

// let input = "#.....#...#.........###.#........#..
// ....#......###..#.#.###....#......##
// ......#..###.......#.#.#.#..#.......
// ......#......#.#....#.##....##.#.#.#
// ...###.#.#.......#..#...............
// ....##...#..#....##....#...#.#......
// ..##...#.###.....##....#.#..##.##...
// ..##....#.#......#.#...#.#...#.#....
// .#.##..##......##..#...#.....##...##
// .......##.....#.....##..#..#..#.....
// ..#..#...#......#..##...#.#...#...##
// ......##.##.#.#.###....#.#..#......#
// #..#.#...#.....#...#...####.#..#...#
// ...##...##.#..#.....####.#....##....
// .#....###.#...#....#..#......#......
// .##.#.#...#....##......#.....##...##
// .....#....###...#.....#....#........
// ...#...#....##..#.#......#.#.#......
// .#..###............#.#..#...####.##.
// .#.###..#.....#......#..###....##..#
// #......#.#.#.#.#.#...#.#.#....##....
// .#.....#.....#...##.#......#.#...#..
// ...##..###.........##.........#.....
// ..#.#..#.#...#.....#.....#...###.#..
// .#..........#.......#....#..........
// ...##..#..#...#..#...#......####....
// .#..#...##.##..##..###......#.......
// .##.....#.......#..#...#..#.......#.
// #.#.#..#..##..#..............#....##
// ..#....##......##.....#...#...##....
// .##..##..#.#..#.................####
// ##.......#..#.#..##..#...#..........
// #..##...#.##.#.#.........#..#..#....
// .....#...#...#.#......#....#........
// ....#......###.#..#......##.....#..#
// #..#...##.........#.....##.....#....";

    let mut asteroids: HashMap<Point, i32> = HashMap::new();
    let mut counts: HashMap<Point, i32> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert(Point{ x: x as i32, y: y as i32 }, 0);
            }
            print!("{}", c);
        }
        println!("");
    }

    for asteroid in asteroids.keys() {
        let count = count_seen_asteroids(asteroid, &asteroids);
        counts.insert(asteroid.clone(), count);
    }

    println!("{:?}", counts.iter().fold(0, |acc, (_, count)| max(acc, *count)));

    let mut a = vec!(Angle{ rx: 1, ry: 1 }, Angle{ rx: 3, ry: 3 });
    println!("{:?}", a.iter().map(get_radians).collect::<Vec<f32>>());
}

fn count_seen_asteroids(asteroid: &Point, map: &HashMap<Point, i32>) -> i32 {
    let mut angles: Vec<f32> = map.iter()
        .filter(|(point, _)| point != &asteroid)
        .map(|(point, _)| get_angle(asteroid, point))
        .map(|angle| get_radians(&angle))
        .collect();

    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    angles.dedup();

    angles.len() as i32
}

fn get_angle(origin: &Point, target: &Point) -> Angle {
    let rx = target.x - origin.x;
    let ry = target.y - origin.y;
    Angle{ rx, ry }
}

fn get_radians(angle: &Angle) -> f32 {
    let asin = (angle.rx as f32 / get_diagnoal(angle.rx, angle.ry) as f32).asin();
    if angle.rx >= 0  && angle.ry >= 0 {
        0. + asin
    } else if angle.rx < 0 && angle.ry >= 0 {
        std::f32::consts::PI - asin
    } else if angle.rx < 0 && angle.ry < 0 {
        std::f32::consts::PI + asin
    } else {
        std::f32::consts::PI * 2.0 - asin
    }
}
