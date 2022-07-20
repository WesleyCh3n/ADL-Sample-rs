use std::{
    alloc::{alloc, dealloc, Layout},
    mem::{align_of, size_of},
    os::raw::{c_int, c_void}, ptr,
};

use windows::core::*;

use adl::adl::*;
use adl::adl_define::*;
use adl::adl_struct::*;
use adl::winapi::*;

fn main() -> Result<()> {
    unsafe { unsafe_main() }
}

unsafe extern "C" fn adl_main_memory_alloc(i_size: c_int) -> *mut c_void {
    let layout =
        Layout::from_size_align_unchecked(i_size as usize, align_of::<c_int>());
    let ptr = alloc(layout) as *mut c_void;
    ptr
}

unsafe fn print_odn_fan_parameters(
    adl: &ADLLibrary,
    lp_adapter_info: *mut AdapterInfo,
) -> Result<()> {
    let context: *mut c_void = ptr::null_mut();
    let (mut i_supported, mut i_enabled, mut i_version) =
        (c_int::default(), c_int::default(), c_int::default());
    (adl.adl2_overdrive_caps)(
        context,
        (*lp_adapter_info.offset(0)).iAdapterIndex,
        &mut i_supported,
        &mut i_enabled,
        &mut i_version,
    );
    println!(
        "i_supported: {}, i_enabled: {}, i_version: {}",
        i_supported, i_enabled, i_version
    );
    if i_version == 7 {
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
        let mut od_nfan_control = ADLODNFanControl::default();
        if ADL_OK
            != (adl.adl2_overdriven_fancontrol_get)(
                context,
                (*lp_adapter_info.offset(0)).iAdapterIndex,
                &mut od_nfan_control,
            )
        {
            println!("ADL2_OverdriveN_FanControl_Get is failed");
        } else {
            if (ADL_ODN_FAN_SPEED_MIN & overdrive_capabilities.iFlags)
                == ADL_ODN_FAN_SPEED_MIN
                || (ADL_ODN_FAN_SPEED_TARGET & overdrive_capabilities.iFlags)
                    == ADL_ODN_FAN_SPEED_TARGET
            {
                println!("{:-^45}", "FAN (Min Fan Limit & Fan Target Speed)");
                println!(
                    "Min: {}, Max: {}, Step: {}",
                    overdrive_capabilities.fanSpeed.iMin,
                    overdrive_capabilities.fanSpeed.iMax,
                    overdrive_capabilities.fanSpeed.iStep
                );
                println!(
                    "odNFanControl.iFanControlMode: {}",
                    od_nfan_control.iFanControlMode
                );
                println!(
                    "odNFanControl.iMinFanLimit: {}",
                    od_nfan_control.iMinFanLimit
                );
                println!(
                    "odNFanControl.iTargetFanSpeed: {}",
                    od_nfan_control.iTargetFanSpeed
                );
                println!("{:-^40}", "");
            }
            if (overdrive_capabilities.iFlags & ADL_ODN_ACOUSTIC_LIMIT_SCLK)
                == ADL_ODN_ACOUSTIC_LIMIT_SCLK
            {
                println!("{:-^40}", "FAN (Min performance)");
                println!(
                    "Min: {:>7}, Max: {:>7}, Step: {:>6}",
                    overdrive_capabilities.minimumPerformanceClock.iMin,
                    overdrive_capabilities.minimumPerformanceClock.iMax,
                    overdrive_capabilities.minimumPerformanceClock.iStep,
                );
                println!(
                    "odNFanControl.iMinPerformanceClock: {:>4}",
                    od_nfan_control.iMinPerformanceClock,
                );
                println!("{:-^40}", "");
            }
            if (overdrive_capabilities.iFlags & ADL_ODN_TEMPERATURE_FAN_MAX)
                == ADL_ODN_TEMPERATURE_FAN_MAX
            {
                println!("{:-^40}", "FAN (Target Temp)");
                println!(
                    "Min: {}, Max: {}, Step: {}",
                    overdrive_capabilities.fanTemperature.iMin,
                    overdrive_capabilities.fanTemperature.iMax,
                    overdrive_capabilities.fanTemperature.iStep,
                );
                println!(
                    "odNFanControl.iTargetTemperature: {:>6}",
                    od_nfan_control.iTargetTemperature,
                );
                println!("{:-^40}", "");
            }
            println!("{:-^40}", "Fan Current Speed");
            println!(
                "odNFanControl.iCurrentFanSpeed: {:>8}",
                od_nfan_control.iCurrentFanSpeed,
            );
            println!(
                "odNFanControl.iCurrentFanSpeedMode: {:>4}",
                od_nfan_control.iCurrentFanSpeedMode,
            );
            println!("{:-^40}", "");
        };
    }
    Ok(())
}

unsafe fn unsafe_main() -> Result<()> {
    let h_dll = load_library(b"atiadlxx.dll\0")?;

    let adl = initialize_adl(&h_dll)?;

    if ADL_OK != (adl.adl_main_control_create)(adl_main_memory_alloc, 1) {
        println!("Failed to initialize nested ADL2 context");
        return Ok(());
    }

    let mut i_number_adapters: c_int = c_int::default();
    if ADL_OK
        != (adl.adl_adapter_number_of_adapters_get)(&mut i_number_adapters)
    {
        println!("Cannot get the number of adapters!");
        return Ok(());
    }
    println!("i_number_adapters: {}", i_number_adapters);

    let lp_adapter_info_raw: *mut c_void;
    println!("size_of AdapterInfo: {}", size_of::<AdapterInfo>());
    let adapter_info_size: usize =
        size_of::<AdapterInfo>() * i_number_adapters as usize;
    let layout =
        Layout::from_size_align(adapter_info_size, align_of::<AdapterInfo>())
            .expect("Align");

    // no adapter found
    if i_number_adapters < 0 {
        return Ok(());
    }

    lp_adapter_info_raw = alloc(layout) as _;
    ptr::write_bytes(lp_adapter_info_raw, b'\0', adapter_info_size);
    let lp_adapter_info = lp_adapter_info_raw as *mut AdapterInfo;
    (adl.adl_adapter_adapterinfo_get)(
        lp_adapter_info,
        adapter_info_size as i32,
    );
    print_odn_fan_parameters(&adl, lp_adapter_info)?;

    dealloc(lp_adapter_info_raw as *mut u8, layout);

    (adl.adl_main_control_destroy)();
    free_library(&h_dll);
    Ok(())
}
