use crate::hid::hidraw_device::HidrawDevice;
use hidapi::{HidApi, HidError};

pub fn get_hidraw_devices() -> Result<Vec<HidrawDevice>, HidError> {
    let api = HidApi::new()?;

    let devices = api
        .device_list()
        .map(|dev| HidrawDevice {
            product_id: dev.product_id(),
            vendor_id: dev.vendor_id(),
        })
        .collect::<Vec<_>>();

    return Ok(devices);
}
