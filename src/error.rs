use thiserror::Error;
use serde_cbor::Error as CError;
use sled::Error as SError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CborError(#[from] CError),
    #[error(transparent)]
    SledError(#[from] SError),
    #[error(transparent)]
    PropagatedError(#[from] Box<dyn std::error::Error>),
    #[error("no data in the DB")]
    NoDataError,
}
