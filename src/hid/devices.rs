use hidapi::{HidApi, HidError};

pub struct HidrawDevice {
    pub vendor_id: u16,
    pub product_id: u16,
}

impl std::fmt::Debug for HidrawDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(VendorId = {:04x}, ProductId = {:04x})",
            self.vendor_id, self.product_id
        )
    }
}

pub fn get_hidraw_devices() -> Result<Vec<HidrawDevice>, HidError> {
    let api = HidApi::new()?;

    let devices = api
        .device_list()
        .map(|dev| HidrawDevice {
            product_id: dev.product_id(),
            vendor_id: dev.vendor_id(),
        })
        .filter(|dev| is_dualshock_device(dev))
        .collect::<Vec<_>>();

    return Ok(devices);
}

fn is_dualshock_device(hid_device: &HidrawDevice) -> bool {
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
