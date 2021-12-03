use aoc2021::{scanner::Scanner, stopcode::StopCode, utils::*};
use std::io::{stdin, stdout, BufWriter, Read, Stdout, Write};

fn b2t(byte: u8) -> (Uint, Uint) {
    match byte {
        b'1' => (0, 1),
        b'0' => (1, 0),
        _ => unreachable!(),
    }
}

#[allow(non_snake_case)]
fn solve<R: Read>(mut scan: Scanner<R>, mut out: BufWriter<Stdout>) -> Result<(), StopCode> {
    let mut v = scan.get_str()?.bytes().map(b2t).collect::<Vec<_>>();
    while let Ok(word) = scan.get_str() {
        v.iter_mut()
            .zip(word.bytes().map(b2t))
            .for_each(|((pz, po), (wz, wo))| {
                *pz += wz;
                *po += wo
            });
    }
    let (g, e) = v
        .drain(..)
        .rev()
        .map(|(z, o)| if z > o { (0, 1) } else { (1, 0) })
        .enumerate()
        .fold((0, 0), |(g, e), (i, (l, s))| (g + (l << i), e + (s << i)));
    write!(out, "{}", g * e)?;
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let stdin = stdin();
    let scan = Scanner::new(stdin.bytes());
    let out = BufWriter::new(stdout());
    solve(scan, out)
}
