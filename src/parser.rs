use std::str;
use std::convert::TryFrom;
use nom::{be_u8, be_u16, Consumer, ConsumerState, ErrorKind, Input, MemProducer, Move, Needed, Producer, IResult};
use nom::Err;
use nom::Err::NodePosition;
use nom::ErrorKind::Custom;

use protocol::{ConnectVariableHeader, ConnectPayload, ControlPacketType, MqttParseError, FixedHeader, FirstByteData, VariableHeader, Payload};

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
					return IResult::Incomplete(Needed::Size(1));
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

named!(pub length_prefixed_utf8_parser<&[u8], &str, MqttParseError>,
	chain!(
		length: fix_error!(MqttParseError, be_u16) ~
		valid_str: add_error!(
			ErrorKind::Custom(MqttParseError::InvalidUTF8Sequence),
			fix_error!(
				MqttParseError,
				map_res!(take!(length), str::from_utf8)
			)
		),
		|| {
			valid_str
		}
	)
);

named!(pub length_prefixed_byte_array<&[u8], &[u8], MqttParseError>,
	chain!(
		length: fix_error!(MqttParseError, be_u16) ~
		valid_str: fix_error!(MqttParseError, take!(length)),
		|| {
			valid_str
		}
	)
);

// Connect Variable Header parser stuff
named!(pub connect_variable_header_parser<&[u8], VariableHeader, MqttParseError>,
	chain!(
		protocol_name: length_prefixed_utf8_parser ~
		protocol_level: fix_error!(MqttParseError, be_u8) ~
		connect_flags: fix_error!(MqttParseError, be_u8) ~
		keep_alive: fix_error!(MqttParseError, be_u16),
		|| {
			VariableHeader::Connect(ConnectVariableHeader {
				protocol_name: protocol_name.into(),
				protocol_level: protocol_level,
				connect_flags: connect_flags,
				keep_alive: keep_alive
			})
		}
	)
);

named!(pub connect_payload_parser<&[u8], Payload, MqttParseError>,
	chain!(
		protocol_name: length_prefixed_utf8_parser ~
		protocol_level: fix_error!(MqttParseError, be_u8) ~
		connect_flags: fix_error!(MqttParseError, be_u8) ~
		keep_alive: fix_error!(MqttParseError, be_u16),
		|| {
			Payload::Connect(ConnectPayload {
				client_id: "hi".into(),
				will_topic: None,
				will_message: None,
				username: None,
				password: None
			})
		}
	)
);

named!(pub connect_packet_parser<&[u8], (VariableHeader, Payload), MqttParseError>,
	chain!(
		variable_header: connect_variable_header_parser ~
		payload: connect_payload_parser,
		|| {
			(variable_header, payload)
		}
	)
);


// Nom Consumer test

enum ParserState {
	ReadingFixedHeader,
	ReadingVariableHeader,
	ReadingPayload,
	Invalid
}

type MqttConsumerState = ConsumerState<FixedHeader, (), Move>;

pub struct MqttConsumer {
	state: ParserState,
	consumer_state: MqttConsumerState,
	fixed_header: Option<FixedHeader>
}

impl MqttConsumer {
	pub fn new() -> MqttConsumer {
		MqttConsumer {
			state: ParserState::ReadingFixedHeader,
			consumer_state: ConsumerState::Continue(Move::Consume(0)),
			fixed_header: None
		}
	}

	pub fn feed_bytes(&mut self, bytes: &[u8]) -> () {
		let mut producer = MemProducer::new(bytes, bytes.len());

		println!("Got {} bytes fed to me!", bytes.len());

		while let &ConsumerState::Continue(_) = producer.apply(self) {
			// Do nothing
		}
	}
}

impl<'a> Consumer<&'a[u8], FixedHeader, (), Move> for MqttConsumer {
	fn state(&self) -> &MqttConsumerState {
		&self.consumer_state
	}

