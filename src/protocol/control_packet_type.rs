use std::convert::TryFrom;

#[derive(Debug)]
pub enum ControlPacketError {
	InvalidControlType,
	InvalidRemainingLength
}

// 0 and 15 are reserved
#[derive(Debug, Eq, PartialEq)]
pub enum ControlPacketType {
	Connect = 1,
	ConnectAck = 2,
	Publish = 3,
	PublickAck = 4,
	PublishReceived = 5,
	PublishRelease = 6,
	PublishComplete = 7,
	Subscribe = 8,
	SubscribeAck = 9,
	Unsubscribe = 10,
	UnsubscribeAck = 11,
	PingRequest = 12,
	PingResponse = 13,
	Disconnect = 14
}

impl TryFrom<u8> for ControlPacketType {
	type Err = ControlPacketError;

    fn try_from(original: u8) -> Result<ControlPacketType, Self::Err> {
        match original {
            1 => Ok(ControlPacketType::Connect),
            2 => Ok(ControlPacketType::ConnectAck),
            3 => Ok(ControlPacketType::Publish),
            4 => Ok(ControlPacketType::PublickAck),
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
            _ => Err(ControlPacketError::InvalidControlType)
        }
    }	
}

#[test]
fn test_from_u8() {
	match ControlPacketType::try_from(0) {
		Err(ControlPacketError::InvalidControlType) => assert!(true),
		_ => assert!(false)
	}

	assert_eq!(ControlPacketType::try_from(1).unwrap(), ControlPacketType::Connect);
	assert_eq!(ControlPacketType::try_from(2).unwrap(), ControlPacketType::ConnectAck);
	assert_eq!(ControlPacketType::try_from(3).unwrap(), ControlPacketType::Publish);
	assert_eq!(ControlPacketType::try_from(4).unwrap(), ControlPacketType::PublickAck);
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

	for n in 15..255 {
		match ControlPacketType::try_from(n) {
			Err(ControlPacketError::InvalidControlType) => assert!(true),
			_ => assert!(false)
		}
	}
}
