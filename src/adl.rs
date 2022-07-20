use windows::core::*;

use std::intrinsics::transmute;

use crate::{
    adl_struct::ADLLibrary,
    winapi::{get_proc_address, ModuleHandle},
};

pub unsafe fn initialize_adl(module: &ModuleHandle) -> Result<ADLLibrary> {
    Ok(ADLLibrary {
        adl_main_control_create: transmute(get_proc_address(
            &module,
            b"ADL_Main_Control_Create\0",
        )?),
        adl_main_control_destroy: transmute(get_proc_address(
            &module,
            b"ADL_Main_Control_Destroy\0",
        )?),
        adl_adapter_number_of_adapters_get: transmute(get_proc_address(
            &module,
            b"ADL_Adapter_NumberOfAdapters_Get\0",
        )?),
        adl_adapter_adapterinfo_get: transmute(get_proc_address(
            &module,
            b"ADL_Adapter_AdapterInfo_Get\0",
        )?),
        adl2_overdrive_caps: transmute(get_proc_address(
            &module,
            b"ADL2_Overdrive_Caps\0",
        )?),
        adl2_overdriven_capabilitiesx2_get: transmute(get_proc_address(
            &module,
            b"ADL2_OverdriveN_CapabilitiesX2_Get\0",
        )?),
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
        )?)
    })
}
