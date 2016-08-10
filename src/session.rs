use super::mqtt_handler::{MqttHandler};
use super::session_state::{State};

use mio::{TryRead, TryWrite};
use mio::tcp::*;
use mio::{PollOpt, EventLoop, EventSet, Token};

// An MQTT Session
pub struct Session {
	pub socket: TcpStream,
	pub token: Token,
	pub state: State
}

impl Session {
	pub fn new(socket: TcpStream, token: Token) -> Session {
		Session {
			socket: socket,
			token: token,
			state: State::Reading(vec![])
		}
	}

	pub fn ready(&mut self, event_loop: &mut EventLoop<MqttHandler>, events: EventSet) -> () {
		match self.state {
			State::Reading(_) => {
				assert!(events.is_readable());
				self.read(event_loop);
			}
			State::Writing(_) => {
				assert!(events.is_writable());
				self.write(event_loop);
			}
			State::Closed => {
				println!("Session was ready for reading but is in the Closed state");
			}
		}
		println!("Session is ready for these events: {:?}", events);
	}

	fn read(&mut self, event_loop: &mut EventLoop<MqttHandler>) {
		match self.socket.try_read_buf(self.state.get_mut_read_buf()) {
			Ok(Some(0)) => {
				// TODO - understand this match
				println!("Read 0 bytes from client; buffered={}", self.state.get_read_buf().len());
				// unimplemented!();

				match self.state.get_read_buf().len() {
					n if n > 0 => {
						self.state.transition_to_writing(n);
						self.reregister(event_loop);
					}
					_ => {
						self.state = State::Closed;
					}
				}

			}
			Ok(Some(n)) => {
				println!("Read {} bytes", n);
				println!("{:?}", self.state.get_read_buf());
				self.state.try_transition_to_writing();
				self.reregister(event_loop);
			}
			Ok(None) => {
				// Socket would block?
				self.reregister(event_loop);
			}
			Err(e) => {
				println!("Error calling read in Session - {}", e);
				self.state = State::Closed;
			}
		}
	}

	fn write(&mut self, event_loop: &mut EventLoop<MqttHandler>) {
		match self.socket.try_write_buf(self.state.get_mut_write_buf()) {
			Ok(Some(n)) => {
				println!("Wrote {} bytes", n);
				self.state.try_transition_to_reading();
				self.reregister(event_loop);
			}
			Ok(None) => {
				// Socket would block?
				self.reregister(event_loop);
			}
			Err(e) => {
				println!("Error calling write in Session - {}", e);
			}
		}
	}

	fn reregister(&mut self, event_loop: &mut EventLoop<MqttHandler>) {
		let event_set = match self.state {
			State::Reading(_) => {
				EventSet::readable()
			}
			State::Writing(_) => {
				EventSet::writable()
			}
			_ => {
				EventSet::none()
			}
		};

		match event_loop.reregister(&self.socket, self.token, event_set, PollOpt::edge() | PollOpt::oneshot()) {
			Ok(_) => {}
			Err(e) => { println!("Error reregistering connection in event loop for event_set {:?} - {}", event_set, e) }
		}
	}

	pub fn is_closed(&self) -> bool {
		match self.state {
			State::Closed => true,
			_ => false
		}
	}
}

// pub enum State {
// 	Reading(Vec<u8>),
// 	Writing(Take<Cursor<Vec<u8>>>),
// 	Closed
// }

// impl State {
// 	fn get_read_buf(&self) -> &[u8] {
// 		match *self {
// 			State::Reading(ref buf) => {
// 				buf
// 			}
// 			_ => {
// 				panic!("Session is not in a reading state");
// 			}
// 		}
// 	}

// 	fn get_mut_read_buf(&mut self) -> &mut Vec<u8> {
// 		match *self {
// 			State::Reading(ref mut buf) => {
// 				buf
// 			}
// 			_ => {
// 				panic!("Session is not in a reading state");
// 			}
// 		}
// 	}

// 	fn get_write_buf(&self) -> &Take<Cursor<Vec<u8>>> {
// 		match *self {
// 			State::Writing(ref buf) => {
// 				buf
// 			}
// 			_ => {
// 				panic!("Session is not in a writing state");
// 			}
// 		}
// 	}

// 	fn get_mut_write_buf(&mut self) -> &mut Take<Cursor<Vec<u8>>> {
// 		match *self {
// 			State::Writing(ref mut buf) => {
// 				buf
// 			}
// 			_ => {
// 				panic!("Session is not in a writing state");
// 			}
// 		}
// 	}

// 	fn try_transition_to_writing(&mut self) {
// 		if let Some(pos) = self.get_read_buf().iter().position(|&b| b == b'\n') {
// 			self.transition_to_writing(pos + 1);
// 		}
// 	}

// 	fn transition_to_writing(&mut self, pos: usize) {
// 		// std::mem::replace will put State::Closed into the self location, and return
// 		// self before it was replaced (returns self in the Reading state)
// 		let old_read_buf = mem::replace(self, State::Closed).unwrap_read_buf();
// 		let new_write_buf = Cursor::new(old_read_buf);
// 		*self = State::Writing(Take::new(new_write_buf, pos));
// 	}

// 	fn try_transition_to_reading(&mut self) {
// 		if !self.get_write_buf().has_remaining() {
// 			let cursor = mem::replace(self, State::Closed).unwrap_write_buf().into_inner();
// 			let pos = cursor.position();
// 			let mut buf = cursor.into_inner();

// 			// Drop all data that has been written to the client
// 			buf.drain(0..(pos as usize));

// 			*self = State::Reading(buf);
// 			self.try_transition_to_writing();
// 		}
// 	}

// 	fn unwrap_read_buf(self) -> Vec<u8> {
// 		match self {
// 			State::Reading(buf) => {
// 				buf
// 			}
// 			_ => {
// 				panic!("Session is not in a reading state");
// 			}
// 		}
// 	}

// 	fn unwrap_write_buf(self) -> Take<Cursor<Vec<u8>>> {
// 		match self {
// 			State::Writing(buf) => {
// 				buf
// 			}
// 			_ => {
// 				panic!("Session is not in a writing state");
// 			}
// 		}
// 	}
// }