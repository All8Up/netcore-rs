//! Manage loading and unloading of the libcoreclr dynamic library.
mod error;
pub use error::CoreClrError as Error;

pub type Result<T> = std::result::Result<T, Error>;

mod types;
pub(crate) use types::*;

mod paths;
pub use paths::Paths;

mod assemblies;
pub use assemblies::Assemblies;

mod properties;
pub use properties::Properties;

mod core_clr;
pub use core_clr::CoreClr;


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::path::Path;

    unsafe extern "C" fn progress(p: i32) -> i32
    {
        println!("ping: {}", p);
        -p
    }

    #[test]
    fn basic_startup() {
        // Load the core clr library from the given path.
        let mut clr = CoreClr::load_from(std::path::Path::new("./tests/ManagedLibrary/deploy"))
            .expect("Coreclr failed to load.");

        // Create the properties for this instance.
        let mut properties = Properties::new();
        let _ = properties.trusted()
            .add(Path::new("./tests/ManagedLibrary/deploy"), &format!("*.{}", Assemblies::EXTENSION));
        assert_eq!(clr.initialize(&std::env::current_dir().unwrap(), "SampleHost", &properties).is_ok(), true);

        // Call the test work.
        type ReportCallback = unsafe extern "C" fn(i32) -> i32;
        type DoWork = unsafe extern "C" fn(
            * const c_char,
            i32,
            i32,
            * const f64,
            ReportCallback
        ) -> * mut c_char;
        let ptr = clr.create_delegate(
            "ManagedLibrary",
            "1.0.0.0",
            "ManagedLibrary",
            "ManagedWorker",
            "DoWork"
        ).unwrap();
        println!("------------: {:?}", ptr);
        let do_work: DoWork = unsafe { std::mem::transmute::<* const c_void, DoWork>(ptr) };

        assert_ne!(ptr, std::ptr::null());
        let data: [f64; 4] = [ 0.0, 0.25, 0.5, 0.75 ];
        let name = CString::new("Test job").expect("Conversion error.");
        let result = unsafe { (do_work)(name.as_ptr(), 5, 5, data.as_ptr(), progress) };
        println!("** result: {:?}", result);
    }
}
