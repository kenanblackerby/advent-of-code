use fancy_regex::Regex;

pub fn process_part1(input: &str) -> u32 {
    0
}

pub fn process_part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_should_count_nice_string() {
        let result = process_part1("ugknbfddgicrmopn");
        assert_eq!(result, 1);
    }

    #[test]
    fn process_part1_should_not_count_naughty_string_with_less_than_3_vowels() {
        assert_eq!(process_part1("dvszwmarrgsowjxmb"), 0);
    }
}
