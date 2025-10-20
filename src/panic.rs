use core::panic::PanicInfo;
use std::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(&l) = _info.location() {
        println!(
            "At file: {}, line {}, column {}",
            l.file(),
            l.line(),
            l.column()
        );
    }
    println!(
        "Panicked: {}",
        _info.message().as_str().unwrap_or("No message...")
    );
    loop {}
}
