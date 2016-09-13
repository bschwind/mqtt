pub struct ConnectData {
	pub protocol_name: String,
	pub protocol_level: u8,
	pub connect_flags: u8,
	pub keep_alive: u16
}

pub struct ConnectAckData {
	pub flags: u8,
	pub return_code: u8
}

pub struct PublishData {
	pub topic_name: String,
	pub packet_id: Option<u16>
}

pub struct PublishAckData {
	pub packet_id: u16
}

pub struct PublishReceivedData {
	pub packet_id: u16
}

pub struct PublishReleaseData {
	pub packet_id: u16
}

pub struct PublishCompleteData {
	pub packet_id: u16
}

pub struct SubscribeData {
	pub packet_id: u16
}

