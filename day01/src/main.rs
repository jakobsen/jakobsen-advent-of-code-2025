#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    clicks: i16,
    direction: Direction,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match split_first_char(value) {
            Some((direction, moves)) => {
                let clicks = moves.parse::<i16>().expect("Invalid moves!");
                let direction = match direction {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Invalid move string"),
                };
                Self { clicks, direction }
            }
            None => {
                panic!("Couldn't split string")
            }
        }
    }
}

fn main() {
    let (_final_position, times_passing_zero, times_at_zero) = include_str!("input.txt")
        .split("\n")
        .filter(|str| !str.is_empty())
        .map(Move::from)
        .fold(
            (50, 0, 0),
            |(position, times_passing_zero, times_at_zero), next_move| {
                let next_position = turn(position, &next_move);
                let next_times_passing_zero =
                    times_passing_zero + count_zero_passes(position, &next_move);
                let next_times_at_zero = times_at_zero + if next_position == 0 { 1 } else { 0 };
                (next_position, next_times_passing_zero, next_times_at_zero)
            },
        );

    println!("Part 1: {}", times_at_zero);
    println!("Part 2: {}", times_at_zero + times_passing_zero);
}

/// Source - https://stackoverflow.com/a/71629113
///
/// Posted by eyelash, modified by community. See post 'Timeline' for change history
///
/// Retrieved 2025-12-01, License - CC BY-SA 4.0
fn split_first_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    chars.next().map(|c| (c, chars.as_str()))
}

fn turn(current_position: i16, next_move: &Move) -> i16 {
    let minimal_turn: i16 = next_move.clicks % 100;
    match next_move.direction {
        Direction::Left => {
            let next_position = current_position - minimal_turn;
            if next_position < 0 {
                next_position + 100
            } else {
                next_position
            }
        }
        Direction::Right => (current_position + minimal_turn) % 100,
    }
}

fn count_zero_passes(position: i16, next_move: &Move) -> i16 {
    let full_turns = next_move.clicks / 100;
    let end = turn(position, next_move);
    if end == 0 || position == 0 {
        full_turns
    } else if (next_move.direction == Direction::Right && end < position)
        || (next_move.direction == Direction::Left && end > position)
    {
        full_turns + 1
    } else {
        full_turns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_turn() {
        assert_eq!(
            turn(
                50,
                &Move {
                    clicks: 1,
                    direction: Direction::Left
                }
            ),
            49
        )
    }

    #[test]
    fn test_full_turns() {
        let full_right_turn = Move {
            clicks: 100,
            direction: Direction::Right,
        };
        let full_left_turn = Move {
            clicks: 100,
            direction: Direction::Left,
        };
        let current_position = 60;
        assert_eq!(turn(current_position, &full_left_turn), current_position);
        assert_eq!(turn(current_position, &full_right_turn), current_position);
    }

    #[test]
    fn test_left_overflow() {
        let actual = turn(
            50,
            &Move {
                clicks: 68,
                direction: Direction::Left,
            },
        );
        let expected = 82;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_right_overflow() {
        let actual = turn(
            52,
            &Move {
                clicks: 48,
                direction: Direction::Right,
            },
        );
        let expected = 0;
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_no_intermittent_times_at_zero() {
        let actual = count_zero_passes(
            50,
            &Move {
                clicks: 0,
                direction: Direction::Left,
            },
        );
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_several_times_at_zero() {
        let actual = count_zero_passes(
            50,
            &Move {
                clicks: 1000,
                direction: Direction::Right,
            },
        );
        assert_eq!(actual, 10);
    }

    #[test]
    fn test_ending_at_zero_going_right_isnt_counted() {
        let actual = count_zero_passes(
            52,
            &Move {
                clicks: 48,
                direction: Direction::Right,
            },
        );
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_ending_at_zero_going_left_isnt_counted() {
        let actual = count_zero_passes(
            52,
            &Move {
                clicks: 52,
                direction: Direction::Left,
            },
        );
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_barely_crossing_zero() {
        let actual = count_zero_passes(
            52,
            &Move {
                clicks: 49,
                direction: Direction::Right,
            },
        );
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_starting_at_zero() {
        let actual = count_zero_passes(
            0,
            &Move {
                clicks: 5,
                direction: Direction::Left,
            },
        );
        assert_eq!(actual, 0)
    }
}
