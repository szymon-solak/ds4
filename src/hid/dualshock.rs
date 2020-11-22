use crate::hid::hidraw_device::HidrawDevice;
use hidapi::{HidApi, HidDevice, HidError};

pub fn is_dualshock_device(hid_device: &HidrawDevice) -> bool {
    let known_devices = vec![
        // Dualshock 4 V1
        (0x054C, 0x05C4),
        // Dualshock 4 V2
        (0x054C, 0x09CC),
    ];

    let dev = known_devices
        .iter()
        .find(|device| device.0 == hid_device.vendor_id && device.1 == hid_device.product_id);

    dev.is_some()
}

pub struct Dualshock {
    pub name: String,
    handle: HidDevice,
}

impl std::fmt::Debug for Dualshock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DS4 ({})", self.name)
    }
}

impl Dualshock {
    pub fn new(hid_device: &HidrawDevice) -> Result<Self, HidError> {
        let api = HidApi::new()?;
        let handle = api.open(hid_device.vendor_id, hid_device.product_id)?;

        Ok(Self {
            name: handle
                .get_product_string()
                .unwrap_or_default()
                .unwrap_or("DS4".into()),
            handle,
        })
    }

    pub fn read_report(&self) -> Result<[u8; 64], HidError> {
        let mut buf = [0u8; 64];
        self.handle.read(&mut buf[..])?;

        return Ok(buf);
    }
}
