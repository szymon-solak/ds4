use libudev::{Error};

mod hid;

fn main() -> Result<(), Error> {
  let devices = hid::get_controllers()?;

  for dev in devices {
    println!(
      "Found device: (Name = {:?}, VID = {:?}, PID = {:?})",
      dev.name,
      dev.vendor_id,
      dev.product_id,
    );
  }

  return Ok(());
}
