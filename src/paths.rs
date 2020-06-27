use std::path::{Path, PathBuf};
use std::collections::HashSet;
use crate::{Error, Result};

/// Container of paths for use with properties which use them.
#[derive(Debug)]
pub struct Paths
{
    paths: HashSet<PathBuf>
}

impl Paths
{
    pub fn new() -> Self
    {
        Paths {
            paths: HashSet::new()
        }
    }

    pub fn is_empty(&self) -> bool
    {
        self.paths.is_empty()
    }

    pub fn add(&mut self, path: &Path) -> Result<()>
    {
        if path.is_dir()
        {
            if self.paths.insert(path.canonicalize()?)
            {
                Ok(())
            } else {
                Err(Error::Duplicate)
            }
        } else {
            Err(Error::NotADirectory)
        }
    }
}

impl Default for Paths
{
    fn default() -> Self
    {
        Paths {
            paths: HashSet::new()
        }
    }
}

impl ToString for Paths
{
    fn to_string(&self) -> String
    {
        let mut result = String::new();
        for p in &self.paths
        {
            if !result.is_empty()
            {
                result += ";";
            }

            result += p.to_str().unwrap();
        }
        result
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_paths()
    {
        let mut paths = Paths::new();
        assert_eq!(paths.add(Path::new("./tests")).is_ok(), true);
        assert_eq!(paths.add(Path::new("./tests/ManagedLibrary")).is_ok(), true);
    }
}
