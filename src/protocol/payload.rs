use protocol::control_packet_type::ControlPacketType;

#[derive(Debug, PartialEq)]
pub struct ConnectPayload {
	pub client_id: String,
	pub will_topic: Option<String>,
	pub will_message: Option<Vec<u8>>,
	pub username: Option<String>,
	pub password: Option<Vec<u8>>
}

#[derive(Debug, PartialEq)]
pub enum Payload {
	Connect(ConnectPayload),
	ConnectAck,
	Publish,
	PublishAck,
	PublishReceived,
	PublishRelease,
	PublishComplete,
	Subscribe,
	SubscribeAck,
	Unsubscribe,
	UnsubscribeAck,
	PingRequest,
	PingResponse,
	Disconnect
}

impl Payload {
	
}
