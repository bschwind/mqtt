use protocol::control_packet_type::ControlPacketType;

#[derive(Debug, PartialEq)]
pub struct FixedHeader {
	pub control_type: ControlPacketType,
	pub bit_0: bool,
	pub bit_1: bool,
	pub bit_2: bool,
	pub bit_3: bool,
	pub remaining_length: u32
}

#[derive(Clone, Debug, PartialEq)]
pub struct FirstByteData {
	pub control_type: ControlPacketType,
	pub bit_0: bool,
	pub bit_1: bool,
	pub bit_2: bool,
	pub bit_3: bool
}

impl FixedHeader {
	pub fn from_first_byte_data(first_byte: FirstByteData, remaining_length: u32) -> FixedHeader {
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

pub struct ConnectHeader {

}

pub enum VariableHeader {
	Connect(ConnectHeader),
	ConnectAck,
	Publish,

}

#[derive(Debug, PartialEq)]
pub struct Header {
	pub fixed_header: FixedHeader,

}