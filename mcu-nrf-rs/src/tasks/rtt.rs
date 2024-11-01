use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use rtt_target::{rtt_init, ChannelMode, DownChannel};

use crate::utils::signals;

const TASK_ID: &str = "RTT";

#[embassy_executor::task]
pub async fn task(
    mut input: DownChannel
) {
    info!("{}", TASK_ID);

    let mut buf = [0u8; 512];

    loop {
        yield_now().await;
        let count = input.read(&mut buf[..]);
        if count > 0 {
            let str = core::str::from_utf8(&buf[..count]).unwrap().trim_end();
            info!("> {}", str);
            signals::send_cli(str.as_bytes());
        }
    }
}


pub fn init(spawner: Spawner) {
    let channels = rtt_init! {
        up: {
            0: {
                size: 1024,
                name: "defmt"
            }
        }
        down: {
            0: {
                size: 512,
                mode: ChannelMode::BlockIfFull,
                name: "Input"
            }
        }
    };

    rtt_target::set_defmt_channel(channels.up.0);
    spawner.must_spawn(task(channels.down.0));
}