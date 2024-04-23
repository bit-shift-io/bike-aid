
use crate::{
    task_blinker::BlinkerMode,
    signals
};

static TASK_ID : &str = "COMMANDER";

#[embassy_executor::task]
pub async fn commander(
    out_blinker_mode: signals::BlinkerModePub
) {
    out_blinker_mode.publish_immediate(BlinkerMode::OnOffFast);

    log::info!("{} : Entering main loop",TASK_ID);
    loop {
        out_blinker_mode.publish_immediate(BlinkerMode::TwoFast);
        out_blinker_mode.publish_immediate(BlinkerMode::OnOffFast);
    }
}