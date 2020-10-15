use std::convert::TryFrom;
use nom::{bytes::complete::take, combinator::map, error::ParseError, IResult};
use derive_try_from_primitive::TryFromPrimitive;

pub struct Report {}

impl Report {
  pub fn parse(raw_report: &[u8]) -> Report {    
    let (_, report_type) = ReportId::parse::<()>(&raw_report).unwrap();
    println!("{:?}", report_type);

    Report {}
  }
}

#[derive(TryFromPrimitive, Debug)]
#[repr(u8)]
enum ReportType {
  Usb = 1
}
struct ReportId {
  pub report_type: ReportType,
}

impl std::fmt::Debug for ReportId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReportId ({:?})", self.report_type)
    }
}

impl ReportId {
  pub fn new(report_type: &[u8]) -> ReportId {
    let t = report_type
      .first()
      .expect("Missing report type");

    ReportId {
      report_type: ReportType::try_from(t.to_owned()).unwrap(),
    }
  }

  pub fn parse<'a, E>(i: &'a [u8]) -> IResult<&'a [u8], Self, E>
  where
    E: ParseError<&'a [u8]>,
  {
    map(take(1_usize), Self::new)(i)
  }
}