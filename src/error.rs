use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O Error")]
    Io(#[from] ::std::io::Error),
    #[error("Parser")]
    Parse,
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
