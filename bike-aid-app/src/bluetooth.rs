
use std::any::Any;
use std::error::Error;
use std::time::Duration;
use btleplug::api::{bleuuid::BleUuid, Central, CentralEvent, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use tokio::sync::futures;
use tokio::time;
use tokio_stream::{Stream, StreamExt};


/* 
pub async fn get_adapter_list() -> Result<Vec<Adapter>, Box<dyn Error>> {
    // get all adapters
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await.unwrap();
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }
    Ok(adapter_list)
}


pub async fn get_central() -> Result<Adapter, Box<dyn Error>> {
    // get first bluetooth adapters
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await.unwrap();
    Ok(adapters.into_iter().nth(0).unwrap())
}
*/


pub async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}


pub async fn scan_sleep() -> Result<(), Box<dyn Error + Send + Sync>> { // added send sync

    let manager = Manager::new().await?;

    // get the first bluetooth adapter
    // connect to the adapter
    let adapter = get_central(&manager).await;
    println!("Starting scan on {}...", adapter.adapter_info().await?);

    adapter
        .start_scan(ScanFilter::default())
        .await
        .expect("Can't scan BLE adapter for connected devices...");

    time::sleep(Duration::from_secs(10)).await;

    let peripherals = adapter.peripherals().await?;
    if peripherals.is_empty() {
        eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
    } else {
        // All peripheral devices in range
        for peripheral in peripherals.iter() {
            let properties = peripheral.properties().await?;
            let is_connected = peripheral.is_connected().await?;
            let local_name = properties
                .unwrap()
                .local_name
                .unwrap_or(String::from("(peripheral name unknown)"));
            println!(
                "Peripheral {:?} is connected: {:?}",
                local_name, is_connected
            );
            if !is_connected {
                println!("Connecting to peripheral {:?}...", &local_name);
                if let Err(err) = peripheral.connect().await {
                    eprintln!("Error connecting to peripheral, skipping: {}", err);
                    continue;
                }
            }
            let is_connected = peripheral.is_connected().await?;
            println!(
                "Now connected ({:?}) to peripheral {:?}...",
                is_connected, &local_name
            );
            peripheral.discover_services().await?;
            println!("Discover peripheral {:?} services...", &local_name);
            for service in peripheral.services() {
                println!(
                    "Service UUID {}, primary: {}",
                    service.uuid, service.primary
                );
                for characteristic in service.characteristics {
                    println!("  {:?}", characteristic);
                }
            }
            if is_connected {
                println!("Disconnecting from peripheral {:?}...", &local_name);
                peripheral
                    .disconnect()
                    .await
                    .expect("Error disconnecting from BLE peripheral");
            }
        }
    }

    Ok(())
}


pub async fn scan_stream() -> Result<(), Box<dyn Error>> {

    let manager = Manager::new().await?;

    // get the first bluetooth adapter
    // connect to the adapter
    let central = get_central(&manager).await;

    // Each adapter has an event stream, we fetch via events(),
    // simplifying the type, this will return what is essentially a
    // Future<Result<Stream<Item=CentralEvent>>>.
    let mut events = central.events().await?;

    // start scanning for devices
    println!("Starting scan on {}...", central.adapter_info().await?);
    central.start_scan(ScanFilter::default()).await?;


    // Print based on whatever the event receiver outputs. Note that the event
    // receiver blocks, so in a real program, this should be run in its own
    // thread (not task, as this library does not yet use async channels).
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                println!("DeviceDiscovered: {:?}", id);
            }
            CentralEvent::DeviceConnected(id) => {
                println!("DeviceConnected: {:?}", id);
            }
            CentralEvent::DeviceDisconnected(id) => {
                println!("DeviceDisconnected: {:?}", id);
            }
            CentralEvent::ManufacturerDataAdvertisement {
                id,
                manufacturer_data,
            } => {
                println!(
                    "ManufacturerDataAdvertisement: {:?}, {:?}",
                    id, manufacturer_data
                );
            }
            CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                println!("ServiceDataAdvertisement: {:?}, {:?}", id, service_data);
            }
            CentralEvent::ServicesAdvertisement { id, services } => {
                let services: Vec<String> =
                    services.into_iter().map(|s| s.to_short_string()).collect();
                println!("ServicesAdvertisement: {:?}, {:?}", id, services);
            }
            _ => {}
        }
    }

    Ok(())
}