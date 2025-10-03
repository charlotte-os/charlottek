use core::panic::PanicInfo;

use crate::isa::lp::LogicalProcessor;
use crate::logln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    LogicalProcessor::halt()
}
