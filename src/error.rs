use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Aho Corasick")]
    AhoCorasick(#[from] aho_corasick::BuildError),
    #[error("Parsing")]
    Parsing,
}

pub type Result<T> = std::result::Result<T, Error>;
