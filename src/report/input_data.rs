use nom::{bytes::complete::take, combinator::map, sequence::tuple, bits::bits};
use ux::{u1, u4};
use crate::report::parse::{self, BitParsable};

#[derive(Debug)]
pub struct Stick {
  pub x: u8,
  pub y: u8,
}

impl Stick {
  fn new(axis_data: &[u8]) -> Stick {
    match axis_data {
      [x, y] => Stick { x: x.to_owned(), y: y.to_owned() },
      _ => panic!("Bad stick data"),
    }
  }

  pub fn parse(i: parse::Input) -> parse::Result<Self> {
    map(take(2_usize), Self::new)(i)
  }
}

#[derive(Debug)]
pub struct Buttons {
  cross: bool,
  square: bool,
  circle: bool,
  triangle: bool,
  up: bool,
  right: bool,
  down: bool,
  left: bool,
}

impl Buttons {
  pub fn parse(i: parse::Input) -> parse::Result<Self> {
    let (i, (
      triangle,
      circle,
      cross,
      square,
      dpad_mask,
    )) = bits(tuple((
      u1::parse,
      u1::parse,
      u1::parse,
      u1::parse,
      u4::parse,
    )))(i)?;

    let up = [u4::new(0b0111), u4::new(0b0001), u4::new(0b0000)].contains(&dpad_mask);
    let left = [u4::new(0b0111), u4::new(0b0110), u4::new(0b0101)].contains(&dpad_mask);
    let right = [u4::new(0b0011), u4::new(0b0010), u4::new(0b0001)].contains(&dpad_mask);
    let down = [u4::new(0b0101), u4::new(0b0100), u4::new(0b0011)].contains(&dpad_mask);
  
    Ok((i, Buttons {
      cross: cross.into(),
      square: square.into(),
      circle: circle.into(),
      triangle: triangle.into(),
      up,
      right,
      down,
      left,
    }))
  }
}

#[derive(Debug)]
pub struct InputData {
  pub left_stick: Stick,
  pub right_stick: Stick,
  pub buttons: Buttons,
}

impl InputData {
  pub fn parse(i: parse::Input) -> parse::Result<Self> {
    map(
      tuple((Stick::parse, Stick::parse, Buttons::parse)),
      |(left_stick, right_stick, buttons)| Self {
        left_stick,
        right_stick,
        buttons,
      }
    )(i)
  }
}