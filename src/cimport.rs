#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use std::ffi::CString;

use data::*;
use postprocess::aiPostProcessSteps;

mod raw_assimp {
    use aiScene;
    use std::os::raw::{c_char, c_uint};

    #[link(name = "c:/Projects/rust/assimp/assimp-vc141-mt")]
    extern "C" {
        pub fn aiImportFile(pFile: *const c_char, pFlags: c_uint) -> *const aiScene;
        pub fn aiReleaseImport(pScene: *const aiScene);
    }
}

// --------------------------------------------------------------------------------
/** Reads the given file and returns its content.
 *
 * If the call succeeds, the imported data is returned in an aiScene structure.
 * The data is intended to be read-only, it stays property of the ASSIMP
 * library and will be stable until aiReleaseImport() is called. After you're
 * done with it, call aiReleaseImport() to free the resources associated with
 * this file. If the import fails, NULL is returned instead. Call
 * aiGetErrorString() to retrieve a human-readable error text.
 * @param pFile Path and filename of the file to be imported,
 *   expected to be a null-terminated c-string. NULL is not a valid value.
 * @param pFlags Optional post processing steps to be executed after
 *   a successful import. Provide a bitwise combination of the
 *   #aiPostProcessSteps flags.
 * @return Pointer to the imported data or NULL if the import failed.
 */
pub fn aiImportFile(pFile: &str, pFlags: aiPostProcessSteps) -> *const aiScene {
    let cstr = CString::new(pFile).unwrap();
    unsafe { raw_assimp::aiImportFile(cstr.as_ptr(), pFlags.into()) }
}

// --------------------------------------------------------------------------------
/** Releases all resources associated with the given import process.
 *
 * Call this function after you're done with the imported data.
 * @param pScene The imported data to release. NULL is a valid value.
 */
pub fn aiReleaseImport(pScene: *const aiScene) {
    unsafe {
        raw_assimp::aiReleaseImport(pScene);
    }
}
