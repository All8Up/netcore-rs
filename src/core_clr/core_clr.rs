use libloading::{Library, Symbol};
use libloading::os::windows::Symbol as RawSymbol;
use std::path::{Path, PathBuf};
use std::ffi::{CString, c_void};
use std::os::raw::c_char;
use super::{Result};

use super::{
    ClrInitialize,
    ClrShutdown,
    ClrCreateDelegate,
    CLR_INITIALIZE,
    CLR_SHUTDOWN,
    CLR_CREATE_DELEGATE
};

#[derive(Debug)]
pub struct CoreClr
{
    library: Library,
    clr_initialize: RawSymbol<ClrInitialize>,
    clr_shutdown: RawSymbol<ClrShutdown>,
    clr_create_delegate: RawSymbol<ClrCreateDelegate>,
    trusted_assemblies: Vec<PathBuf>,

    host_handle: * mut c_void,
    domain_id: u32
}

impl CoreClr
{
    /// Load the coreclr library from the current working directory.
    pub fn load() -> Result<CoreClr>
    {
        let library = libloading::Library::new("coreclr.dll")?;

        Ok(CoreClr {
            clr_initialize: unsafe { Symbol::into_raw(library.get(CLR_INITIALIZE)?) },
            clr_shutdown: unsafe { Symbol::into_raw(library.get(CLR_SHUTDOWN)?) },
            clr_create_delegate: unsafe {Symbol::into_raw(library.get(CLR_CREATE_DELEGATE)?) },
            library: library,
            trusted_assemblies: Vec::new(),
            host_handle: std::ptr::null_mut(),
            domain_id: 0
        })
    }

    /// Load the coreclr library found in the given path.
    pub fn load_from(path: &Path) -> Result<CoreClr>
    {
        let path_lib = path.join("coreclr.dll").canonicalize().unwrap();
        println!("{:?}", path_lib);
        let library = libloading::Library::new(path_lib)?;

        Ok(CoreClr {
            clr_initialize: unsafe { Symbol::into_raw(library.get(CLR_INITIALIZE)?) },
            clr_shutdown: unsafe { Symbol::into_raw(library.get(CLR_SHUTDOWN)?) },
            clr_create_delegate: unsafe {Symbol::into_raw(library.get(CLR_CREATE_DELEGATE)?) },
            library: library,
            trusted_assemblies: Vec::new(),
            host_handle: std::ptr::null_mut(),
            domain_id: 0
        })
    }

    pub fn add_trusted_assemblies_from(&mut self, path: &Path) -> Result<()>
    {
        if path.is_dir()
        {
            for result in std::fs::read_dir(path)? {
                match result {
                    Ok(entry) => {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_file() {
                                // Filter for dll's.
                                // TODO: make the .dll string os specific.
                                if entry.path().extension().unwrap_or(std::ffi::OsStr::new("")) == "dll"
                                {
                                    if let Ok(p) = entry.path().canonicalize()
                                    {
                                        self.trusted_assemblies.push(p);
                                    }
                                }
                            }
                        }
                    },
                    _ => { /* ignoring errors inside the directory. */ }
                }
            }
        }
        Ok(())
    }

    pub fn initialize(&mut self, app_path: &Path, domain_name: &str) -> bool
    {
        // Build the keys.
        // TODO: Make this generic with optional keys.
        let prop_tpa = CString::new("TRUSTED_PLATFORM_ASSEMBLIES").expect("Conversion error.");
        let prop_keys: [* const c_char; 1] = [prop_tpa.as_ptr()];

        // Build out the values.
        let mut tpa_string = String::new();
        for path in &self.trusted_assemblies
        {
            tpa_string += path.to_str().expect("Conversion error.");
            tpa_string += ";";
        }
        let prop_tpa: CString = CString::new(tpa_string).expect("Conversion error.");
        let prop_tpa_values: [* const c_char; 1] = [prop_tpa.as_ptr()];

        // Convert other parameters.
        let exe_path = CString::new(app_path.to_str().expect("Conversion error.")).expect("Conversion error.");
        let friendly_name = CString::new(domain_name).expect("Conversion error.");

        // And call out to the mess.
        unsafe {
            let result = (self.clr_initialize)(
                exe_path.as_ptr(),
                friendly_name.as_ptr(),
                1,
                prop_keys.as_ptr(),
                prop_tpa_values.as_ptr(),
                &mut self.host_handle,
                &mut self.domain_id
            );
            
            result >= 0
        }
    }

    pub fn create_delegate(&self) -> Option<* const c_void>
    {
        let assembly_name = CString::new("ManagedLibrary, Version=1.0.0.0").expect("Conversion error.");
        let type_name = CString::new("ManagedLibrary.ManagedWorker").expect("Conversion error.");
        let method_name = CString::new("DoWork").expect("Conversion error.");

        let mut fn_ptr: * mut c_void = std::ptr::null_mut();
        let result = unsafe { (self.clr_create_delegate)(
            self.host_handle,
            self.domain_id,
            assembly_name.as_ptr(),
            type_name.as_ptr(),
            method_name.as_ptr(),
            &mut fn_ptr
        ) };
        println! ("host: {:?}  result: {} - ptr: {:?}", self.host_handle, result, fn_ptr);

        if result >= 0
        {
            return Some(fn_ptr);
        }

        None
    }
}

impl Drop for CoreClr
{
    fn drop(&mut self)
    {
        // Make sure this happens before the library is dropped.

        // TODO: Fixme Doesn't like this right now.  Hmm....
        //let _result = sunsafe { (self.clr_shutdown)(self.host_handle, self.domain_id) };
    }
}
