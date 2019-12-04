use std::io::{prelude::*, BufReader, Result};
use std::string::String;
use std::fs::File;
use std::cmp;

#[derive(Debug)]
struct Point ( i32, i32);

fn main() -> Result<()> {
    let file = File::open("res/3_1.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let line1 = lines.next().expect("No line")?;
    let line2 = lines.next().expect("No line")?;

    let path1 = parse_line(line1);
    let path2 = parse_line(line2);

    println!("{:?}", path1);
    println!("{:?}", path2);

    let mut intersections: Vec<Point> = Vec::new();

    for i in 2..(path1.len() - 1) {
        let a0 = &path1[i - 1];
        let a1 = &path1[i];

        for j in 2..(path2.len() - 1) {
            let b0 = &path2[j - 1];
            let b1 = &path2[j];

            if let Some(point) = find_interserction(a0, a1, b0, b1) {
                intersections.push(point);
            }
        }
    }

    Ok(())
}

fn parse_line(line: String) -> Vec<Point> {
    let legs: Vec<&str> = line.split(',').collect();

    let mut points:Vec<Point> = Vec::new();
    points.push(Point(0, 0));

    for leg in legs {
        let Point(x0, y0) = points.last().unwrap();
        let (direction, d) = leg.split_at(1);
        let distance: i32 = d.parse().unwrap();

        let mut next_point = Point(*x0, *y0);
        match direction {
            "L" => { next_point.0 -= distance },
            "U" => { next_point.1 += distance },
            "R" => { next_point.0 += distance },
            "D" => { next_point.1 -= distance },
            _ => panic!("Invalid direction")
        }
        points.push(next_point);
    }

    points
}

fn find_interserction (a1: &Point, a2: &Point, b1: &Point, b2: &Point) -> Option<Point> {
    if (a1.0 == a2.0 && b1.0 == b2.0) {
        None
    }
    if (a1.1 == a2.1 && b1.1 == b2.1) {
        None
    }

    if (a1.0 == a2.0) {

    }

    // There is an intersection
    Some(Point(, ))
}
