use std::convert::TryFrom;
use nom::{Needed, IResult};

use protocol::{ControlPacketType, ControlPacketError, FixedHeader};

pub fn remaining_length_parser(input: &[u8]) -> IResult<&[u8], Result<u32, ControlPacketError>> {
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
					return IResult::Incomplete(Needed::Size(1));
				}
			}

			consumed_bytes += 1;

			value += ((encoded_byte & 0b01111111) as u32) * multiplier;
			multiplier *= 128;

			if multiplier > (128 * 128 * 128) {
				// return IResult::Error(Err());
				return IResult::Done(&input[consumed_bytes..], Err(ControlPacketError::InvalidRemainingLength));
			}

			if encoded_byte & 0b10000000 == 0b00000000 {
				break;
			}
		}

		IResult::Done(&input[consumed_bytes..], Ok(value))
	}
}

named!(pub fixed_header_parser(&[u8]) -> Result<FixedHeader, ControlPacketError>,
	chain!(
		first_byte: take!(1) ~
		remaining_length: remaining_length_parser,
		|| {
			let first_byte = first_byte[0];

			ControlPacketType::try_from(((first_byte & 0b11110000) >> 4))
			.map(|control_type| {
				FixedHeader {
					control_type: control_type,
					bit_0: first_byte & 0b00001000 == 0b00001000,
					bit_1: first_byte & 0b00000100 == 0b00000100,
					bit_2: first_byte & 0b00000010 == 0b00000010,
					bit_3: first_byte & 0b00000001 == 0b00000001,
					remaining_length: remaining_length.unwrap_or(0)
				}
			})
		}
	)
);

#[test]
fn test_length() {
	match remaining_length_parser(&[193, 2, 143, 121, 110]) {
		IResult::Done(i, o) => {
			assert_eq!(i, &[143, 121, 110]);
			assert_eq!(o.unwrap(), 321)
		}
		_ => panic!()
	}
}

#[test]
fn test_header_parser() {
	let test_input = &[16, 30];
	let output = fixed_header_parser(test_input);

	match output {
		IResult::Done(i, o) => {
			assert_eq!(i, &[]);

			assert_eq!(o.unwrap(), FixedHeader {
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
