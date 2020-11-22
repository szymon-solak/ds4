use ux::{u4, u1};
use nom::{combinator::map, bits::complete::take as take_bits};

pub type Input<'a> = &'a [u8];
pub type Result<'a, T> = nom::IResult<Input<'a>, T, Error<Input<'a>>>;

pub type BitInput<'a> = (&'a [u8], usize);
pub type BitResult<'a, T> = nom::IResult<BitInput<'a>, T, Error<BitInput<'a>>>;

#[derive(Debug)]
pub struct Error<I> {
  pub errors: Vec<(I, nom::error::ErrorKind)>,
}

impl<I> nom::error::ParseError<I> for Error<I> {
  fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
      let errors = vec![(input, kind)];
      Self { errors }
  }

  fn append(input: I, kind: nom::error::ErrorKind, mut other: Self) -> Self {
      other.errors.push((input, kind));
      other
  }
}

use nom::{ErrorConvert, Slice};
use std::ops::RangeFrom;

impl<I> ErrorConvert<Error<I>> for Error<(I, usize)>
where
    I: Slice<RangeFrom<usize>>,
{
    fn convert(self) -> Error<I> {
        let errors = self
            .errors
            .into_iter()
            .map(|((rest, offset), err)| (rest.slice(offset / 8..), err))
            .collect();
        Error { errors }
    }
}

pub trait BitParsable
where
  Self: Sized,
{
  fn parse(i: BitInput) -> BitResult<Self>;
}

impl BitParsable for u4 {
  fn parse(i: BitInput) -> BitResult<Self> {
      map(take_bits(4_usize), Self::new)(i)
  }
}

impl BitParsable for u1 {
  fn parse(i: BitInput) -> BitResult<Self> {
      map(take_bits(1_usize), Self::new)(i)
  }
}