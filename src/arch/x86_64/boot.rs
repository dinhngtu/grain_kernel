#[no_mangle]
pub extern "C" fn x86_64_start() -> ! {
    crate::start();
}
