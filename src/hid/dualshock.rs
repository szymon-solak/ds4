use crate::hid::hidraw_device::HidrawDevice;

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
