/*
The communication channel type commonly used is the Embassy PubSubChannel.
We use it to pass messages between tasks.
*/

use embassy_sync::{
    pubsub::{PubSubChannel,Publisher,Subscriber},
    blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex}
};

// Take a value from a channel if it exists and assign it to the existing variable
pub fn try_assign_from_channel<const CAP : usize, const SUBS : usize, const PUBS : usize, M:RawMutex, T:Clone>
(channel : &mut Subscriber<M,T,CAP,SUBS,PUBS>, variable : &mut T) {
    if let Some(new_value) = channel.try_next_message_pure() { *variable = new_value }
}

type ChannelMutex = CriticalSectionRawMutex;

// Short-hand type alias for PubSubChannel
type Pub<T,const N: usize> = Publisher<'static,ChannelMutex,T,1,N,1>;
type Sub<T,const N: usize> = Subscriber<'static,ChannelMutex,T,1,N,1>;
type Ch<T,const N: usize> = PubSubChannel<ChannelMutex,T,1,N,1>;


const BLINKER_MODE_NUM: usize = 1;
pub type BlinkerModeType = crate::task_blinker::BlinkerMode;
pub type BlinkerModePub = Pub<BlinkerModeType,BLINKER_MODE_NUM>;
pub type BlinkerModeSub = Sub<BlinkerModeType,BLINKER_MODE_NUM>;
pub static BLINKER_MODE : Ch<BlinkerModeType,BLINKER_MODE_NUM> = PubSubChannel::new();

/* DO NOT MODIFY TEMPLATE
const CH_PROTOTYPE_X_NUM: usize = 1;
pub type ChPrototypeXType = bool;
pub type ChPrototypeXPub = Pub<ChPrototypeXType,1,CH_PROTOTYPE_X_NUM,1>; // publisher
pub type ChPrototypeXSub = Sub<ChPrototypeXType,1,CH_PROTOTYPE_X_NUM,1>; // subscriber
pub static CH_PROTOTYPE_X : Ch<ChPrototypeXType,1,CH_PROTOTYPE_X_NUM,1> = PubSubChannel::new(); // channel
*/