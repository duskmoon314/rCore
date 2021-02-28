use crate::{console::ANSICON, sbi::shutdown};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println_colorized!(
            "Panicked at {}:{} {}",
            ANSICON::FgRed,
            ANSICON::BgDefault,
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println_colorized!(
            "Panicked: {}",
            ANSICON::FgRed,
            ANSICON::BgDefault,
            info.message().unwrap()
        );
    }
    shutdown()
}
