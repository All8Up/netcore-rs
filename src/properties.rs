use std::ffi::CString;
use std::os::raw::c_char;
use crate::{Assemblies, Paths};

const TRUSTED_PLATFORM_ASSEMBLIES: &str = "TRUSTED_PLATFORM_ASSEMBLIES";
const APP_PATHS: &str = "APP_PATHS";
const APP_NI_PATHS: &str = "APP_NI_PATHS";
const NATIVE_DLL_SEARCH_DIRECTORIES: &str = "NATIVE_DLL_SEARCH_DIRECTORIES";
const PLATFORM_RESOURCE_ROOTS: &str = "PLATFORM_RESOURCE_ROOTS";

#[derive(Debug)]
pub(crate) struct PropCStrings
{
    key_strings: Vec<CString>,
    value_strings: Vec<CString>,
    pub keys: Vec<* const c_char>,
    pub values: Vec<* const c_char>
}

#[derive(Debug)]
pub struct Properties
{
    trusted: Assemblies,
    app: Paths,
    native: Paths,
    pinvoke: Paths,
    resource: Paths
}

impl Properties
{
    pub fn new() -> Self
    {
        Properties {
            trusted: Assemblies::new(),
            app: Paths::new(),
            native: Paths::new(),
            pinvoke: Paths::new(),
            resource: Paths::new()
        }
    }

    pub fn trusted(&mut self) -> &mut Assemblies
    {
        &mut self.trusted
    }

    pub fn application(&mut self) -> &mut Paths
    {
        &mut self.app
    }

    pub fn native(&mut self) -> &mut Paths
    {
        &mut self.native
    }

    pub fn pinvoke(&mut self) -> &mut Paths
    {
        &mut self.pinvoke
    }

    pub fn resource(&mut self) -> &mut Paths
    {
        &mut self.resource
    }

    pub(crate) fn to_cstrings(&self) -> PropCStrings
    {
        let mut result = PropCStrings {
            key_strings: Vec::new(),
            value_strings: Vec::new(),
            keys: Vec::new(),
            values: Vec::new()
        };

        if !self.trusted.is_empty()
        {
            result.key_strings.push(CString::new(TRUSTED_PLATFORM_ASSEMBLIES).unwrap());
            result.value_strings.push(CString::new(self.trusted.to_string()).unwrap());
        }

        if !self.app.is_empty()
        {
            result.key_strings.push(CString::new(APP_PATHS).unwrap());
            result.value_strings.push(CString::new(self.app.to_string()).unwrap());
        }

        if !self.native.is_empty()
        {
            result.key_strings.push(CString::new(APP_NI_PATHS).unwrap());
            result.value_strings.push(CString::new(self.native.to_string()).unwrap());
        }

        if !self.pinvoke.is_empty()
        {
            result.key_strings.push(CString::new(NATIVE_DLL_SEARCH_DIRECTORIES).unwrap());
            result.value_strings.push(CString::new(self.pinvoke.to_string()).unwrap());
        }

        if !self.resource.is_empty()
        {
            result.key_strings.push(CString::new(PLATFORM_RESOURCE_ROOTS).unwrap());
            result.value_strings.push(CString::new(self.resource.to_string()).unwrap());
        }

        // Build the key/value pointer vectors.
        for (index, key) in result.key_strings.iter().enumerate()
        {
            result.keys.push(key.as_ptr());
            result.values.push(result.value_strings[index].as_ptr());
        }

        result
    }
}
