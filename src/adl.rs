use windows::core::*;

use std::{
    alloc::{alloc, Layout},
    intrinsics::transmute,
    mem::align_of,
    os::raw::{c_int, c_void},
};

use crate::{
    adl_define::*,
    winapi::{free_library, get_proc_address, load_library, ModuleHandle},
};

unsafe extern "C" fn adl_main_memory_alloc(i_size: c_int) -> *mut c_void {
    let layout =
        Layout::from_size_align_unchecked(i_size as usize, align_of::<c_int>());
    let ptr = alloc(layout) as *mut c_void;
    ptr
}

pub struct ADL {
    pub module_handle: ModuleHandle,
    pub adl_main_control_create: ADL_Main_Control_Create,
    pub adl_main_control_destroy: ADL_Main_Control_Destroy,
    pub adl_adapter_number_of_adapters_get: ADL_ADAPTER_NUMBEROFADAPTERS_GET,
    pub adl_adapter_adapterinfo_get: ADL_Adapter_AdapterInfo_Get,
    pub adl2_overdrive_caps: ADL2_Overdrive_Caps,
    pub adl2_overdriven_capabilitiesx2_get: ADL2_OverdriveN_CapabilitiesX2_Get,
    pub adl2_overdriven_fancontrol_get: ADL2_OverdriveN_FanControl_Get,
    pub adl2_overdriven_powerlimit_get: ADL2_OverdriveN_PowerLimit_Get,
    pub adl2_overdriven_temperature_get: ADL2_OverdriveN_Temperature_Get,
}

impl ADL {
    pub fn new() -> Result<Self> {
        unsafe {
            let module = load_library(b"atiadlxx.dll\0")?;
            let adl = ADL {
                module_handle: module,
                adl_main_control_create: transmute(get_proc_address(
                    &module,
                    b"ADL_Main_Control_Create\0",
                )?),
                adl_main_control_destroy: transmute(get_proc_address(
                    &module,
                    b"ADL_Main_Control_Destroy\0",
                )?),
                adl_adapter_number_of_adapters_get: transmute(
                    get_proc_address(
                        &module,
                        b"ADL_Adapter_NumberOfAdapters_Get\0",
                    )?,
                ),
                adl_adapter_adapterinfo_get: transmute(get_proc_address(
                    &module,
                    b"ADL_Adapter_AdapterInfo_Get\0",
                )?),
                adl2_overdrive_caps: transmute(get_proc_address(
                    &module,
                    b"ADL2_Overdrive_Caps\0",
                )?),
                adl2_overdriven_capabilitiesx2_get: transmute(
                    get_proc_address(
                        &module,
                        b"ADL2_OverdriveN_CapabilitiesX2_Get\0",
                    )?,
                ),
                adl2_overdriven_fancontrol_get: transmute(get_proc_address(
                    &module,
                    b"ADL2_OverdriveN_FanControl_Get\0",
                )?),
                adl2_overdriven_powerlimit_get: transmute(get_proc_address(
                    &module,
                    b"ADL2_OverdriveN_PowerLimit_Get\0",
                )?),
                adl2_overdriven_temperature_get: transmute(get_proc_address(
                    &module,
                    b"ADL2_OverdriveN_Temperature_Get\0",
                )?),
            };
            if ADL_OK != (adl.adl_main_control_create)(adl_main_memory_alloc, 1)
            {
                return Err(Error::new(HRESULT(1), HSTRING::new()));
            }
            Ok(adl)
        }
    }
}

impl Drop for ADL {
    fn drop(&mut self) {
        unsafe {
            (self.adl_main_control_destroy)();
            free_library(&self.module_handle);
        }
    }
}
