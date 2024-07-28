
use defmt::{info, *};
use core::cell::{Cell, RefCell};
use nrf_softdevice::ble::security::{IoCapabilities, SecurityHandler};
use nrf_softdevice::ble::{Connection, EncryptionInfo, IdentityKey, MasterId, PasskeyReply};
use nrf_softdevice::ble::gatt_server;


// security bonding
#[derive(Debug, Clone, Copy)]
struct Peer {
    master_id: MasterId,
    key: EncryptionInfo,
    peer_id: IdentityKey,
}

pub struct Bonder {
    peer: Cell<Option<Peer>>,
    sys_attrs: RefCell<heapless::Vec<u8, 62>>,
}

impl Default for Bonder {
    fn default() -> Self {
        Bonder {
            peer: Cell::new(None),
            sys_attrs: Default::default(),
        }
    }
}

impl SecurityHandler for Bonder {
    fn io_capabilities(&self) -> IoCapabilities {
        IoCapabilities::None
        //IoCapabilities::DisplayOnly
        //IoCapabilities::KeyboardOnly
    }

    fn can_bond(&self, _conn: &Connection) -> bool {
        true
    }

    /// Display `passkey` to the user for confirmation on the remote device.
    /// Must be implemented if [`io_capabilities()`][Self::io_capabilities] is one of `DisplayOnly`, `DisplayYesNo`, or `KeyboardDisplay`.
    fn display_passkey(&self, passkey: &[u8; 6]) {
        info!("The passkey is \"{:a}\"", passkey)
    }

    /// Allow the user to enter a passkey displayed on the remote device.
    /// Must be implemented if [`io_capabilities()`][Self::io_capabilities] is one of `KeyboardOnly` or `KeyboardDisplay`.
    fn enter_passkey(&self, _reply: PasskeyReply) {
        info!("todo: passkey fixed here?");
        //PasskeyReply::from("1234");
    }

    fn on_bonded(&self, _conn: &Connection, master_id: MasterId, key: EncryptionInfo, peer_id: IdentityKey) {
        debug!("storing bond for: id: {}, key: {}", master_id, key);

        // In a real application you would want to signal another task to permanently store the keys in non-volatile memory here.
        self.sys_attrs.borrow_mut().clear();
        self.peer.set(Some(Peer {
            master_id,
            key,
            peer_id,
        }));
    }

    fn get_key(&self, _conn: &Connection, master_id: MasterId) -> Option<EncryptionInfo> {
        debug!("getting bond for: id: {}", master_id);

        self.peer
            .get()
            .and_then(|peer| (master_id == peer.master_id).then_some(peer.key))
    }

    fn save_sys_attrs(&self, conn: &Connection) {
        debug!("saving system attributes for: {}", conn.peer_address());

        if let Some(peer) = self.peer.get() {
            if peer.peer_id.is_match(conn.peer_address()) {
                let mut sys_attrs = self.sys_attrs.borrow_mut();
                let capacity = sys_attrs.capacity();
                unwrap!(sys_attrs.resize(capacity, 0));
                let len = unwrap!(gatt_server::get_sys_attrs(conn, &mut sys_attrs)) as u16;
                sys_attrs.truncate(usize::from(len));
                // In a real application you would want to signal another task to permanently store sys_attrs for this connection's peer
            }
        }
    }

    fn load_sys_attrs(&self, conn: &Connection) {
        let addr = conn.peer_address();
        debug!("loading system attributes for: {}", addr);

        let attrs = self.sys_attrs.borrow();
        // In a real application you would search all stored peers to find a match
        let attrs = if self.peer.get().map(|peer| peer.peer_id.is_match(addr)).unwrap_or(false) {
            (!attrs.is_empty()).then_some(attrs.as_slice())
        } else {
            None
        };

        unwrap!(gatt_server::set_sys_attrs(conn, attrs));
    }
}
