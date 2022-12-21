use md5;

pub fn process_part(input: String, target_suffix: &str) -> u32 {
    let mut suffix = 0;
    let mut digest_hex = String::from("");
    while !digest_hex.starts_with(target_suffix) {
        suffix += 1;
        let digest = md5::compute((input.clone() + &suffix.to_string()).as_bytes());
        digest_hex = format!("{:x}", digest);
    }
    suffix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_should_calculate_mining_suffix() {
        assert_eq!(process_part("abcdef".to_string(), "00000"), 609043);
        assert_eq!(process_part("pqrstuv".to_string(), "00000"), 1048970);
    }
}
