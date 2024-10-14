use crate::ble::server::{self, Server};
use crate::utils::signals;
use defmt::*;
use embassy_executor::Spawner;
use core::mem;
use nrf_softdevice::ble::advertisement_builder::{Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList};
use nrf_softdevice::ble::{self, gatt_server, peripheral, Connection};
use nrf_softdevice::{raw, Softdevice};
use futures::future::{select, Either};
use futures::pin_mut;

const TASK_ID: &str = "BLUETOOTH";

// softdevice task
#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) {
    sd.run().await;
}

// main task
#[embassy_executor::task]
pub async fn task(
    spawner: Spawner
) {
    info!("{}", TASK_ID);
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();

    // configure bluetooth
    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 16,
            rc_temp_ctiv: 2,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 2, // 6
            event_length: 24, // 24
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 517 }), // 517 is the android default
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t { attr_tab_size: 2048 }), // increase if nomem error, default: attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 1, // 3
            central_role_count: 1, // 3
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: b"BScooter" as *const u8 as _, // TODO: use device name here
            current_len: 8,
            max_len: 8,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&config);
    let server: Server = unwrap!(Server::new(sd));
    unwrap!(spawner.spawn(softdevice_task(sd)));

    info!("{}: address {:X}", TASK_ID, ble::get_address(sd).bytes);

    static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .services_128(
            ServiceList::Incomplete, 
            &[[
                0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x01, 0x00, 0x40, 0x6E
            ]]
        )
        .short_name("BScooter")
        .build();

    static SCAN_DATA: [u8; 0] = [];

    loop {
        let config = peripheral::Config::default();
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected {
            adv_data: &ADV_DATA,
            scan_data: &SCAN_DATA,
        };
        
        // with or without bonding
        let conn: Connection = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);

        // Create two futures:
        //  - My server which allows services to listens for signals and processes them 
        //  - A GATT server listening for events from the connected client.
        let server_future = server::run(&conn, &server);
        //let gatt_future = gatt_server::run(&conn, &server, |_| {});
        let gatt_future = gatt_server::run(&conn, &server, |e| {info!("{}: event : {:?}", TASK_ID, e)});

        pin_mut!(server_future, gatt_future);

        // We are using "select" to wait for either one of the futures to complete.
        // There are some advantages to this approach:
        //  - we only gather data when a client is connected, therefore saving some power.
        //  - when the GATT server finishes operating, our ADC future is also automatically aborted.
        let _ = match select(server_future, gatt_future).await {
            Either::Left((_, _)) => {
                info!("{}: server run encountered an error and stopped!", TASK_ID);
                pub_piezo.publish_immediate(signals::PiezoModeType::Notify);
            }
            Either::Right((e, _)) => {
                info!("{}: gatt_server exited with error: {:?}", TASK_ID, e);
                pub_piezo.publish_immediate(signals::PiezoModeType::Notify);

            }
        };
    }
}