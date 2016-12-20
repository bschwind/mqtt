use super::session_state::{State};
use super::parser::{MqttConsumer, MqttParser};

use std::io;
use std::io::{ErrorKind, Read};

use mio::tcp::*;
use mio::{Poll, PollOpt, Ready, Token};

// An MQTT Session
pub struct Session {
	pub socket: TcpStream,
	pub token: Token,
	pub state: State,
	pub parser: MqttParser,
	pub mqtt_consumer: MqttConsumer
}

impl Session {
	pub fn new(socket: TcpStream, token: Token) -> Session {
		Session {
			socket: socket,
			token: token,
			state: State::Reading,
			parser: MqttParser::new(),
			mqtt_consumer: MqttConsumer::new()
		}
	}

	pub fn handle_event(&mut self, poll: &mut Poll, event_type: Ready) -> io::Result<()> {
		println!("Session ({:?}) is ready for these events: {:?}", self.token, event_type);

		match self.state {
			State::Reading => {
				assert!(event_type.is_readable());
				try!(self.read(poll));
			}
			State::Writing => {
				assert!(event_type.is_writable());
				self.write(poll);
			}
			State::Closed => {
				println!("Session was ready for reading but is in the Closed state");
			}
		}

		if event_type.is_hup() {
			self.state = State::Closed;
		}

		Ok(())
	}

	fn read(&mut self, poll: &mut Poll) -> io::Result<()> {
		let mut buf = vec![0; 1024];

		match self.socket.read(&mut buf) {
			Ok(0) => {
				println!("Read 0 bytes from socket, shit!");
			},
			Ok(n) => {
				println!("Read {} bytes from socket jdfalfjadfdsa!", n);
				// self.mqtt_consumer.feed_bytes(&buf[0..n]);
				self.parser.feed_bytes(&buf[0..n]);
			},
			Err(e) => {
				match e.kind() {
					ErrorKind::WouldBlock => {
						println!("Socket would block here (session.rs)");
						// self.reregister(poll);
						// try!(self.reregister(poll));
					}
					_ => println!("Error calling write in Session - {}", e)
				}
			}
		}

		try!(self.reregister(poll));

		Ok(())
	}

	fn write(&mut self, _: &mut Poll) {

	}

	fn reregister(&mut self, poll: &mut Poll) -> io::Result<()> {
		let mut interest = Ready::readable();
		interest.insert(Ready::hup());

		poll.register(&self.socket, self.token, interest, PollOpt::edge() | PollOpt::oneshot())
		.or_else(|e| {
			println!("Failed to reregister {:?}, {:?}", self.token, e);
			Err(e)
		})
	}

	pub fn is_closed(&self) -> bool {
		match self.state {
			State::Closed => true,
			_ => false
		}
	}
}
