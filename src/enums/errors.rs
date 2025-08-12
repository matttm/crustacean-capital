use thiserror::Error;

#[derive(Debug, Error, Display, From)]
pub enum Error {
    InputError(String),
    DataError(String),
    InternalError(String),
}
