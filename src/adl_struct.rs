#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::os::raw::{c_char, c_int, c_void};

use crate::adl_define::*;

pub type ADL_CONTEXT_HANDLE = *mut c_void;

pub struct ADLLibrary {
    pub adl_main_control_create: ADL_Main_Control_Create,
    pub adl_main_control_destroy: ADL_Main_Control_Destroy,
    pub adl_adapter_number_of_adapters_get: ADL_ADAPTER_NUMBEROFADAPTERS_GET,
    pub adl_adapter_adapterinfo_get: ADL_Adapter_AdapterInfo_Get,
    pub adl2_overdrive_caps: ADL2_Overdrive_Caps,
    pub adl2_overdriven_capabilitiesx2_get: ADL2_OverdriveN_CapabilitiesX2_Get,
    pub adl2_overdriven_fancontrol_get: ADL2_OverdriveN_FanControl_Get,
}

#[derive(Debug)]
#[repr(C)]
pub struct AdapterInfo {
    pub iSize: c_int,
    pub iAdapterIndex: c_int,
    pub strUDID: [c_char; ADL_MAX_PATH],
    pub iBusNumber: c_int,
    pub iDeviceNumber: c_int,
    pub iFunctionNumber: c_int,
    pub iVendorID: c_int,
    pub strAdapterName: [c_char; ADL_MAX_PATH],
    pub strDisplayName: [c_char; ADL_MAX_PATH],
    pub iPresent: c_int,
    pub iExist: c_int,
    pub strDriverPath: [c_char; ADL_MAX_PATH],
    pub strDriverPathExt: [c_char; ADL_MAX_PATH],
    pub strPNPString: [c_char; ADL_MAX_PATH],
    pub iOSDisplayIndex: c_int,
}

impl Default for AdapterInfo {
    fn default() -> AdapterInfo {
        AdapterInfo {
            iSize: 0,
            iAdapterIndex: 0,
            strUDID: [0; ADL_MAX_PATH],
            iBusNumber: 0,
            iDeviceNumber: 0,
            iFunctionNumber: 0,
            iVendorID: 0,
            strAdapterName: [0; ADL_MAX_PATH],
            strDisplayName: [0; ADL_MAX_PATH],
            iPresent: 0,
            iExist: 0,
            strDriverPath: [0; ADL_MAX_PATH],
            strDriverPathExt: [0; ADL_MAX_PATH],
            strPNPString: [0; ADL_MAX_PATH],
            iOSDisplayIndex: 0,
        }
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ADLODNParameterRange
{
    pub iMode: c_int,
    pub iMin: c_int,
    pub iMax: c_int,
    pub iStep: c_int,
    pub iDefault: c_int,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ADLODNCapabilitiesX2
{
    pub iMaximumNumberOfPerformanceLevels: c_int,
    pub iFlags: c_int,
    pub sEngineClockRange: ADLODNParameterRange,
    pub sMemoryClockRange: ADLODNParameterRange,
    pub svddcRange: ADLODNParameterRange,
    pub power: ADLODNParameterRange,
    pub powerTuneTemperature: ADLODNParameterRange,
    pub fanTemperature: ADLODNParameterRange,
    pub fanSpeed: ADLODNParameterRange,
    pub minimumPerformanceClock: ADLODNParameterRange,
    pub throttleNotificaion: ADLODNParameterRange,
    pub autoSystemClock: ADLODNParameterRange,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ADLODNFanControl
{
    pub iMode: c_int,
    pub iFanControlMode: c_int,
    pub iCurrentFanSpeedMode: c_int,
    pub iCurrentFanSpeed: c_int,
    pub iTargetFanSpeed: c_int,
    pub iTargetTemperature: c_int,
    pub iMinPerformanceClock: c_int,
    pub iMinFanLimit: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct ADLODNPerformanceLevelX2 {
    pub iClock: c_int,
    pub iVddc: c_int,
    pub iEnabled: c_int,
    pub iControl: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct ADLODNPerformanceLevelsX2
{
    pub iSize: c_int,
    pub iMode: c_int,
    pub iNumberOfPerformanceLevels: c_int,
    pub aLevels: [ADLODNPerformanceLevelX2; 1],
}

