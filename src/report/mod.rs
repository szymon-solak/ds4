mod parse;
mod report_id;
mod input_data;

use report_id::{ReportId};
use input_data::{InputData};

pub struct Report {}

impl Report {
  pub fn parse(raw_report: &[u8]) -> Report {
    match ReportId::parse(&raw_report) {
      Ok((_input, _report_id)) => {
        let (_, input) = InputData::parse(_input).unwrap();
        println!("{:#?}", input);
      }
      Err(e) => {
        println!("could not parse report id: {:?}", e);
      }
    }

    Report {}
  }
}