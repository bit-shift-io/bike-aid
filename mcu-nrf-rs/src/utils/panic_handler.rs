use core::panic::PanicInfo;
use cortex_m::asm::nop;
use panic_persist as _;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    defmt::error!("{}", defmt::Display2Format(info));
    panic_persist::report_panic_info(info);

    for _ in 0..2_000_000 { // delay before reset
        nop()
    }
    cortex_m::peripheral::SCB::sys_reset();
}