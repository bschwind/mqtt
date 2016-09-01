use std::convert::TryFrom;
use nom::{ErrorKind, Needed, IResult};
use nom::Err;

use protocol::{ControlPacketType, MqttParseError, FixedHeader};

#[derive(Clone, Debug, PartialEq)]
struct FirstByteData {
	control_type: ControlPacketType,
	bit_0: bool,
	bit_1: bool,
	bit_2: bool,
	bit_3: bool
}

impl FixedHeader {
	fn from_first_byte_data(first_byte: FirstByteData, remaining_length: u32) -> FixedHeader {
		FixedHeader {
			control_type: first_byte.control_type,
			bit_0: first_byte.bit_0,
			bit_1: first_byte.bit_1,
			bit_2: first_byte.bit_2,
			bit_3: first_byte.bit_3,
			remaining_length: remaining_length
		}
	}
}

fn first_byte_parser(input: &[u8]) -> IResult<&[u8], FirstByteData, MqttParseError> {
	if input.len() < 1 {
		IResult::Incomplete(Needed::Size(1))
	} else {
		let first_byte = input[0];

		match ControlPacketType::try_from(((first_byte & 0b11110000) >> 4)) {
			Ok(control_type) => {
				IResult::Done(&input[1..], FirstByteData {
					control_type: control_type,
					bit_0: first_byte & 0b00001000 == 0b00001000,
					bit_1: first_byte & 0b00000100 == 0b00000100,
					bit_2: first_byte & 0b00000010 == 0b00000010,
					bit_3: first_byte & 0b00000001 == 0b00000001
				})
			}
			Err(e) => {
				IResult::Error(Err::Code(ErrorKind::Custom(e)))
			}
		}
	}
}

fn remaining_length_parser(input: &[u8]) -> IResult<&[u8], u32, MqttParseError> {
	if input.len() < 1 {
		IResult::Incomplete(Needed::Size(1))
	} else {
		let mut multiplier = 1;
		let mut value: u32 = 0;
		let mut iterator = input.iter();
		let mut consumed_bytes: usize = 0;

		loop {
			let encoded_byte;
			match iterator.next() {
				Some(n) => {
					encoded_byte = n;
				}
				None => {
					return IResult::Error(Err::Code(ErrorKind::Custom(MqttParseError::InvalidRemainingLength)));
				}
			}

			consumed_bytes += 1;

			value += ((encoded_byte & 0b01111111) as u32) * multiplier;
			multiplier *= 128;

			if multiplier > (128 * 128 * 128) {
				return IResult::Error(Err::Code(ErrorKind::Custom(MqttParseError::InvalidRemainingLength)));
			}

			if encoded_byte & 0b10000000 == 0b00000000 {
				break;
			}
		}

		IResult::Done(&input[consumed_bytes..], value)
	}
}

named!(pub fixed_header_parser<&[u8], FixedHeader, MqttParseError>,
	chain!(
		first_byte_data: first_byte_parser ~
		remaining_length: remaining_length_parser,
		|| {
			FixedHeader::from_first_byte_data(first_byte_data, remaining_length)
		}
	)
);

#[test]
fn test_first_byte_parser() {
	match first_byte_parser(&[16, 2, 143, 121, 110]) {
		IResult::Done(i, o) => {
			assert_eq!(i, &[2, 143, 121, 110]);
			assert_eq!(o, FirstByteData {
				control_type: ControlPacketType::Connect,
				bit_0: false,
				bit_1: false,
				bit_2: false,
				bit_3: false
			})
		}
		_ => panic!()
	}
}

#[test]
fn test_first_byte_parser_invalid_data() {
	match first_byte_parser(&[0]) {
		IResult::Done(_, _) => {
			panic!("Expected first_byte to be invalid, but it was found to be valid")
		}
		IResult::Error(Err::Code(ErrorKind::Custom(e))) => assert_eq!(e, MqttParseError::InvalidControlType),
		_ => panic!()
	}
}

#[test]
fn test_length() {
	match remaining_length_parser(&[193, 2, 143, 121, 110]) {
		IResult::Done(i, o) => {
			assert_eq!(i, &[143, 121, 110]);
			assert_eq!(o, 321)
		}
		_ => panic!()
	}
}

#[test]
fn test_invalid_length() {
	match remaining_length_parser(&[193, 255, 255]) {
		IResult::Done(_, _) => {
			panic!("Expected remaining_length to be invalid, but it was found to be valid")
		}
		IResult::Error(Err::Code(ErrorKind::Custom(e))) => assert_eq!(e, MqttParseError::InvalidRemainingLength),
		e => panic!("{:?}", e)
	}
}

#[test]
fn test_header_parser() {
	let test_input = &[16, 30];
	let output = fixed_header_parser(test_input);

	match output {
		IResult::Done(i, o) => {
			assert_eq!(i, &[]);

			assert_eq!(o, FixedHeader {
				control_type: ControlPacketType::Connect,
				bit_0: false,
				bit_1: false,
				bit_2: false,
				bit_3: false,
				remaining_length: 30
			})
		}
		_ => panic!()
	}
}
