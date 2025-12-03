use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    File::open("input/input01.txt".to_string())?.read_to_string(&mut s)?;

    println!("part 1: {}", process1(&s)?);

    Ok(())
}

fn process1(s: &str) -> Result<u32, Box<dyn Error>> {
    let mut dial = Dial::new(50, 100);
    let mut count_zeros = 0;
    for line in s.lines() {
        let turn = Turn::from_str(line)?;
        dial.turn_by(turn);
        if dial.value == 0 {
            count_zeros += 1;
        };
    }
    Ok(count_zeros)
}

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

#[derive(Debug, Clone)]
struct Dial {
    value: isize,
    upper_bound: usize,
}

impl Dial {
    fn new(value: isize, upper_bound: usize) -> Self {
        Self { value, upper_bound }
    }

    fn turn_by(&mut self, turn: Turn) {
        let sign = match turn.0 {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        self.value = (self.value + sign * turn.1 as isize).rem_euclid(self.upper_bound as isize);
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

    #[test]
    fn test_dial() {
        // normal
        let mut dial = Dial::new(50, 100);
        dial.turn_by(Turn(Direction::Left, 1));
        assert_eq!(dial.value, 49);

        let mut dial = Dial::new(50, 100);
        dial.turn_by(Turn(Direction::Right, 1));
        assert_eq!(dial.value, 51);

        // overflow

        let mut dial = Dial::new(50, 100);
        dial.turn_by(Turn(Direction::Left, 101));
        assert_eq!(dial.value, 49);

        let mut dial = Dial::new(50, 100);
        dial.turn_by(Turn(Direction::Right, 101));
        assert_eq!(dial.value, 51);

        // edge case

        let mut dial = Dial::new(99, 100);
        dial.turn_by(Turn(Direction::Right, 1));
        assert_eq!(dial.value, 0);

        let mut dial = Dial::new(0, 100);
        dial.turn_by(Turn(Direction::Left, 0));
        assert_eq!(dial.value, 0);
    }

    #[test]
    fn example1() {
        let mut dial = Dial::new(50, 100);
        assert_eq!(dial.value, 50);

        let mut turns = EXAMPLE.lines().map(|l| Turn::from_str(l).unwrap());

        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 82);
        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 52);
        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 0);
        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 95);
        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 55);
        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 0);
        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 99);
        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 0);
        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 14);
        dial.turn_by(turns.next().unwrap());
        assert_eq!(dial.value, 32);
    }
}
