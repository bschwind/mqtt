use std::convert::TryFrom;
use super::MqttParseError;

// 0 and 15 are reserved
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ControlPacketType {
	Connect,
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

impl TryFrom<u8> for ControlPacketType {
	type Err = MqttParseError;

	fn try_from(original: u8) -> Result<ControlPacketType, MqttParseError> {
		match original {
			1 => Ok(ControlPacketType::Connect),
			2 => Ok(ControlPacketType::ConnectAck),
			3 => Ok(ControlPacketType::Publish),
			4 => Ok(ControlPacketType::PublishAck),
			5 => Ok(ControlPacketType::PublishReceived),
			6 => Ok(ControlPacketType::PublishRelease),
			7 => Ok(ControlPacketType::PublishComplete),
			8 => Ok(ControlPacketType::Subscribe),
			9 => Ok(ControlPacketType::SubscribeAck),
			10 => Ok(ControlPacketType::Unsubscribe),
			11 => Ok(ControlPacketType::UnsubscribeAck),
			12 => Ok(ControlPacketType::PingRequest),
			13 => Ok(ControlPacketType::PingResponse),
			14 => Ok(ControlPacketType::Disconnect),
			_ => Err(MqttParseError::InvalidControlType)
		}
	}
}

#[test]
fn test_from_u8() {
	match ControlPacketType::try_from(0) {
		Err(MqttParseError::InvalidControlType) => assert!(true),
		_ => assert!(false)
	}

	assert_eq!(ControlPacketType::try_from(1).unwrap(), ControlPacketType::Connect);
	assert_eq!(ControlPacketType::try_from(2).unwrap(), ControlPacketType::ConnectAck);
	assert_eq!(ControlPacketType::try_from(3).unwrap(), ControlPacketType::Publish);
	assert_eq!(ControlPacketType::try_from(4).unwrap(), ControlPacketType::PublishAck);
	assert_eq!(ControlPacketType::try_from(5).unwrap(), ControlPacketType::PublishReceived);
	assert_eq!(ControlPacketType::try_from(6).unwrap(), ControlPacketType::PublishRelease);
	assert_eq!(ControlPacketType::try_from(7).unwrap(), ControlPacketType::PublishComplete);
	assert_eq!(ControlPacketType::try_from(8).unwrap(), ControlPacketType::Subscribe);
	assert_eq!(ControlPacketType::try_from(9).unwrap(), ControlPacketType::SubscribeAck);
	assert_eq!(ControlPacketType::try_from(10).unwrap(), ControlPacketType::Unsubscribe);
	assert_eq!(ControlPacketType::try_from(11).unwrap(), ControlPacketType::UnsubscribeAck);
	assert_eq!(ControlPacketType::try_from(12).unwrap(), ControlPacketType::PingRequest);
	assert_eq!(ControlPacketType::try_from(13).unwrap(), ControlPacketType::PingResponse);
	assert_eq!(ControlPacketType::try_from(14).unwrap(), ControlPacketType::Disconnect);

	for n in 15..256 {
		match ControlPacketType::try_from(n) {
			Err(MqttParseError::InvalidControlType) => assert!(true),
			_ => assert!(false)
		}
	}
}
