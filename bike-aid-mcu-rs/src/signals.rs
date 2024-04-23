use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};

type ChannelMutex = CriticalSectionRawMutex;
pub static TEST_CHANNEL: PubSubChannel<ChannelMutex, u32, 2, 2, 2> = PubSubChannel::new();