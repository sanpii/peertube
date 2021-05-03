pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Http(#[from] reqwest::Error),
    #[error("{}", .0.error)]
    Peertube(crate::Err),
}
