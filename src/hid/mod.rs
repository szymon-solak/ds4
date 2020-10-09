mod devices;
mod dualshock;
mod hidraw_device;

use hidapi::HidError;

pub fn get_controllers() -> Result<Vec<hidraw_device::HidrawDevice>, HidError> {
    let devices = devices::get_hidraw_devices()?
        .into_iter()
        .filter(|device| dualshock::is_dualshock_device(device))
        .collect::<_>();

    Ok(devices)
}
