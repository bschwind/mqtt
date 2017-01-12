pub use self::control_packet_type::*;
pub use self::fixed_header::*;
pub use self::variable_header::*;
pub use self::payload::*;

pub mod control_packet_type;
pub mod fixed_header;
pub mod variable_header;
pub mod payload;

#[derive(Debug, PartialEq)]
pub enum MqttParseError {
	InvalidControlType,
	InvalidRemainingLength,
	InvalidUTF8Sequence
}

#[derive(Debug)]
pub enum MQTTQoS {
	AtMostOnce,
	AtLeastOnce,
	ExactlyOne
}

#[derive(Debug)]
pub struct MQTTPacket {
	pub control_type: ControlPacketType,
	pub qos: Option<MQTTQoS>,
	pub remaining_length: u32,
	pub variable_header: Option<VariableHeader>,
	pub payload: Vec<u8>
}
