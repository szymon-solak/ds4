use libudev::{self, Error};
use regex::Regex;

fn main() -> Result<(), Error> {
  let context = libudev::Context::new()
    .expect("Failed to create libudev context");
  
  let devices = get_hidraw_devices(&context)?;

  return Ok(());
}

struct HidrawDevice {
  name: String,
  vendor_id: [u16; 4],
  product_id: [u16; 4],
}

fn get_hidraw_devices(context: &libudev::Context) -> Result<Vec<HidrawDevice>, Error> {
  let mut enumerator = libudev::Enumerator::new(&context)?;

  enumerator.match_subsystem("hidraw")?;

  // ds4 v1 => VID = 0x054C, PID = 0x5C4
  // ds4 v2 => VID = 0x054C, PID = 0x09CC

  let modalias_regex = Regex::new(r"^.*v(.*)p(.*)$").unwrap();

  for device in enumerator.scan_devices()? {
    println!("found device: {:?}", device.syspath());

    match device.parent() {
      Some(dev) => {
        let modalias = dev
          .property_value("MODALIAS")
          .unwrap()
          .to_str()
          .unwrap();

        let caps = modalias_regex
          .captures(modalias)
          .unwrap();

        let vid = caps.get(1).unwrap().as_str();
        let pid = caps.get(2).unwrap().as_str();

        println!("(VID = {:?}, PID = {:?})", vid, pid);
      },
      None => {},
    }
  }

  Ok(vec![])
}
