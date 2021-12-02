use aoc2021::{scanner::Scanner, stopcode::StopCode};
use std::io::{self, stdin, stdout, BufRead, BufWriter, Read, Stdout, Write};

#[allow(non_snake_case)]
fn solve<R: Read + BufRead>(
    mut scan: Scanner<R>,
    mut out: BufWriter<Stdout>,
) -> Result<(), StopCode> {
    //
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let stdin = stdin();
    let scan = Scanner::new(stdin.lock());
    let out = BufWriter::new(stdout());
    solve(scan, out)
}