	fn handle(&mut self, input: Input<&'a[u8]>) -> &MqttConsumerState {
		// TODO - update state based on input

		match self.state {
			ParserState::ReadingFixedHeader => {
				println!("In Header state!");

				match input {
					Input::Empty | Input::Eof(None) => {
						self.state = ParserState::Invalid;
						self.consumer_state = ConsumerState::Error(());
					}
					Input::Element(slice) | Input::Eof(Some(slice)) => {
						match fixed_header_parser(slice) {
							IResult::Error(_) => {
								self.state = ParserState::Invalid;
								self.consumer_state = ConsumerState::Error(());
							}
							IResult::Incomplete(n) => {
								self.consumer_state = ConsumerState::Continue(Move::Await(n));
							}
							IResult::Done(_, fixed_header) => {
								self.fixed_header = Some(fixed_header);
								self.state = ParserState::ReadingVariableHeader;
							}
						}
					}
				}
			}
			ParserState::ReadingVariableHeader => {
				println!("Fixed header is {:?}", self.fixed_header);

				if let Some(ref fixed_header) = self.fixed_header {
					match fixed_header.control_type {
						ControlPacketType::Connect => {
							println!("WE GOT A CONNECT PACKET!");

						}
						ControlPacketType::ConnectAck => {

						}
						ControlPacketType::Publish => {

						}
						ControlPacketType::PublishAck => {

						}
						ControlPacketType::PublishReceived => {

						}
						ControlPacketType::PublishRelease => {

						}
						ControlPacketType::PublishComplete => {

						}
						ControlPacketType::Subscribe => {

						}
						ControlPacketType::SubscribeAck => {

						}
						ControlPacketType::Unsubscribe => {

						}
						ControlPacketType::UnsubscribeAck => {

						}
						ControlPacketType::PingRequest => {

						}
						ControlPacketType::PingResponse => {

						}
						ControlPacketType::Disconnect => {

						}
					}
				}
				// println!("");
				// println!("In Payload state!");
			}
			ParserState::ReadingPayload => {
				// println!("Reading Payload!")
			}
			ParserState::Invalid => {
				// println!("In Invalid state!")
			}
		}

		&self.consumer_state
	}
}

// End Nom Consumer test


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
fn test_remaining_length_needs_more() {
	match remaining_length_parser(&[193, 255, 255]) {
		IResult::Done(_, _) => {
			panic!("Expected to need at least one more byte, but it was found to be valid")
		}
		IResult::Incomplete(Needed::Size(n)) => assert_eq!(n, 1),
		e => panic!("{:?}", e)
	}
}

#[test]
fn test_invalid_remaining_length() {
	match remaining_length_parser(&[193, 0xFF, 0xFF, 0xFF, 0xFF]) {
		IResult::Done(_, _) => {
			panic!("Expected remaining_length to be invalid, but it was found to be valid")
		}
		IResult::Error(Err::Code(ErrorKind::Custom(e))) => assert_eq!(e, MqttParseError::InvalidRemainingLength),
		e => panic!("{:?}", e)
	}
}

#[test]
fn test_fixed_header_parser() {
	let test_input = vec!(0x10, 0x1E);
	let output = fixed_header_parser(&test_input);

	match output {
		IResult::Done(i, o) => {
			// assert_eq!(i, &[]);

			assert_eq!(o, FixedHeader {
				control_type: ControlPacketType::Connect,
				bit_0: false,
				bit_1: false,
				bit_2: false,
				bit_3: false,
				remaining_length: 30
			})
		}
		IResult::Incomplete(e) => panic!("Input was incomplete: {:?}", e),
		IResult::Error(Err::Code(ErrorKind::Custom(e))) => assert_eq!(e, MqttParseError::InvalidRemainingLength),
		IResult::Error(e) => panic!("Error: {:?}", e)
	}
}

#[test]
fn test_utf8_parser_valid() {
	// The UTF-8 sequence "A"
	let test_input = vec!(0x00, 0x01, 0x41);

	match length_prefixed_utf8_parser(&test_input) {
		IResult::Done(i, o) => {
			assert_eq!(o, "A");
		}
		e => panic!("{:?}", e)
	}
}

#[test]
fn test_utf8_parser_invalid() {
	// 0xDFFF is an invalid UTF-8 sequence
	let test_input = vec!(0x00, 0x02, 0xDF, 0xFF);

	match length_prefixed_utf8_parser(&test_input) {
		IResult::Done(i, o) => {
			panic!("Expected invalid UTF8 but parser claims it's valid");
		}
		IResult::Error(NodePosition(Custom(e), _, _)) => assert_eq!(e, MqttParseError::InvalidUTF8Sequence),
		e => panic!("{:?}", e)
	}
}

#[test]
fn test_connect_variable_header_parser() {
	let test_input = vec!(
		0x00, 0x04, b'M', b'Q', b'T', b'T', // Protocol Name
		0x04, // Protocol Level
		0x00, // Connect Flags
		0x00, 0x3C // Keep alive time - 60 seconds
	);

	match connect_variable_header_parser(&test_input) {
		IResult::Done(i, o) => {
			assert_eq!(o, VariableHeader::Connect(ConnectVariableHeader {
				protocol_name: "MQTT".into(),
				protocol_level: 4,
				connect_flags: 0,
				keep_alive: 60
			}));
		}
		e => panic!("{:?}", e)
	}
}
