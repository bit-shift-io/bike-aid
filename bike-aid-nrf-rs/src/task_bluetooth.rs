use crate::ble_server::Server;
use crate::ble_security::Bonder;
use crate::signals;

use defmt::{info, *};
use embassy_executor::Spawner;
use core::mem;
use nrf_softdevice::ble::advertisement_builder::{AdvertisementDataType, Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList, ServiceUuid16};
use nrf_softdevice::ble::{gatt_server, peripheral, Connection};
use nrf_softdevice::{raw, Softdevice};
use static_cell::StaticCell;
use futures::future::{select, Either};
use futures::pin_mut;

const TASK_ID: &str = "BLUETOOTH";
const DEVICE_NAME: &str = "Bronson Scooter";
const PERIPHERAL_REQUESTS_SECURITY: bool = false;



// softdevice task
#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) {
    sd.run().await;
}

// update task
async fn update_task<'a>(server: &'a Server, connection: &'a Connection) {
    let mut sub_throttle_in = signals::THROTTLE_IN.subscriber().unwrap();

    loop {
        let val = sub_throttle_in.next_message_pure().await;

        // try notify, if fails due to other device not allowing, then just set the data
        match server.data.throttle_input_voltage_notify(connection, &val) {
            Ok(_) => (),
            Err(_) => unwrap!(server.data.throttle_input_voltage_set(&val)),
        };
    }
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
            current_len: 15,
            max_len: 15,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&config);
    let server: Server = unwrap!(Server::new(sd));
    unwrap!(spawner.spawn(softdevice_task(sd)));

    let test = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E";
    let byte = test.as_bytes();
    info!("UUID: {}", byte);

    static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .services_16(
            ServiceList::Complete, // or complete
            &[
                ServiceUuid16::BATTERY, 
                ServiceUuid16::USER_DATA, 
                ServiceUuid16::DEVICE_INFORMATION, 
            ]) // TODO: UART service id
        .full_name(DEVICE_NAME)
        //.raw(AdvertisementDataType::APPEARANCE, &[0xC1, 0x03]) // TODO: research
        .build();

    static SCAN_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .services_16(
            ServiceList::Complete,
            &[
                ServiceUuid16::DEVICE_INFORMATION,
                ServiceUuid16::BATTERY,
                ServiceUuid16::USER_DATA,
            ])
        .build();

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
        let conn: Connection = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);
        info!("advertising done!");

        if PERIPHERAL_REQUESTS_SECURITY {
            if let Err(err) = conn.request_security() {
                error!("Security request failed: {:?}", err);
                continue;
            }
        }


        // We have a GATT connection. Now we will create two futures:
        //  - An infinite loop gathering data from the ADC and notifying the clients.
        //  - A GATT server listening for events from the connected client.
        //
        // Event enums (ServerEvent's) are generated by nrf_softdevice::gatt_server
        // proc macro when applied to the Server struct above
        let update_fut = update_task(&server, &conn);
        let gatt_fut = gatt_server::run(&conn, &server, |_| {});
        /*
        let gatt_fut = gatt_server::run(&conn, &server, |e| match e {
            ServerEvent::Bas(e) => match e {
                BatteryServiceEvent::BatteryLevelCccdWrite { notifications } => {
                    info!("battery notifications: {}", notifications)
                }
            },
        }); */

        pin_mut!(update_fut, gatt_fut);

        // We are using "select" to wait for either one of the futures to complete.
        // There are some advantages to this approach:
        //  - we only gather data when a client is connected, therefore saving some power.
        //  - when the GATT server finishes operating, our ADC future is also automatically aborted.
        let _ = match select(update_fut, gatt_fut).await {
            Either::Left((_, _)) => {
                info!("BLE update task encountered an error and stopped!")
            }
            Either::Right((e, _)) => {
                info!("gatt_server run exited with error: {:?}", e);
            }
        };

        
        // Run the GATT server on the connection. This returns when the connection gets disconnected.
        //let e = gatt_server::run(&conn, &server, |_| {}).await;
        //info!("gatt_server run exited with error: {:?}", e);


    }
}