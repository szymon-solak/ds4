use hidapi::HidError;

mod hid;

fn main() -> Result<(), HidError> {
    let devices = hid::get_controllers()?;

    for dev in devices {
        println!("Found device: {:?}", dev);
    }

    Ok(())
}
