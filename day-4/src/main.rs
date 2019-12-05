use rayon::prelude::*;
use std::time::Instant;

fn check_password(num: i64) -> bool {
    let string = num.to_string();
    let chars: Vec<char> = string.chars().collect();

    let conditions = chars
        .windows(2)
        .fold((false, true), |acc, w| {
            let double = w[0] == w[1];
            let not_decreasing = w[1] >= w[0];
            return (acc.0 || double, acc.1 && not_decreasing);
        });

    return conditions.0 && conditions.1;
}

fn check_password_part_2(num: i64) -> bool {
    let string = num.to_string();
    let mut chars = string.chars();

    let mut prev_char: char = chars.next().unwrap();
    let mut has_double = false;
    let mut current_double_count = 1;

    for c in chars {
        // must never increase
        if prev_char > c {
            return false;
        }

        if prev_char == c {
            current_double_count += 1;
        } else {
            if current_double_count == 2 {
                has_double = true;
            }
            current_double_count = 1;
        }

        prev_char = c;
    }

    if current_double_count == 2 {
        has_double = true;
    }

    return has_double;
}

fn series(passwords: &Vec<i64>) {
    let total = passwords
        .iter()
        .filter(|p| check_password(**p))
        .count();
    
    println!("part1: {}", total);

    let total2 = passwords
        .iter()
        .filter(|p| check_password_part_2(**p))
        .count();

    println!("part2: {}", total2);
}

fn parallel(passwords: &Vec<i64>) {
    let total = passwords
        .par_iter()
        .filter(|p| check_password(**p))
        .count();
    
    println!("part1: {}", total);

    let total2 = passwords
        .par_iter()
        .filter(|p| check_password_part_2(**p))
        .count();

    println!("part2: {}", total2);
}

fn main() {
    let passwords: Vec<i64> = (307237..=769058).collect();

    // parallel/series benchmarks:
    //   debug series:     1.754s
    //   debug parallel:   532ms
    //
    //   release series:   175ms
    //   release parallel: 55ms
    //
    // the parallel version tends to be about 70% faster, nice

    let start = Instant::now();
    series(&passwords);
    let end = Instant::now();
    println!("series: {:?}\n", end.duration_since(start));

    let start = Instant::now();
    parallel(&passwords);
    let end = Instant::now();
    println!("parallel: {:?}", end.duration_since(start));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_password() {
        let password = 111234;
        assert_eq!(check_password(password), true);
    }

    #[test]
    fn test_check_password_all_ones() {
        let password = 111111;
        assert_eq!(check_password(password), true);
    }

    #[test]
    fn test_check_password_decreases() {
        let password = 223450;
        assert_eq!(check_password(password), false);
    }

    #[test]
    fn test_check_password_no_double() {
        let password = 123789;
        assert_eq!(check_password(password), false);
    }

    #[test]
    fn test_check_password_part_2() {
        let password = 112233;
        assert_eq!(check_password_part_2(password), true);
    }

    #[test]
    fn test_check_password_part_2_group() {
        let password = 123444;
        assert_eq!(check_password_part_2(password), false);
    }

    #[test]
    fn test_check_password_part_2_repeat() {
        let password = 111122;
        assert_eq!(check_password_part_2(password), true);
    }
}