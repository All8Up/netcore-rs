extern crate thiserror;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreClrError
{
    #[error("Not found.")]
    NotFound,
    #[error("Load error.")]
    LibraryError(libloading::Error),
    #[error("Standard io error.")]
    IoError(std::io::Error),
    #[error("Ffi calling error.")]
    FfiError(std::ffi::NulError),
    #[error("Failure to initialize CoreClr.")]
    InitializationFailure,
    #[error("Invalid path result.")]
    PathError(String),
    #[error("Pattern error.")]
    PatternError(glob::PatternError)
}

impl From<std::ffi::NulError> for CoreClrError
{
    fn from(error: std::ffi::NulError) -> CoreClrError
    {
        CoreClrError::FfiError(error)
    }
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

impl From<glob::PatternError> for CoreClrError
{
    fn from(error: glob::PatternError) -> CoreClrError
    {
        CoreClrError::PatternError(error)
    }
}
