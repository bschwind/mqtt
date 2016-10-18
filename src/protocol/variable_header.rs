use protocol::control_packet_type::ControlPacketType;

#[derive(Debug, PartialEq)]
pub struct ConnectVariableHeader {
	pub protocol_name: String,
	pub protocol_level: u8,
	pub connect_flags: u8,
	pub keep_alive: u16
}

#[derive(Debug, PartialEq)]
pub struct ConnectAckVariableHeader {
	pub flags: u8,
	pub return_code: u8
}

#[derive(Debug, PartialEq)]
pub struct PublishVariableHeader {
	pub topic_name: String,
	pub packet_id: Option<u16>
}

#[derive(Debug, PartialEq)]
pub struct PublishAckVariableHeader {
	pub packet_id: u16
}

#[derive(Debug, PartialEq)]
pub struct PublishReceivedVariableHeader {
	pub packet_id: u16
}

#[derive(Debug, PartialEq)]
pub struct PublishReleaseVariableHeader {
	pub packet_id: u16
}

#[derive(Debug, PartialEq)]
pub struct PublishCompleteVariableHeader {
	pub packet_id: u16
}

#[derive(Debug, PartialEq)]
pub struct SubscribeVariableHeader {
	pub packet_id: u16
}

#[derive(Debug, PartialEq)]
pub enum VariableHeader {
	Connect(ConnectVariableHeader),
	ConnectAck(ConnectAckVariableHeader),
	Publish(PublishVariableHeader),
	PublishAck(PublishAckVariableHeader),
	PublishReceived(PublishReceivedVariableHeader),
	PublishRelease(PublishReleaseVariableHeader),
	PublishComplete(PublishCompleteVariableHeader),
	Subscribe(SubscribeVariableHeader),
	SubscribeAck,
	Unsubscribe,
	UnsubscribeAck,
	PingRequest,
	PingResponse,
	Disconnect
}

impl VariableHeader {
	
}
