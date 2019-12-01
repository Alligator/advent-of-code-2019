use std::fs;

fn calc_fuel(mass: i64) -> i64 {
    let fuel = (mass / 3) - 2;
    if fuel > 0 {
        let extra = calc_fuel(fuel);
        return extra + fuel;
    }
    return 0;
}

fn part_one(lines: &Vec<&str>) -> i64 {
    let mut sum: i64 = 0;
    for line in lines {
        let num = line.parse::<i64>().unwrap();
        sum += (num / 3) - 2;
    }
    return sum;
}

fn part_two(lines: &Vec<&str>) -> i64 {
    return lines.iter().fold(0, |acc, x| {
        let num = x.parse::<i64>().unwrap();
        return acc + calc_fuel(num);
    });
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split_whitespace().collect();

    println!("part 1: {0}", part_one(&lines));
    println!("part 2: {0}", part_two(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&vec!["12"]), 2);
        assert_eq!(part_one(&vec!["14"]), 2);
        assert_eq!(part_one(&vec!["12", "14"]), 4);
        assert_eq!(part_one(&vec!["1969"]), 654);
        assert_eq!(part_one(&vec!["100756"]), 33583);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&vec!["12"]), 2);
        assert_eq!(part_two(&vec!["14"]), 2);
        assert_eq!(part_two(&vec!["1969"]), 966);
        assert_eq!(part_two(&vec!["100756"]), 50346);
        assert_eq!(part_two(&vec!["1969", "100756"]), 51312);
    }
}