use std::collections::HashMap;
use std::cmp::max;
use std::cmp::Ordering;

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

//     let input = ".#..#..###
// ####.###.#
// ....###.#.
// ..###.##.#
// ##.##.#.#.
// ....###..#
// ..#.#..#.#
// #..#.#.###
// .##...##.#
// .....#.#..";

    let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

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

    // 1113 too high

    println!("{:?}", counts.iter().fold(0, |acc, (_, count)| max(acc, *count)));
    beam_em(&Point{ x: 11, y: 13 }, &mut asteroids);
}

fn count_seen_asteroids(asteroid: &Point, map: &HashMap<Point, i32>) -> i32 {
    let mut angles: Vec<f32> = map.iter()
        .filter(|(point, _)| point != &asteroid)
        .map(|(point, _)| (point, get_angle(asteroid, point)))
        .map(|(point, angle)| get_radians(&angle))
        .collect();

    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    angles.dedup();

    if angles.len() == 303 {
        println!("{:?}", asteroid); // 26, 29
    }

    angles.len() as i32
}

fn get_distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p1.x - p2.x) as f64;
    let dy = (p1.y - p2.y) as f64;

    ((dx * dx) + (dy * dy)).sqrt()
}

fn cmp_distance(a: &Point, b: &Point, huh: &Point) -> Ordering {
    let dist1 = get_distance(a, huh);
    let dist2 = get_distance(b, huh);

    if dist1 < dist2 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn get_rock_line(asteroid: &Point, map: &mut HashMap<Point, i32>) -> Vec<Point> {
    let mut angles: Vec<(&Point,f32)> = map.iter()
        .filter(|(point, _)| point != &asteroid)
        .map(|(point, _)| (point, get_angle(asteroid, point)))
        .map(|(point, angle)| (point, get_radians(&angle)))
        .collect();

    angles.sort_by(|(p1, a), (p2, b)| match a.partial_cmp(b).unwrap() {
        Ordering::Equal => cmp_distance(p1, p2, asteroid),
        other => other
    });

    let mut uniq: Vec<(&Point, f32)> = vec!(angles[0]);
    for i in 1..(angles.len()) {
        if angles[i - 1].1 != angles[i].1 {
            println!("{}, {:?}, {}", uniq.len(), angles[i], get_distance(angles[i].0, asteroid));
            uniq.push(angles[i]);
        }
    }
    // println!("{:?}", uniq);

    uniq.iter().map(|(point,_)| Point{x:point.x, y:point.y}).collect()
}

fn beam_em(asteroid: &Point, map: &mut HashMap<Point, i32>) {
    let mut count = 0;
    let mut buffer: Vec<Point> = vec!();
    while count != 100 {
        if buffer.len() == 0 {
            buffer = get_rock_line(asteroid, map);
            println!("new ones {}", buffer.len());
        }
        // println!("{} {:?}, {}, {:?}", count + 1, buffer[0], map.len(), asteroid);
        map.remove(&buffer[0]);
        buffer.remove(0);
        count += 1;
    }
}

fn get_angle(origin: &Point, target: &Point) -> Angle {
    let rx = target.x - origin.x;
    let ry = target.y - origin.y;
    Angle{ rx, ry }
}

fn get_radians(angle: &Angle) -> f32 {
    let asin = (angle.rx as f64 / get_diagnoal(angle.rx, angle.ry) as f64).asin();
    // let acos = (((-angle.ry) as f64) / get_diagnoal(angle.rx, angle.ry) as f64).acos();

    let mut ans: f64 = 0.0;
    let mut ans1: f64 = 0.0;

    if angle.rx == 0 && -angle.ry > 0 {
        return (std::f64::consts::PI) as f32;
    } else if angle.rx == 0 && -angle.ry < 0 {
        return 0.0;
    } else if angle.ry == 0 && angle.rx > 0 {
        return (std::f64::consts::PI / 2.0) as f32;
    } else if angle.ry == 0 && angle.rx < 0 {
        return (std::f64::consts::PI * 1.5) as f32;
    }

    if angle.rx >= 0  && -angle.ry >= 0 {
        ans1 = (0. + asin) ;
    } else if angle.rx < 0 && -angle.ry >= 0 {
        ans1 = (std::f64::consts::PI - asin) ;
    } else if angle.rx < 0 && -angle.ry < 0 {
        ans1 = (std::f64::consts::PI + asin) ;
    } else {
        ans1 = (std::f64::consts::PI * 2.0 - asin) ;
    }


    // if angle.rx >= 0  && angle.ry < 0 {
    //     ans = (0. + asin) ;
    // } else if angle.rx >= 0 && angle.ry >= 0 {
    //     ans = (std::f64::consts::PI - asin) ;
    // } else if angle.rx < 0 && angle.ry >= 0 {
    //     ans = (std::f64::consts::PI + asin) ;
    // } else {
    //     ans = (std::f64::consts::PI * 2.0 - asin) ;
    // }

    // println!("{}, {}", ans, ans1);

    ((ans1 * 1000.0).round() / 1000.0) as f32
}
