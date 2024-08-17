use snafu::{Location, ResultExt, Snafu};
use stack_error_macro::stack_trace_debug;

// layer 1
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[stack_trace_debug]
pub enum RustError {
    #[snafu(display("Failed to call Java"))]
    Rust {
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        source: JavaError,
    },
}

// layer 2
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[stack_trace_debug]
pub enum JavaError {
    #[snafu(display("Failed to call Python"))]
    Python {
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        source: PythonError,
    },
}

// layer 3
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[stack_trace_debug]
pub enum PythonError {
    #[snafu(display("IO Error"))]
    IO {
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        source: std::io::Error,
    },
}

fn fn1() -> Result<(), RustError> {
    fn2().context(RustSnafu)
}

fn fn2() -> Result<(), JavaError> {
    fn3().context(PythonSnafu)
}

fn fn3() -> Result<(), PythonError> {
    let res = Err(std::io::Error::new(std::io::ErrorKind::Other, "error"));
    res.context(IOSnafu)
}

fn main() {
    let res = fn1();
    println!("{:?}", res);
}
