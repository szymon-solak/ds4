use hidapi::HidError;
use report::Report;

mod hid;
mod report;

fn main() -> Result<(), HidError> {
    let devices = hid::get_controllers()?;

    for dev in &devices {
        println!("Found device: {:?}", dev);
    }

    let ds4 = devices
        .first()
        .map(|dev| hid::dualshock::Dualshock::new(dev))
        .expect("No devices found")?;

    loop {
        let report = ds4.read_report()?;
        Report::parse(&report);
    }
}
