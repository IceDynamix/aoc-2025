use std::str::FromStr;

fn main() {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Turn(Direction, usize);

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let dir = match chars.next().expect("non-empty line") {
            'L' => Direction::Left,
            'R' => Direction::Right,
            x => return Err(format!("unexpected direction {x}")),
        };

        let amount = chars
            .collect::<String>()
            .parse()
            .map_err(|e| format!("{e}"))?;

        Ok(Turn(dir, amount))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &'static str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn parse_turns() {
        let mut iter = EXAMPLE.lines().map(Turn::from_str);
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Left, 68))));
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Left, 30))));
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Right, 48))));
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Left, 5))));
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Right, 60))));
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Left, 55))));
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Left, 1))));
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Left, 99))));
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Right, 14))));
        assert_eq!(iter.next(), Some(Ok(Turn(Direction::Left, 82))));
        assert_eq!(iter.next(), None);

        assert!(Turn::from_str("Z10").is_err());
        assert!(Turn::from_str("L-10").is_err());
    }
}
