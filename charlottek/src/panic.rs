use core::panic::PanicInfo;

use crate::isa::lp::LpControl;
use crate::logln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    LpControl::halt()
}
