mod adl;
mod adl_define;
mod adl_struct;
mod winapi;

use std::{
    alloc::{alloc, dealloc, Layout},
    mem::{align_of, size_of},
    os::raw::{c_int, c_void},
};

use windows::core::*;

use adl::*;
use adl_define::*;
use adl_struct::*;
use winapi::*;

fn main() -> Result<()> {
    unsafe { unsafe_main() }
}

unsafe extern "C" fn adl_main_memory_alloc(i_size: c_int) -> *mut c_void {
    let layout =
        Layout::from_size_align_unchecked(i_size as usize, align_of::<c_int>());
    let ptr = alloc(layout) as *mut c_void;
    ptr
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
    if 0 < i_number_adapters {
        println!("size_of AdapterInfo: {}", size_of::<AdapterInfo>());
        let adapter_info_size: usize =
            size_of::<AdapterInfo>() * i_number_adapters as usize;
        let layout = Layout::from_size_align(
            adapter_info_size,
            align_of::<AdapterInfo>(),
        )
        .expect("Align");
        lp_adapter_info_raw = alloc(layout) as _;
        std::ptr::write_bytes(lp_adapter_info_raw, b'\0', adapter_info_size);
        let lp_adapter_info = lp_adapter_info_raw as *mut AdapterInfo;
        (adl.adl_adapter_adapterinfo_get)(
            lp_adapter_info,
            adapter_info_size as i32,
        );
        /* println!(
            "{:#?}",
            std::ffi::CStr::from_ptr(
                (*(lp_adapter_info as *mut AdapterInfo))
                    .strPNPString
                    .as_ptr()
            )
        ); */

        println!(
            "lpAdapterInfo[0].iBusNumber: {}",
            (*lp_adapter_info.offset(0)).iBusNumber
        );

        let context: *mut c_void = std::ptr::null_mut();
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
                println!("ADL2_OverdriveN_CapabilitiesX2_Get is failed\n");
            }
            let mut od_nfan_control = ADLODNFanControl::default();
            if ADL_OK
                != (adl.adl2_overdriven_fancontrol_get)(
                    context,
                    (*lp_adapter_info.offset(0)).iAdapterIndex,
                    &mut od_nfan_control,
                )
            {
                println!("ADL2_OverdriveN_FanControl_Get is failed\n");
            } else {
                println!(
                    "overdrive_capabilities.iFlags: {}",
                    overdrive_capabilities.iFlags
                );
            };
        }

        dealloc(lp_adapter_info_raw as *mut u8, layout);
    }

    (adl.adl_main_control_destroy)();
    free_library(&h_dll);
    Ok(())
}
