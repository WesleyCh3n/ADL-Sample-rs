mod fan;
mod temp;

use std::{
    alloc::{alloc, dealloc, Layout},
    mem::{align_of, size_of},
    os::raw::{c_int, c_void},
    ptr,
};

use windows::core::Result;

use adl::adl::*;
use adl::adl_define::*;
use adl::adl_struct::*;

use self::fan::print_odn_fan_parameters;
use self::temp::print_odn_temp_parameters;

fn main() -> Result<()> {
    unsafe { unsafe_main() }
}

unsafe fn unsafe_main() -> Result<()> {
    let adl = ADL::new()?;

    let mut i_number_adapters: c_int = c_int::default();
    if ADL_OK
        != (adl.adl_adapter_number_of_adapters_get)(&mut i_number_adapters)
    {
        println!("Cannot get the number of adapters!");
        return Ok(());
    }
    // no adapter found
    if i_number_adapters < 0 {
        println!("No adapter found!");
        return Ok(());
    }

    let lp_adapter_info_raw: *mut c_void;
    let all_adapter_info_size: usize =
        size_of::<AdapterInfo>() * i_number_adapters as usize;
    let layout = Layout::from_size_align(
        all_adapter_info_size,
        align_of::<AdapterInfo>(),
    )
    .expect("Align");

    lp_adapter_info_raw = alloc(layout) as _;
    ptr::write_bytes(lp_adapter_info_raw, b'\0', all_adapter_info_size);
    let lp_adapter_info = lp_adapter_info_raw as *mut AdapterInfo;
    (adl.adl_adapter_adapterinfo_get)(
        lp_adapter_info,
        all_adapter_info_size as i32,
    );
    print_odn_fan_parameters(&adl, lp_adapter_info)?;
    print_odn_temp_parameters(&adl, lp_adapter_info)?;

    dealloc(lp_adapter_info_raw as *mut u8, layout);

    /* (adl.adl_main_control_destroy)();
    free_library(&h_dll); */
    Ok(())
}
