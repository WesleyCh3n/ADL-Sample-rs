use std::{
    os::raw::{c_int, c_void},
    ptr,
};

use windows::core::Result;

use adl::adl_define::*;
use adl::adl_struct::*;

pub unsafe fn print_odn_temp_parameters(
    adl: &ADLLibrary,
    lp_adapter_info: *mut AdapterInfo,
) -> Result<()> {
    let context: *mut c_void = ptr::null_mut();
    let (mut i_supported, mut i_enabled, mut i_version) =
        (c_int::default(), c_int::default(), c_int::default());
    if (*lp_adapter_info.offset(0)).iBusNumber < -1 {
        println!("adapter iBusNumber < -1");
        return Ok(());
    }
    (adl.adl2_overdrive_caps)(
        context,
        (*lp_adapter_info.offset(0)).iAdapterIndex,
        &mut i_supported,
        &mut i_enabled,
        &mut i_version,
    );
    if i_version != 7 {
        println!("i_version != 7, which is {}", i_version);
        return Ok(());
    }
    let mut overdrive_capabilities = ADLODNCapabilitiesX2::default();
    if ADL_OK
        != (adl.adl2_overdriven_capabilitiesx2_get)(
            context,
            (*lp_adapter_info.offset(0)).iAdapterIndex,
            &mut overdrive_capabilities,
        )
    {
        println!("ADL2_OverdriveN_CapabilitiesX2_Get is failed");
    }
    let mut od_npower_control = ADLODNPowerLimitSetting::default();
    if ADL_OK
        != ((adl.adl2_overdriven_powerlimit_get)(
            context,
            (*lp_adapter_info.offset(0)).iAdapterIndex,
            &mut od_npower_control,
        ))
    {
        println!("ADL2_OverdriveN_PowerLimit_Get is failed\n");
        return Ok(());
    }
    let mut temp = c_int::default();
    (adl.adl2_overdriven_temperature_get)(
        context,
        (*lp_adapter_info.offset(0)).iAdapterIndex,
        1,
        &mut temp,
    );
    println!("{:-^40}", "");
    println!("ADL2_OverdriveN_PowerLimit_Get Data");
    println!("{:-^40}", "");
    println!("odNPowerControl.iMode: {:>17}", od_npower_control.iMode);
    println!("Current temperature: {:>17}\u{00B0}C", temp/1000);
    println!("{:-^40}", " POWER ");
    println!(
        "Min: {:>7}, Max: {:>7}, Step: {:>6}",
        overdrive_capabilities.powerTuneTemperature.iMin,
        overdrive_capabilities.powerTuneTemperature.iMax,
        overdrive_capabilities.powerTuneTemperature.iStep
    );
    println!(
        "odNPowerControl.iMaxOperatingTemperature: {}",
        od_npower_control.iMaxOperatingTemperature
    );
    println!("{:-^40}", " TOP LIMITS ");
    println!(
        "Min: {:>7}, Max: {:>7}, Step: {:>6}",
        overdrive_capabilities.power.iMin,
        overdrive_capabilities.power.iMax,
        overdrive_capabilities.power.iStep
    );
    println!(
        "odNPowerControl.iTDPLimit: {:>13}",
        od_npower_control.iTDPLimit
    );
    println!("{:-^40}", "");

    Ok(())
}
