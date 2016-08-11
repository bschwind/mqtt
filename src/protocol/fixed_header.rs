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
