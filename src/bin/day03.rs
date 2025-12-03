use std::fs::File;
use std::io::Read;

fn main() -> anyhow::Result<()> {
    let mut s = String::new();
    File::open("input/input03.txt".to_string())?.read_to_string(&mut s)?;

    println!("part 1: {}", todo!());
    println!("part 2: {}", todo!());

    Ok(())
}

#[cfg(test)]
mod test {}
