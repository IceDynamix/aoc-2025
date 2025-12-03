use anyhow::anyhow;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Id = usize;

fn main() -> anyhow::Result<()> {
    let mut s = String::new();
    File::open("input/input02.txt".to_string())?.read_to_string(&mut s)?;

    println!("part 1: {}", process1(&s));
    println!("part 2: {}", process2(&s));

    Ok(())
}

fn is_id_invalid(num: Id, max_num_parts: Option<Id>) -> bool {
    if num == 0 {
        return false;
    }
    
    let num_digits = num.ilog10() as usize + 1;
    if num_digits == 1 {
        return false;
    }

    let mut num_iter = num;

    let digits = {
        let mut digits = Vec::with_capacity(num_digits);

        for _ in 0..num_digits {
            digits.push(num_iter % 10);
            num_iter /= 10; // right shift
        }
        digits.reverse();
        digits
    };

    assert!(!digits.is_empty());

    let max_num_parts = match max_num_parts {
        Some(d) => d.min(num_digits),
        None => num_digits,
    };

    for num_parts in 2..=max_num_parts {
        if !num_digits.is_multiple_of(num_parts) {
            continue;
        }

        let mut chunks = digits.chunks_exact(num_digits / num_parts);
        let first = chunks.next().unwrap(); // guaranteed to be present because digits is not empty

        // .eq() checks if slices/iterators return the same items
        if chunks.all(|slice| slice.eq(first)) {
            return true;
        }
    }

    false
}

fn process1(s: &str) -> Id {
    s.split(',')
        .map(|l| IdRange::from_str(l).unwrap())
        .flat_map(|r| iter_invalid_ids_with_max_parts(r, 2))
        .sum()
}

fn process2(s: &str) -> Id {
    s.split(',')
        .map(|l| IdRange::from_str(l).unwrap())
        .flat_map(iter_invalid_ids)
        .sum()
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

fn iter_invalid_ids(range: IdRange) -> impl Iterator<Item = Id> {
    (range.0..=range.1).filter(|x| is_id_invalid(*x, None))
}

fn iter_invalid_ids_with_max_parts(range: IdRange, max_parts: usize) -> impl Iterator<Item = Id> {
    (range.0..=range.1).filter(move |x| is_id_invalid(*x, Some(max_parts)))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

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
        for x in [11, 1188511885] {
            assert!(is_id_invalid(x, Some(2)));
        }
        for x in [12, 123, 1188511886, 1, 0] {
            // neq
            assert!(!is_id_invalid(x, Some(2)));
        }

        fn check(from: Id, to: Id, res: Vec<Id>) {
            assert_eq!(
                iter_invalid_ids_with_max_parts(IdRange(from, to), 2).collect::<Vec<_>>(),
                res
            );
        }

        check(11, 22, vec![11, 22]);
        check(95, 115, vec![99]);
        check(998, 1012, vec![1010]);
        check(1188511880, 1188511890, vec![1188511885]);
        check(222220, 222224, vec![222222]);
        check(1698522, 1698528, vec![]);
        check(446443, 446449, vec![446446]);
        check(38593856, 38593862, vec![38593859]);
    }

    #[test]
    fn example1() {
        assert_eq!(process1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_invalid_digits2() {
        for x in [11, 1188511885, 111, 121212, 123123123123] {
            assert!(is_id_invalid(x, None));
        }
        for x in [12, 123, 1188511886, 1, 0] {
            // neq
            assert!(!is_id_invalid(x, None));
        }

        fn check(from: Id, to: Id, res: Vec<Id>) {
            assert_eq!(iter_invalid_ids(IdRange(from, to)).collect::<Vec<_>>(), res);
        }

        check(11, 22, vec![11, 22]);
        check(95, 115, vec![99, 111]);
        check(998, 1012, vec![999, 1010]);
        check(1188511880, 1188511890, vec![1188511885]);
        check(222220, 222224, vec![222222]);
        check(1698522, 1698528, vec![]);
        check(446443, 446449, vec![446446]);
        check(38593856, 38593862, vec![38593859]);
        check(38593856, 38593862, vec![38593859]);
        check(565653, 565659, vec![565656]);
        check(824824821, 824824827, vec![824824824]);
        check(2121212118, 2121212124, vec![2121212121]);
    }

    #[test]
    fn example2() {
        assert_eq!(process2(EXAMPLE), 4174379265);
    }
}
