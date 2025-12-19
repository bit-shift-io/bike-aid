use btleplug::api::{Central, Manager as _, ScanFilter, Peripheral};
use btleplug::platform::{Adapter, Manager};
use std::time::Duration;
use tokio::time;

pub async fn start_scan() -> anyhow::Result<Vec<String>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    
    if adapters.is_empty() {
        return Err(anyhow::anyhow!("No Bluetooth adapters found on this device. Please ensure Bluetooth is enabled and the app has permissions."));
    }

    let central = adapters.into_iter().next().unwrap();

    // Start scanning
    central.start_scan(ScanFilter::default()).await?;
    
    // Wait for discovery to populate
    time::sleep(Duration::from_secs(5)).await;

    let peripherals = central.peripherals().await?;
    let mut names = Vec::new();

    for peripheral in peripherals {
        if let Ok(Some(properties)) = peripheral.properties().await {
            let name = properties.local_name
                .or(properties.class_of_device.map(|c| format!("Device Class {}", c)))
                .unwrap_or_else(|| "Unknown Device".to_string());
            names.push(name);
        }
    }

    // Stop to save battery
    let _ = central.stop_scan().await;

    Ok(names)
}
