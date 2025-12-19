use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ScooterState {
    pub power_on: bool,
    pub alarm_on: bool,
    pub sport_on: bool,
    pub lights_on: bool,
    pub horn_active: bool,
    pub speed: String,
    pub throttle_level: String,
    pub temperature: String,
    pub clock_minutes: String,
    pub clock_hours: String,
    pub cruise_level: i32,
    pub battery_level: String,
    pub battery_power: String,
    pub brake_active: bool,
    pub park_brake_active: bool,
    pub odometer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScooterCommand {
    TogglePower,
    ToggleAlarm,
    ToggleSport,
    ToggleLights,
    CruiseUp,
    CruiseDown,
    Horn,
    SetUart(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BleAction {
    pub bytes: Vec<u8>,
    pub service_uuid: String,
    pub characteristic_uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub state: ScooterState,
    pub log: Option<String>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn parse_characteristic_data(state: ScooterState, uuid_16: String, data: Vec<u8>) -> ParseResult {
    let mut new_state = state;
    let mut log = None;
    
    match uuid_16.as_str() {
        // 1000 series is settings
        "1001" => {
            if !data.is_empty() {
                new_state.power_on = data[0] != 0;
            }
        }
        "1003" => {
            if !data.is_empty() {
                new_state.lights_on = data[0] != 0;
            }
        }
        "1004" => {
            if !data.is_empty() {
                new_state.alarm_on = data[0] != 0;
            }
        }
        "1005" => {
            if !data.is_empty() {
                new_state.sport_on = data[0] != 0;
            }
        }
        
        // 2000 series is data
        "2001" => {
            if !data.is_empty() {
                new_state.speed = format!("{:02}", data[0]);
            }
        }
        "2002" => {
            if data.len() == 2 {
                let v = (data[0] as u16) | ((data[1] as u16) << 8);
                new_state.throttle_level = v.to_string();
            }
        }
        "2004" => {
            if !data.is_empty() {
                new_state.temperature = format!("{:02}", data[0]);
            }
        }
        "2005" => {
            if !data.is_empty() {
                new_state.clock_minutes = format!("{:02}", data[0]);
            }
        }
        "2006" => {
            if !data.is_empty() {
                new_state.clock_hours = format!("{:02}", data[0]);
            }
        }
        "2007" => {
            if !data.is_empty() {
                new_state.brake_active = data[0] != 0;
            }
        }
        "2008" => {
            if !data.is_empty() {
                new_state.park_brake_active = data[0] != 0;
            }
        }
        "2009" => {
            if !data.is_empty() {
                new_state.cruise_level = data[0] as i32;
            }
        }
        
        // Battery
        "2a19" => {
            if !data.is_empty() {
                new_state.battery_level = data[0].to_string();
            }
        }
        "2b05" => {
            if data.len() == 2 {
                let v = (data[0] as u16) | ((data[1] as u16) << 8);
                new_state.battery_power = v.to_string();
            }
        }
        "2003" => {
            if data.len() >= 2 {
                let v = (data[0] as u16) | ((data[1] as u16) << 8);
                new_state.odometer = format!("{:04}", v);
            }
        }
        // UART RX
        "0003" => {
            if let Ok(s) = String::from_utf8(data) {
                log = Some(s);
            }
        }
        _ => {}
    }
    
    ParseResult { state: new_state, log }
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_command_bytes(command: ScooterCommand, current_state: ScooterState) -> Vec<u8> {
    match command {
        ScooterCommand::TogglePower => {
            if current_state.power_on {
                vec![0]
            } else {
                vec![183]
            }
        }
        ScooterCommand::ToggleAlarm => {
            if current_state.alarm_on {
                vec![0]
            } else {
                vec![205]
            }
        }
        ScooterCommand::ToggleSport => {
            if current_state.sport_on {
                vec![0]
            } else {
                vec![1]
            }
        }
        ScooterCommand::ToggleLights => {
            if current_state.lights_on {
                vec![0]
            } else {
                vec![1]
            }
        }
        ScooterCommand::CruiseUp => vec![1],
        ScooterCommand::CruiseDown => vec![0],
        ScooterCommand::Horn => vec![1],
        ScooterCommand::SetUart(s) => s.into_bytes(),
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_command_action(command: ScooterCommand, current_state: ScooterState) -> BleAction {
    let bytes = create_command_bytes(command.clone(), current_state);
    
    let (service, characteristic) = match command {
        ScooterCommand::TogglePower => ("1000", "1001"),
        ScooterCommand::ToggleLights => ("1000", "1003"),
        ScooterCommand::ToggleAlarm => ("1000", "1004"),
        ScooterCommand::ToggleSport => ("1000", "1005"),
        ScooterCommand::CruiseUp | ScooterCommand::CruiseDown => ("1000", "1006"),
        ScooterCommand::Horn => ("1000", "1002"),
        ScooterCommand::SetUart(_) => ("6E400001-B5A3-F393-E0A9-E50E24DCCA9E", "6E400002-B5A3-F393-E0A9-E50E24DCCA9E"),
    };

    let expand_16 = |s: &str| -> String {
        if s.len() == 4 {
            format!("0000{}-0000-1000-8000-00805F9B34FB", s)
        } else {
            s.to_string()
        }
    };

    BleAction {
        bytes,
        service_uuid: expand_16(service),
        characteristic_uuid: expand_16(characteristic),
    }
}
