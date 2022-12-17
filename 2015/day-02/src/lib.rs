pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|dimensions| {
            let dimensions = dimensions
                .split('x')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let surface_area = calculate_surface_area(dimensions[0], dimensions[1], dimensions[2]);
            let overage = calculate_overage(dimensions[0], dimensions[1], dimensions[2]);
            surface_area + overage
        })
        .sum()
}

fn calculate_surface_area(length: u32, width: u32, height: u32) -> u32 {
    2 * length * width + 2 * length * height + 2 * width * height
}

fn calculate_overage(length: u32, width: u32, height: u32) -> u32 {
    if length < width && height < width {
        length * height
    } else if length < height {
        length * width
    } else {
        height * width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2x3x4
1x1x10";

    #[test]
    fn should_calculate_surface_area() {
        assert_eq!(calculate_surface_area(1, 2, 3), 22);
    }

    #[test]
    fn should_calculate_overage() {
        assert_eq!(calculate_overage(1, 2, 3), 2);
        assert_eq!(calculate_overage(3, 4, 2), 6);
    }

    #[test]
    fn should_process_part1() {
        assert_eq!(process_part1("2x3x4"), 58);
        assert_eq!(process_part1("1x1x10"), 43);

        assert_eq!(process_part1(INPUT), 101);
    }
}
