pub fn process_part1(input: &str) -> i32 {
    input.chars().map(convert_braces_to_direction).sum()
}

pub fn process_part2(input: &str) -> usize {
    let directions = input
        .chars()
        .map(|brace| match brace {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .enumerate();
    let mut acc = 0;
    for (index, direction) in directions {
        acc += direction;
        if acc < 0 && direction == -1 {
            return index + 1;
        }
    }
    usize::MAX
}

fn convert_braces_to_direction(brace: char) -> i32 {
    match brace {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(process_part1("()"), 0);
        assert_eq!(process_part1("(())"), 0);
        assert_eq!(process_part1("()()"), 0);
        assert_eq!(process_part1("((("), 3);
        assert_eq!(process_part1("(()(()("), 3);
        assert_eq!(process_part1("))((((("), 3);
        assert_eq!(process_part1("())"), -1);
        assert_eq!(process_part1("))("), -1);
        assert_eq!(process_part1(")))"), -3);
        assert_eq!(process_part1(")())())"), -3);
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(")"), 1);
        assert_eq!(process_part2("()())"), 5);
    }
}
