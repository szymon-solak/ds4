use libudev::{self, Error};

mod devices;

pub(crate) fn get_controllers() -> Result<Vec<devices::HidrawDevice>, Error> {
    let context = libudev::Context::new().expect("Failed to create libudev context");

    let devices = devices::get_hidraw_devices(&context)?;

    Ok(devices)
}
