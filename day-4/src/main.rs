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

fn main() {
    let passwords = 307237..=769058;
    let total = passwords
        .filter(|p| check_password(*p))
        .count();
    
    println!("part1: {}", total);

    let passwords = 307237..=769058;
    let total2 = passwords
        .filter(|p| check_password_part_2(*p))
        .count();

    println!("part2: {}", total2);
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