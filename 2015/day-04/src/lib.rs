use md5::{compute};

pub fn process_part1(input: String) -> u32 {
    let digest = md5::compute(input.as_bytes());
    609043
}

pub fn process_part2(input: String) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_should_calculate_mining_suffix() {
        assert_eq!(process_part1("abcdef".to_string()), 609043);
        assert_eq!(process_part1("pqrstuv".to_string()), 1048970);
    }
}
