use std::{
    os::raw::{c_int, c_void},
    ptr,
};

use windows::core::Result;

use adl::adl_struct::*;
use adl::{adl::ADL, adl_define::*};

pub unsafe fn print_odn_fan_parameters(
    adl: &ADL,
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
