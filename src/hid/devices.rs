use libudev::{self, Error};
use regex::Regex;

pub(crate) struct HidrawDevice {
  pub name: String,
  pub vendor_id: String,
  pub product_id: String,
}

pub(crate) fn get_hidraw_devices(context: &libudev::Context) -> Result<Vec<HidrawDevice>, Error> {
  let mut enumerator = libudev::Enumerator::new(&context)?;

  enumerator.match_subsystem("hidraw")?;

  // ds4 v1 => VID = 0x054C, PID = 0x5C4
  // ds4 v2 => VID = 0x054C, PID = 0x09CC

  let modalias_regex = Regex::new(r"^.*v(.*)p(.*)$").unwrap();

  let mut devices: Vec<HidrawDevice> = Vec::new();

  for device in enumerator.scan_devices()? {
    match device.parent() {
      Some(dev) => {
        let modalias = dev
          .property_value("MODALIAS")
          .unwrap()
          .to_str()
          .unwrap();

        let name = dev
          .property_value("HID_NAME")
          .unwrap()
          .to_str()
          .unwrap();

        let caps = modalias_regex
          .captures(modalias)
          .unwrap();

        let vid = caps.get(1).unwrap().as_str();
        let pid = caps.get(2).unwrap().as_str();

        devices.push(HidrawDevice {
          name: name.into(),
          vendor_id: vid.into(),
          product_id: pid.into(),
        })
      },
      None => {},
    }
  }

  Ok(devices)
}