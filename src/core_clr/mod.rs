//! Manage loading and unloading of the libcoreclr dynamic library.
extern crate libloading;
extern crate thiserror;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreClrError
{
    #[error("Library not found.")]
    NotFound,
    #[error("Load error.")]
    LibraryError(libloading::Error),
    #[error("Standard io error.")]
    IoError(std::io::Error)
}

impl From<libloading::Error> for CoreClrError
{
    fn from(error: libloading::Error) -> CoreClrError
    {
        CoreClrError::LibraryError(error)
    }
}

impl From<std::io::Error> for CoreClrError
{
    fn from(error: std::io::Error) -> CoreClrError
    {
        CoreClrError::IoError(error)
    }
}

pub type Result<T> = std::result::Result<T, CoreClrError>;

mod core_types;
pub(crate) use core_types::*;

mod core_clr;
pub use core_clr::CoreClr;
