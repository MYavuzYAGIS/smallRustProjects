use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: pike <target.com>")]
    CliUsage,
}
