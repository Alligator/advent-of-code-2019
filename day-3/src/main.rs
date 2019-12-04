use std::fs;
use std::collections::{HashSet, HashMap};

type Point = (i64, i64);

fn get_points(path: &[&str]) -> (HashSet<Point>, HashMap<Point, i64>) {
    let mut current_point: Point = (0, 0);
    let mut points = HashSet::<Point>::new();
    let mut steps = HashMap::new();
    let mut total_steps = 0;

    points.insert(current_point);

    for item in path {
        let count = &item[1..].parse::<i64>().unwrap();
        let mut x_diff = 0;
        let mut y_diff = 0;

        if item.starts_with("R") {
            x_diff = 1;
            y_diff = 0;
        } else if item.starts_with("L") {
            x_diff = -1;
            y_diff = 0;
        } else if item.starts_with("U") {
            x_diff = 0;
            y_diff = -1;
        } else if item.starts_with("D") {
            x_diff = 0;
            y_diff = 1;
        }

        for i in 0..=*count {
            let point = (current_point.0 + (i * x_diff), current_point.1 + (i * y_diff));
            points.insert(point);

            if !steps.contains_key(&point) {
                steps.insert(point, total_steps);
            }

            total_steps += 1;
        }

        // take off the extra step we counted
        total_steps -= 1;

        current_point = (current_point.0 + (*count * x_diff), current_point.1 + (*count * y_diff));
    }

    return (points, steps);
}

fn get_closest_distance(path1: &[&str], path2: &[&str]) -> i64 {
    let (points1, _steps1) = get_points(&path1);
    let (points2, _steps2) = get_points(&path2);

    return points1
        .intersection(&points2)
        .copied()
        .filter(|p| !(p.0 == 0 && p.1 == 0))
        .map(|p| (p.0.abs() + p.1.abs()))
        .min()
        .unwrap();
}

fn get_closest_steps(path1: &[&str], path2: &[&str]) -> i64 {
    let (points1, steps1) = get_points(&path1);
    let (points2, steps2) = get_points(&path2);

    return points1
        .intersection(&points2)
        .copied()
        .filter(|p| !(p.0 == 0 && p.1 == 0))
        .map(|p| steps1.get(&p).unwrap() + steps2.get(&p).unwrap())
        .min()
        .unwrap();
}

fn main() {
    let src = fs::read_to_string("input.txt").unwrap();
    let mut lines = src
        .split_whitespace()
        .map(|x| x.split(",").collect());
    
    let path1: Vec<&str> = lines.next().unwrap();
    let path2: Vec<&str> = lines.next().unwrap();

    let point = get_closest_distance(&path1, &path2);
    println!("part1: {}", point);

    let steps = get_closest_steps(&path1, &path2);
    println!("part2: {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_points() {
        let path = ["R8","U5","L5","D3"];
        let (points, _steps) = get_points(&path);

        let mut expected_points = HashSet::new();
        // R8
        expected_points.insert((0, 0));
        expected_points.insert((1, 0));
        expected_points.insert((2, 0));
        expected_points.insert((3, 0));
        expected_points.insert((4, 0));
        expected_points.insert((5, 0));
        expected_points.insert((6, 0));
        expected_points.insert((7, 0));
        expected_points.insert((8, 0));

        // U5
        expected_points.insert((8, -1));
        expected_points.insert((8, -2));
        expected_points.insert((8, -3));
        expected_points.insert((8, -4));
        expected_points.insert((8, -5));

        // L5
        expected_points.insert((7, -5));
        expected_points.insert((6, -5));
        expected_points.insert((5, -5));
        expected_points.insert((4, -5));
        expected_points.insert((3, -5));

        // D3
        expected_points.insert((3, -4));
        expected_points.insert((3, -3));
        expected_points.insert((3, -2));

        assert_eq!(points, expected_points);
    }

    #[test]
    fn test_get_closest_point_1() {
        let path1 = ["R8","U5","L5","D3"];
        let path2 = ["U7","R6","D4","L4"];
        assert_eq!(get_closest_distance(&path1, &path2), 6);
    }

    #[test]
    fn test_get_closest_point_2() {
        let path1 = ["R75","D30","R83","U83","L12","D49","R71","U7","L72"];
        let path2 = ["U62","R66","U55","R34","D71","R55","D58","R83"];
        assert_eq!(get_closest_distance(&path1, &path2), 159);
    }

    #[test]
    fn test_get_closest_point_3() {
        let path1 = ["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"];
        let path2 = ["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"];
        assert_eq!(get_closest_distance(&path1, &path2), 135);
    }

    #[test]
    fn test_get_closest_point_cross_at_zero() {
        let path1 = ["R10"];
        let path2 = ["U5", "R5", "D5"];
        assert_eq!(get_closest_distance(&path1, &path2), 5);
    }

    #[test]
    fn test_get_closest_steps() {
        let path1 = ["R8","U5","L5","D3"];
        let path2 = ["U7","R6","D4","L4"];
        assert_eq!(get_closest_steps(&path1, &path2), 30);
    }
}