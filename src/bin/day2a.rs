use aoc2021::{scanner::Scanner, stopcode::StopCode, utils::*};
use std::io::{stdin, stdout, BufWriter, Read, Stdout, Write};

struct Point {
    x: Uint,
    y: Uint,
}

#[allow(non_snake_case)]
fn solve<R: Read>(mut scan: Scanner<R>, mut out: BufWriter<Stdout>) -> Result<(), StopCode> {
    let mut pos = Point { x: 0, y: 0 };
    while let Ok(word) = scan.get_str() {
        let word = word.to_owned();
        let d = scan.next::<Uint>()?;
        match &word[..] {
            "forward" => pos.x += d,
            "down" => pos.y += d,
            "up" => pos.y -= d,
            _ => unreachable!(),
        }
    }
    write!(out, "{}", pos.x * pos.y)?;
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let stdin = stdin();
    let scan = Scanner::new(stdin.bytes());
    let out = BufWriter::new(stdout());
    solve(scan, out)
}
