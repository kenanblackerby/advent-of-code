use std::{iter::StepBy, str::Chars};

pub fn process_part1(input: &str) -> usize {
    let mut visited = Vec::new();
    visited.push((0, 0));

    follow_a_santa_movements(&mut visited, input.chars().step_by(1));
    
    visited.len()
}

pub fn process_part2(input: &str) -> usize {
    let mut visited = Vec::new();
    visited.push((0,0));

    // Real Santa
    follow_a_santa_movements(&mut visited, input.chars().step_by(2));
    // Robo Santa
    follow_a_santa_movements(&mut visited, input[1..].chars().step_by(2));

    visited.len()
}

fn follow_a_santa_movements(visited: &mut Vec<(i32, i32)>, input: StepBy<Chars>) {
    let mut position = (0, 0);
    for direction in input {
        position = move_position(direction, position);
        if !visited.contains(&position) {
            visited.push(position);
        }
    }
}

fn move_position(direction: char, position: (i32, i32)) -> (i32, i32) {
    match direction {
        '>' => (position.0 + 1, position.1),
        '<' => (position.0 - 1, position.1),
        '^' => (position.0, position.1 - 1),
        'v' => (position.0, position.1 + 1),
        _ => panic!("invalid direction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_process_part1_single_move() {
        assert_eq!(process_part1(">"), 2);
    }

    #[test]
    fn should_process_part1_4_moves() {
        assert_eq!(process_part1("^>v<"), 4);
    }

    #[test]
    fn should_process_part2_1_move_each() {
        assert_eq!(process_part2("^v"), 3);
    }

    #[test]
    fn should_process_part2_2_move_each() {
        assert_eq!(process_part2("^>v<"), 3);
    }

    #[test]
    fn should_process_part2_5_moves_each() {
        assert_eq!(process_part2("^v^v^v^v^v"), 11);
    }
}
