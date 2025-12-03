use anyhow::anyhow;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let mut s = String::new();
    File::open("input/input03.txt".to_string())?.read_to_string(&mut s)?;

    println!("part 1: {}", process1(&s, 2));
    println!("part 2: {}", process1(&s, 12));

    Ok(())
}

fn process1(s: &str, num_digits: usize) -> usize {
    s.lines()
        .map(Bank::from_str)
        .map(|x| joltage(x.unwrap(), num_digits))
        .map(|x| x as usize)
        .sum()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Bank(Vec<u8>);

impl FromStr for Bank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|x| x as u8)
                    .ok_or(anyhow!("invalid digit"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Bank(chars))
    }
}

impl Deref for Bank {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

fn joltage(bank: Bank, num_digits: usize) -> u64 {
    assert!(bank.len() >= num_digits);

    let mut slice = &bank[..];
    let mut sum = 0;
    for d in 0..num_digits {
        assert!(slice.len() > 0);

        let &max = slice[..slice.len() + 1 + d - num_digits]
            .iter()
            .max()
            .unwrap();

        let i = slice.iter().position(|&x| x == max).unwrap();

        slice = &slice[i + 1..];
        sum += max as u64 * 10u64.pow(num_digits as u32 - 1 - d as u32);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Bank::from_str("987654321111111").unwrap(),
            Bank(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1])
        )
    }

    const EXAMPLE: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn example1() {
        let num_digits = 2;

        let check =
            |s: &str, sol: u64| assert_eq!(joltage(Bank::from_str(s).unwrap(), num_digits), sol);

        check("987654321111111", 98);
        check("811111111111119", 89);
        check("234234234234278", 78);
        check("818181911112111", 92);

        assert_eq!(process1(EXAMPLE, num_digits), 357);
    }

    #[test]
    fn example2() {
        let num_digits = 12;

        let check =
            |s: &str, sol: u64| assert_eq!(joltage(Bank::from_str(s).unwrap(), num_digits), sol);

        check("987654321111111", 987654321111);
        check("811111111111119", 811111111119);
        check("234234234234278", 434234234278);
        check("818181911112111", 888911112111);

        assert_eq!(process1(EXAMPLE, num_digits), 3121910778619);
    }
}
