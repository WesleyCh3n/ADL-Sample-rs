#![allow(non_camel_case_types)]
use std::os::raw::{c_int, c_void};

use crate::adl_struct::{
    ADLODNCapabilitiesX2, ADLODNFanControl, AdapterInfo, ADL_CONTEXT_HANDLE,
};

pub const ADL_OK: c_int = 0;
pub const ADL_MAX_PATH: usize = 256;

pub type ADL_MAIN_MALLOC_CALLBACK = unsafe extern "C" fn(c_int) -> *mut c_void;

pub type ADL_ADAPTER_NUMBEROFADAPTERS_GET = fn(*mut c_int) -> c_int;
pub type ADL_Main_Control_Create = fn(ADL_MAIN_MALLOC_CALLBACK, c_int) -> c_int;
pub type ADL_Main_Control_Destroy = fn() -> c_int;
pub type ADL_Adapter_AdapterInfo_Get = fn(*mut AdapterInfo, c_int) -> c_int;

pub type ADL2_Overdrive_Caps =
    fn(ADL_CONTEXT_HANDLE, c_int, *mut c_int, *mut c_int, *mut c_int) -> c_int;

pub type ADL2_OverdriveN_CapabilitiesX2_Get =
    fn(ADL_CONTEXT_HANDLE, c_int, *mut ADLODNCapabilitiesX2) -> c_int;

pub type ADL2_OverdriveN_FanControl_Get =
    fn(ADL_CONTEXT_HANDLE, c_int, *mut ADLODNFanControl) -> c_int;
