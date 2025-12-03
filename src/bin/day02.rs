use anyhow::anyhow;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Id = usize;

fn main() -> anyhow::Result<()> {
    let mut s = String::new();
    File::open("input/input02.txt".to_string())?.read_to_string(&mut s)?;

    // println!("part 1: {}", todo!());
    // println!("part 2: {}", todo!());

    Ok(())
}

fn split_num(num: Id) -> Option<(Id, Id)> {
    let num_digits = num.ilog10() + 1;
    if !num_digits.is_multiple_of(2) {
        return None;
    }

    let left_half = num / 10usize.pow(num_digits / 2);
    let right_half = num % 10usize.pow(num_digits / 2);

    Some((left_half, right_half))
}

fn is_id_invalid(num: Id) -> bool {
    match split_num(num) {
        Some((left_half, right_half)) => left_half == right_half,
        None => false,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct IdRange(Id, Id);

impl FromStr for IdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once('-').ok_or(anyhow!("expected two values"))?;
        Ok(Self(first.parse()?, second.parse()?))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //     const EXAMPLE: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    // 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    // 824824821-824824827,2121212118-2121212124";

    #[test]
    fn parse_ids() {
        assert_eq!(IdRange::from_str("11-22").unwrap(), IdRange(11, 22));
        assert_eq!(IdRange::from_str("95-115").unwrap(), IdRange(95, 115));
        assert_eq!(IdRange::from_str("998-1012").unwrap(), IdRange(998, 1012));
        assert_eq!(
            IdRange::from_str("1188511880-1188511890").unwrap(),
            IdRange(1188511880, 1188511890)
        );
        assert_eq!(
            IdRange::from_str("222220-222224").unwrap(),
            IdRange(222220, 222224)
        );
        assert_eq!(
            IdRange::from_str("1698522-1698528").unwrap(),
            IdRange(1698522, 1698528)
        );
        assert_eq!(
            IdRange::from_str("446443-446449").unwrap(),
            IdRange(446443, 446449)
        );
        assert_eq!(
            IdRange::from_str("38593856-38593862").unwrap(),
            IdRange(38593856, 38593862)
        );
        assert_eq!(
            IdRange::from_str("565653-565659").unwrap(),
            IdRange(565653, 565659)
        );
        assert_eq!(
            IdRange::from_str("824824821-824824827").unwrap(),
            IdRange(824824821, 824824827)
        );
        assert_eq!(
            IdRange::from_str("2121212118-2121212124").unwrap(),
            IdRange(2121212118, 2121212124)
        );

        assert!(IdRange::from_str("ababa").is_err());
        assert!(IdRange::from_str("-1").is_err());
        assert!(IdRange::from_str("1-").is_err());
        assert!(IdRange::from_str("-0-0").is_err());
        assert!(IdRange::from_str("0--1").is_err());
        assert!(IdRange::from_str("-10--1").is_err());
    }

    #[test]
    fn test_invalid_digits() {
        dbg!(split_num(11));
        dbg!(split_num(12));
        dbg!(split_num(12));
        dbg!(split_num(123));
        dbg!(split_num(1188511885));
        dbg!(split_num(1188511886));
        assert!(is_id_invalid(11));
        assert!(!is_id_invalid(12));
        assert!(!is_id_invalid(12));
        assert!(!is_id_invalid(123));
        assert!(is_id_invalid(1188511885));
        assert!(!is_id_invalid(1188511886));
    }

    #[test]
    fn example1() {}
}
