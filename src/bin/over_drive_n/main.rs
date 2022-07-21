mod fan;
mod temp;

use windows::core::Result;

use adl::adl::ADL;

use self::fan::print_odn_fan_parameters;
use self::temp::print_odn_temp_parameters;

fn main() -> Result<()> {
    unsafe { unsafe_main() }
}

unsafe fn unsafe_main() -> Result<()> {
    let adl = ADL::new()?;

    print_odn_fan_parameters(&adl)?;
    print_odn_temp_parameters(&adl)?;

    Ok(())
}
