//! Utility to load and initialize the core clr library.
use libloading::{Library, Symbol};
use libloading::os::windows::Symbol as RawSymbol;
use std::path::Path;
use std::ffi::{CString, c_void};
use log::{info};

use crate::{
    Result,
    Error,
    Properties,
    ClrInitialize,
    ClrShutdown,
    ClrCreateDelegate,
    CLR_INITIALIZE,
    CLR_SHUTDOWN,
    CLR_CREATE_DELEGATE
};

/// The coreclr library to load for this platform.
#[cfg(target_os = "windows")]
const CORECLR_LIB: &str = "coreclr.dll";

/// The coreclr library to load for this platform.
#[cfg(target_os = "macos")]
const CORECLR_LIB: &str = "libcoreclr.dylib";

/// The coreclr library to load for this platform.
#[cfg(target_os = "linux")]
const CORECLR_LIB: &str = "libcoreclr.so";


/// Structure to manage the core clr library and initialization data.
#[derive(Debug)]
pub struct CoreClr
{
    library: Library,
    clr_initialize: RawSymbol<ClrInitialize>,
    clr_shutdown: RawSymbol<ClrShutdown>,
    clr_create_delegate: RawSymbol<ClrCreateDelegate>,

    host_handle: * mut c_void,
    domain_id: u32
}

impl CoreClr
{
    /// Load the coreclr library from the current working directory.
    pub fn load() -> Result<CoreClr>
    {
        let library = libloading::Library::new(CORECLR_LIB)?;

        Ok(CoreClr {
            clr_initialize: unsafe { Symbol::into_raw(library.get(CLR_INITIALIZE)?) },
            clr_shutdown: unsafe { Symbol::into_raw(library.get(CLR_SHUTDOWN)?) },
            clr_create_delegate: unsafe {Symbol::into_raw(library.get(CLR_CREATE_DELEGATE)?) },
            library,
            host_handle: std::ptr::null_mut(),
            domain_id: 0
        })
    }

    /// Load the coreclr library found in the given path.
    pub fn load_from(path: &Path) -> Result<CoreClr>
    {
        let path_lib = path.join(CORECLR_LIB).canonicalize().unwrap();
        let library = libloading::Library::new(path_lib)?;

        Ok(CoreClr {
            clr_initialize: unsafe { Symbol::into_raw(library.get(CLR_INITIALIZE)?) },
            clr_shutdown: unsafe { Symbol::into_raw(library.get(CLR_SHUTDOWN)?) },
            clr_create_delegate: unsafe {Symbol::into_raw(library.get(CLR_CREATE_DELEGATE)?) },
            library,
            host_handle: std::ptr::null_mut(),
            domain_id: 0
        })
    }

    /// Initialize the coreclr library.
    pub fn initialize(&mut self, app_path: &Path, domain_name: &str, properties: &Properties) -> Result<()>
    {
        // Convert other parameters.
        let exe_path = CString::new(app_path.to_str().expect("Conversion error."))?;
        let friendly_name = CString::new(domain_name)?;

        // Get the property strings.
        let prop_cstrings = properties.to_cstrings();

        // And call out to the mess.
        unsafe {
            let result = (self.clr_initialize)(
                exe_path.as_ptr(),
                friendly_name.as_ptr(),
                prop_cstrings.keys.len() as i32,
                prop_cstrings.keys.as_ptr(),
                prop_cstrings.values.as_ptr(),
                &mut self.host_handle,
                &mut self.domain_id
            );
            
            if result >= 0
            {
                return Ok(());
            }
            Err(Error::InitializationFailure)
        }
    }

    /// Create a delegate from the loaded assemblies.
    /// TODO: Should be a generic that takes the fn signature, but I can't
    /// quite figure out how to constrain it properly so I can perform the
    /// transform.
    pub fn create_delegate(
        &self,
        lib_name: &str,
        version: &str,
        namespace: &str,
        class: &str,
        fn_name: &str
    ) -> Result<* const c_void>
    {
        // Format the assembly and type strings.
        let assembly_str = format!("{}, Version={}", lib_name, version);
        let type_name_str = format!("{}.{}", namespace, class);

        // Build CStrings for the function call.
        let assembly_name = CString::new(assembly_str)?;
        let type_name = CString::new(type_name_str)?;
        let method_name = CString::new(fn_name)?;

        // Call out to clr to fetch the delegate.
        let mut fn_ptr: * mut c_void = std::ptr::null_mut();
        let result = unsafe { (self.clr_create_delegate)(
            self.host_handle,
            self.domain_id,
            assembly_name.as_ptr(),
            type_name.as_ptr(),
            method_name.as_ptr(),
            &mut fn_ptr
        ) };

        if result >= 0
        {
            return Ok(fn_ptr);
        }

        // TODO: If there are any hresult errors that can be returned,
        // break them down in the result.
        Err(Error::NotFound)
    }
}

impl Drop for CoreClr
{
    fn drop(&mut self)
    {
        // Make sure this happens before the library is dropped.
        let _result = unsafe { (self.clr_shutdown)(self.host_handle, self.domain_id) };

        // Force the library drop.
        unsafe { std::ptr::drop_in_place(&mut self.library) };
    }
}
