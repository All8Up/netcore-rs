//! Search for and manage the assemblies given to CoreClr.
extern crate glob;
use glob::glob;
use std::path::{Path, PathBuf};
use crate::{Error, Result};


/// Library extension for the platform.
#[cfg(target_os = "windows")]
const LIB_EXT: &str = "dll";

/// Library extension for the platform.
#[cfg(target_os = "macos")]
const LIB_EXT: &str = "dylib";

/// Library extension for the platform.
#[cfg(target_os = "linux")]
const LIB_EXT: &str = "so";

/// Container for found assemblies.
#[derive(Debug)]
pub struct Assemblies
{
    assemblies: Vec<PathBuf>
}

impl Assemblies
{
    pub const LIBRARY_EXT: &'static str = LIB_EXT;

    /// New up an empty assembly container.
    pub fn new() -> Self
    {
        Assemblies {
            assemblies: Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool
    {
        self.assemblies.is_empty()
    }

    /// Add all assemblies which are found on the path with the given pattern.
    pub fn add(&mut self, path: &Path, pattern: &str) -> Result<()>
    {
        if let Some(glob_string) = path.join(pattern).to_str()
        {
            for entry in glob(glob_string)?
            {
                if let Ok(p) = entry {
                    self.assemblies.push(p);
                }
            }
            Ok(())
        } else {
            Err(Error::PathError("Invalid path+pattern.".to_string()))
        }
    }
}

impl ToString for Assemblies
{
    fn to_string(&self) -> String
    {
        let mut result = String::new();

        for path in &self.assemblies
        {
            let canonical = path.canonicalize().unwrap();
            result += canonical.to_str().expect("Conversion error.");
            result += ";";
        }
        
        result
    }
}

impl Default for Assemblies
{
    fn default() -> Self
    {
        Assemblies::new()
    }
}
