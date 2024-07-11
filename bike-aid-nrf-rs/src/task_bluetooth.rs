use crate::ble_server::Server;
use crate::ble_security::Bonder;

use defmt::{info, *};
use embassy_executor::Spawner;
use core::mem;
use nrf_softdevice::ble::advertisement_builder::{Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList, ServiceUuid16};
use nrf_softdevice::ble::{gatt_server, peripheral};
use nrf_softdevice::{raw, Softdevice};
use static_cell::StaticCell;

const TASK_ID: &str = "BLUETOOTH";
const DEVICE_NAME: &str = "Bronson Scooter";
const PERIPHERAL_REQUESTS_SECURITY: bool = false;


// softdevice task
#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) {
    sd.run().await;
}


// main task
#[embassy_executor::task]
pub async fn bluetooth (
    spawner: Spawner
) {
    info!("{}: start", TASK_ID);

    // configure bluetooth
    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 16,
            rc_temp_ctiv: 2,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 6,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 3,
            central_role_count: 3,
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: b"Bronson Scooter" as *const u8 as _, // TODO: use device name here
            current_len: 9,
            max_len: 9,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&config);
    let server = unwrap!(Server::new(sd));
    unwrap!(spawner.spawn(softdevice_task(sd)));

    static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .services_16(ServiceList::Complete, &[ServiceUuid16::BATTERY])
        .full_name(DEVICE_NAME)
        .build();

    static SCAN_DATA: [u8; 0] = [];

    // bonder / security
    static BONDER: StaticCell<Bonder> = StaticCell::new();
    let bonder = BONDER.init(Bonder::default());

    info!("{}: loop", TASK_ID);
    loop {
        let config = peripheral::Config::default();
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected {
            adv_data: &ADV_DATA,
            scan_data: &SCAN_DATA,
        };
        
        // with or without bonding
        //let conn = unwrap!(peripheral::advertise_pairable(sd, adv, &config, bonder).await);
        let conn = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);
        info!("advertising done!");

        if PERIPHERAL_REQUESTS_SECURITY {
            if let Err(err) = conn.request_security() {
                error!("Security request failed: {:?}", err);
                continue;
            }
        }

        // Run the GATT server on the connection. This returns when the connection gets disconnected.
        let e = gatt_server::run(&conn, &server, |_| {}).await;
        info!("gatt_server run exited with error: {:?}", e);
    }
}