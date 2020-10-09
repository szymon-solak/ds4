use libudev::{self, Error};
use regex::Regex;

#[derive(Debug)]
pub enum ConnectionType {
    Usb,
    Bluetooth,
    Unknown,
}

pub struct HidrawDevice {
    pub name: String,
    pub vendor_id: String,
    pub product_id: String,
    pub connection_type: ConnectionType,
}

pub fn get_hidraw_devices(context: &libudev::Context) -> Result<Vec<HidrawDevice>, Error> {
    let mut enumerator = libudev::Enumerator::new(&context)?;

    enumerator.match_subsystem("hidraw")?;

    let modalias_regex = Regex::new(r"^.*v(.*)p(.*)$").unwrap();

    let mut devices: Vec<HidrawDevice> = Vec::new();

    for device in enumerator.scan_devices()? {
        if let Some(dev) = device.parent() {
            let modalias = dev.property_value("MODALIAS").unwrap().to_str().unwrap();
            let name = dev.property_value("HID_NAME").unwrap().to_str().unwrap();
            let devpath = dev.property_value("DEVPATH").unwrap().to_str().unwrap();

            let caps = modalias_regex.captures(modalias).unwrap();

            let vid = caps.get(1).unwrap().as_str();
            let pid = caps.get(2).unwrap().as_str();

            let vid_len = vid.len();
            let pid_len = pid.len();

            let clean_vid = &vid[vid_len - 4..vid_len];
            let clean_pid = &pid[pid_len - 4..pid_len];

            devices.push(HidrawDevice {
                name: name.into(),
                vendor_id: clean_vid.into(),
                product_id: clean_pid.into(),
                connection_type: get_connection_type_from_path(&devpath),
            })
        }
    }

    let dualshocks = devices
        .into_iter()
        .filter(|d| is_dualshock_device(d))
        .collect();

    Ok(dualshocks)
}

fn is_dualshock_device(hid_device: &HidrawDevice) -> bool {
    let known_devices = vec![
        // Dualshock 4 V1
        ("054C", "05C4"),
        // Dualshock 4 V2
        ("054C", "09CC"),
    ];

    let dev = known_devices
        .iter()
        .find(|device| device.0 == hid_device.vendor_id && device.1 == hid_device.product_id);

    dev.is_some()
}

fn get_connection_type_from_path(path: &str) -> ConnectionType {
    if path.contains("bluetooth") {
        return ConnectionType::Bluetooth;
    }

    if path.contains("usb") {
        return ConnectionType::Usb;
    }

    ConnectionType::Unknown
}
