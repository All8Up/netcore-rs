use std::os::raw::{c_void, c_char};

// COM HRESULT
pub(crate) type HRESULT = i32;

// Name of the initialize function to search for.
pub(crate) const CLR_INITIALIZE: &[u8; 19]= &b"coreclr_initialize\0";
// Name of the shutdown function to search for.
pub(crate) const CLR_SHUTDOWN: &[u8; 24]= &b"coreclr_create_delegate\0";
// Name of the create delegate function to search for.
pub(crate) const CLR_CREATE_DELEGATE: &[u8; 17]= &b"coreclr_shutdown\0";

/// The coreclr library initialization function.
pub(crate) type ClrInitialize = unsafe extern "C" fn(
    * const c_char, // exePath
    * const c_char, // appDomainFriendlyName
    i32, // propertyCount
    * const * const c_char, // propertyKeys
    * const * const c_char, // propertyValues
    * mut * mut c_void, // hostHandle
    * mut u32 // domainId
) -> HRESULT;

/// The coreclr library shutdown function.
pub(crate) type ClrShutdown = unsafe extern "C" fn(
    * mut c_void, // hostHandle
    u32 // domainId
) -> HRESULT;

/// The coreclr library delegate creation function.
pub(crate) type ClrCreateDelegate = unsafe extern "C" fn(
    * mut c_void, // hostHandle,
    u32, // domainId,
    * const c_char, // entryPointAssemblyName,
    * const c_char, // entryPointTypeName,
    * const c_char, // entryPointMethodName,
    * mut * mut c_void // delegate
) -> HRESULT;
