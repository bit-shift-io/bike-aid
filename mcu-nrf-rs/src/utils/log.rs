use defmt::info;
use rtt_target::UpChannel;
use portable_atomic::{AtomicBool, Ordering};

use super::signals;

static mut CHANNEL: Option<UpChannel> = None;

#[defmt::global_logger]
struct Logger;

/// Sets the channel to use for [`defmt`] macros.
pub fn set_defmt_channel(channel: UpChannel) {
    unsafe { CHANNEL = Some(channel) }
    //rtt_target::set_defmt_channel(channel);
}

/// Global logger lock.
static TAKEN: AtomicBool = AtomicBool::new(false);
static mut CS_RESTORE: critical_section::RestoreState = critical_section::RestoreState::invalid();
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();

unsafe impl defmt::Logger for Logger {
    fn acquire() {
        // safety: Must be paired with corresponding call to release(), see below
        let restore = unsafe { critical_section::acquire() };

        if TAKEN.load(Ordering::Relaxed) {
            panic!("defmt logger taken reentrantly")
        }

        // no need for CAS because interrupts are disabled
        TAKEN.store(true, Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        unsafe { CS_RESTORE = restore };

        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        unsafe { ENCODER.start_frame(do_write) }
    }

    unsafe fn flush() {}

    unsafe fn release() {
        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        ENCODER.end_frame(do_write);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        TAKEN.store(false, Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        let restore = CS_RESTORE;

        // safety: Must be paired with corresponding call to acquire(), see above
        critical_section::release(restore);
    }

    unsafe fn write(bytes: &[u8]) {
        

        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        ENCODER.write(bytes, do_write);
        //panic!("...");
    }
}

fn do_write(bytes: &[u8]) {
    

    unsafe {
        if let Ok(message) = core::str::from_utf8(bytes) {
            if message.contains("CLI") {
                info!("found!");
            };
        }
        
        let channel = core::ptr::addr_of_mut!(CHANNEL);
        if let Some(Some(c)) = channel.as_mut() {
            c.write(bytes);
        }
    }
}