#![allow(non_camel_case_types)]
#![allow(dead_code)]
use std::os::raw::{c_int, c_void};

use crate::adl_struct::{
    ADLODNCapabilitiesX2, ADLODNFanControl, AdapterInfo, ADL_CONTEXT_HANDLE,
};

pub const ADL_OK: c_int = 0;
pub const ADL_MAX_PATH: usize = 256;

pub const ADL_ODN_SCLK_DPM: c_int = 1 << 0;
pub const ADL_ODN_MCLK_DPM: c_int = 1 << 1;
pub const ADL_ODN_SCLK_VDD: c_int = 1 << 2;
pub const ADL_ODN_MCLK_VDD: c_int = 1 << 3;
pub const ADL_ODN_FAN_SPEED_MIN: c_int = 1 << 4;
pub const ADL_ODN_FAN_SPEED_TARGET: c_int = 1 << 5;
pub const ADL_ODN_ACOUSTIC_LIMIT_SCLK: c_int = 1 << 6;
pub const ADL_ODN_TEMPERATURE_FAN_MAX: c_int = 1 << 7;
pub const ADL_ODN_TEMPERATURE_SYSTEM: c_int = 1 << 8;
pub const ADL_ODN_POWER_LIMIT: c_int = 1 << 9;
pub const ADL_ODN_SCLK_AUTO_LIMIT: c_int = 1 << 10;
pub const ADL_ODN_MCLK_AUTO_LIMIT: c_int = 1 << 11;
pub const ADL_ODN_SCLK_DPM_MASK_ENABLE: c_int = 1 << 12;
pub const ADL_ODN_MCLK_DPM_MASK_ENABLE: c_int = 1 << 13;
pub const ADL_ODN_MCLK_UNDERCLOCK_ENABLE: c_int = 1 << 14;
pub const ADL_ODN_SCLK_DPM_THROTTLE_NOTIFY: c_int = 1 << 15;
pub const ADL_ODN_POWER_UTILIZATION: c_int = 1 << 16;
pub const ADL_ODN_PERF_TUNING_SLIDER: c_int = 1 << 17;
pub const ADL_ODN_REMOVE_WATTMAN_PAGE: c_int = 1 << 31;

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
