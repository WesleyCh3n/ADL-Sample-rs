use std::os::raw::c_void;
use windows::{
    core::*,
    Win32::{
        Foundation::HINSTANCE,
        System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA},
    },
};

pub type ModuleHandle = HINSTANCE;

pub unsafe fn load_library(filename: &[u8]) -> Result<ModuleHandle> {
    LoadLibraryA(PCSTR(filename.as_ptr()))
}

pub unsafe fn free_library(module: &ModuleHandle) -> bool {
    FreeLibrary(module).as_bool()
}

pub unsafe fn get_proc_address(
    module: &ModuleHandle,
    proc_name: &[u8],
) -> Result<*mut c_void> {
    if let Some(ptr) = GetProcAddress(*module, PCSTR(proc_name.as_ptr())) {
        Ok(ptr as _)
    } else {
        Err(Error::from_win32())
    }
}
