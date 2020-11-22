use std::convert::TryFrom;
use nom::{bytes::complete::take, combinator::map};
use derive_try_from_primitive::TryFromPrimitive;

use crate::report::parse;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum ReportType {
  Usb = 1
}
pub struct ReportId {
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
      report_type: ReportType::try_from(t.to_owned())
        .expect(&format!("Unhandled report type: {}", t)),
    }
  }

  pub fn parse(i: parse::Input) -> parse::Result<Self> {
    map(take(1_usize), Self::new)(i)
  }
}