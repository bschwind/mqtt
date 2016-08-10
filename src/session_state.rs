use std::io::{Cursor};
use bytes::{Buf, Take};
use std::mem;

pub enum State {
	Reading(Vec<u8>),
	Writing(Take<Cursor<Vec<u8>>>),
	Closed
}

impl State {
	pub fn get_read_buf(&self) -> &[u8] {
		match *self {
			State::Reading(ref buf) => {
				buf
			}
			_ => {
				panic!("Session is not in a reading state");
			}
		}
	}

	pub fn get_mut_read_buf(&mut self) -> &mut Vec<u8> {
		match *self {
			State::Reading(ref mut buf) => {
				buf
			}
			_ => {
				panic!("Session is not in a reading state");
			}
		}
	}

	pub fn get_write_buf(&self) -> &Take<Cursor<Vec<u8>>> {
		match *self {
			State::Writing(ref buf) => {
				buf
			}
			_ => {
				panic!("Session is not in a writing state");
			}
		}
	}

	pub fn get_mut_write_buf(&mut self) -> &mut Take<Cursor<Vec<u8>>> {
		match *self {
			State::Writing(ref mut buf) => {
				buf
			}
			_ => {
				panic!("Session is not in a writing state");
			}
		}
	}

	pub fn try_transition_to_writing(&mut self) {
		if let Some(pos) = self.get_read_buf().iter().position(|&b| b == b'\n') {
			self.transition_to_writing(pos + 1);
		}
	}

	pub fn transition_to_writing(&mut self, pos: usize) {
		// std::mem::replace will put State::Closed into the self location, and return
		// self before it was replaced (returns self in the Reading state)
		let old_read_buf = mem::replace(self, State::Closed).unwrap_read_buf();
		let new_write_buf = Cursor::new(old_read_buf);
		*self = State::Writing(Take::new(new_write_buf, pos));
	}

	pub fn try_transition_to_reading(&mut self) {
		if !self.get_write_buf().has_remaining() {
			let cursor = mem::replace(self, State::Closed).unwrap_write_buf().into_inner();
			let pos = cursor.position();
			let mut buf = cursor.into_inner();

			// Drop all data that has been written to the client
			buf.drain(0..(pos as usize));

			*self = State::Reading(buf);
			self.try_transition_to_writing();
		}
	}

	fn unwrap_read_buf(self) -> Vec<u8> {
		match self {
			State::Reading(buf) => {
				buf
			}
			_ => {
				panic!("Session is not in a reading state");
			}
		}
	}

	fn unwrap_write_buf(self) -> Take<Cursor<Vec<u8>>> {
		match self {
			State::Writing(buf) => {
				buf
			}
			_ => {
				panic!("Session is not in a writing state");
			}
		}
	}
}