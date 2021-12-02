use std::{
    error, fmt,
    fmt::{Display, Formatter},
    io,
};

#[derive(Debug, Default)]
pub struct StopCode;

impl error::Error for StopCode {}

impl Display for StopCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Code failed to execute")
    }
}

impl From<io::Error> for StopCode {
    fn from(_: io::Error) -> StopCode {
        StopCode
    }
}
