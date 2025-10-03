use core::panic::PanicInfo;

use crate::cpu::isa::lp::*;
use crate::logln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    LogicalProcessor::halt()
}
