use windows::core::*;

use std::{
    alloc::{alloc, dealloc, Layout},
    intrinsics::transmute,
    mem::{align_of, size_of},
    os::raw::{c_int, c_void},
    ptr,
};

use crate::{
    adl_define::*,
    adl_struct::AdapterInfo,
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
    pub num_adapters: c_int,
    pub lp_adapter_info: *mut AdapterInfo,

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

macro_rules! get_proc {
    ($a:expr,$b:expr) => {
        transmute(get_proc_address($a, $b)?)
    };
}

impl ADL {
    pub unsafe fn new() -> Result<Self> {
        let module = load_library(b"atiadlxx.dll\0")?;
        #[rustfmt::skip]
        let mut adl = ADL {
            module_handle: module,
            lp_adapter_info: std::ptr::null_mut(),
            num_adapters: c_int::default(),
            adl_main_control_create: get_proc!(&module, b"ADL_Main_Control_Create\0"),
            adl_main_control_destroy: get_proc!(&module, b"ADL_Main_Control_Destroy\0"),
            adl_adapter_number_of_adapters_get: get_proc!(&module, b"ADL_Adapter_NumberOfAdapters_Get\0"),
            adl_adapter_adapterinfo_get: get_proc!(&module, b"ADL_Adapter_AdapterInfo_Get\0"),
            adl2_overdrive_caps: get_proc!(&module, b"ADL2_Overdrive_Caps\0"),
            adl2_overdriven_capabilitiesx2_get: get_proc!(&module, b"ADL2_OverdriveN_CapabilitiesX2_Get\0"),
            adl2_overdriven_fancontrol_get: get_proc!(&module, b"ADL2_OverdriveN_FanControl_Get\0"),
            adl2_overdriven_powerlimit_get: get_proc!(&module, b"ADL2_OverdriveN_PowerLimit_Get\0"),
            adl2_overdriven_temperature_get: get_proc!(&module, b"ADL2_OverdriveN_Temperature_Get\0"),
        };

        if ADL_OK != (adl.adl_main_control_create)(adl_main_memory_alloc, 1) {
            return Err(Error::new(HRESULT(1), HSTRING::new()));
        }
        if ADL_OK
            != (adl.adl_adapter_number_of_adapters_get)(&mut adl.num_adapters)
        {
            println!("Cannot get the number of adapters!");
            return Err(Error::new(HRESULT(1), HSTRING::new()));
        }
        if adl.num_adapters < 0 {
            println!("No adapter found!");
            return Err(Error::new(HRESULT(1), HSTRING::new()));
        }
        // let raw_ptr: *mut c_void;
        let all_info_size: usize =
            size_of::<AdapterInfo>() * adl.num_adapters as usize;
        let layout =
            Layout::from_size_align(all_info_size, align_of::<AdapterInfo>())
                .expect("Align AdapterInfo Failed");
        let raw_ptr = alloc(layout) as *mut c_void;
        ptr::write_bytes(raw_ptr, b'\0', all_info_size);
        adl.lp_adapter_info = raw_ptr as _;

        Ok(adl)
    }
}

impl Drop for ADL {
    fn drop(&mut self) {
        unsafe {
            let all_info_size: usize =
                size_of::<AdapterInfo>() * self.num_adapters as usize;
            let layout = Layout::from_size_align(
                all_info_size,
                align_of::<AdapterInfo>(),
            )
            .expect("Align AdapterInfo Failed");
            dealloc(self.lp_adapter_info as *mut u8, layout);
            (self.adl_main_control_destroy)();
            free_library(&self.module_handle);
        }
    }
}
