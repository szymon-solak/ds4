use hidapi::HidError;

mod devices;

pub(crate) fn get_controllers() -> Result<Vec<devices::HidrawDevice>, HidError> {
    let devices = devices::get_hidraw_devices()?;

    Ok(devices)
}
