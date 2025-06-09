use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {
        // panic_info.message() 直接返回 PanicMessage 而不是 Option
        let message = panic_info.message();
        println!(
            "Panicked at {}:{}, {}", 
            location.file(), 
            location.line(), 
            message
        );
    } else {
        let message = panic_info.message();
        println!("Panicked: {}", message);
    }
    loop {}
}