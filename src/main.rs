use hidapi::HidError;

mod hid;

fn main() -> Result<(), HidError> {
    let devices = hid::get_controllers()?;

    for dev in &devices {
        println!("Found device: {:?}", dev);
    }

    let ds4 = devices
        .first()
        .map(|dev| hid::dualshock::Dualshock::new(dev));

    println!("{:?}", ds4);

    Ok(())
}
