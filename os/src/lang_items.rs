use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        // info.message() now returns PanicMessage directly, not Option<PanicMessage>
        let message = info.message();
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            message
        );
    } else {
        // If no location, just output the message
        let message = info.message();
        println!("Panicked: {}", message);
    }

    shutdown()
}