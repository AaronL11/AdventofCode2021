use aoc2021::{scanner::Scanner, stopcode::StopCode};
use std::io::{stdin, stdout, BufWriter, Read, Stdout, Write};

#[allow(non_snake_case)]
fn solve<R: Read>(mut scan: Scanner<R>, mut out: BufWriter<Stdout>) -> Result<(), StopCode> {
    writeln!(
        out,
        "{}",
        scan.words::<u16>()
            .collect::<Vec<_>>()
            .windows(3)
            .fold((-1, 0), |(count, prev), w| {
                let sum = w.into_iter().sum();
                if prev < sum {
                    (count + 1, sum)
                } else {
                    (count, sum)
                }
            })
            .0
    )?;
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let stdin = stdin();
    let scan = Scanner::new(stdin.bytes());
    let out = BufWriter::new(stdout());
    solve(scan, out)
}
