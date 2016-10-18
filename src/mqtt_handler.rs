extern crate mio;

use super::session::{Session};

use std::io;
use std::io::{ErrorKind};
use std::result::Result;
use std::usize;
use mio::tcp::*;
use mio::{Event, Events, Poll, PollOpt, Ready, Token};
use slab;

type Slab<T> = slab::Slab<T, Token>;

pub const SERVER_TOKEN: Token = mio::Token(usize::MAX-1);

pub struct MqttHandler {
	socket: TcpListener,
	sessions: Slab<Session>
}

impl MqttHandler {
	pub fn new(socket: TcpListener) -> MqttHandler {
		MqttHandler {
			socket: socket,
			sessions: Slab::with_capacity(2)
		}
	}
}

pub enum MqttError {
	Io(io::Error),
	TooManyConnections
}

impl From<io::Error> for MqttError {
	fn from(err: io::Error) -> MqttError {
		MqttError::Io(err)
	}
}

impl MqttHandler {
	fn handle_event(&mut self, poll: &mut Poll, event: Event) -> Result<(), MqttError> {

		let token = event.token();
		let event_type = event.kind();

		match token {
			SERVER_TOKEN => {
				assert!(event_type.is_readable());
				println!("The server is ready to accept a connection!");

				match self.socket.accept() {
					Ok((socket, addr)) => {
						println!("Client addr is {}", addr);

						// Insert client into sessions
						match self.sessions.vacant_entry() {
							Some(entry) => {
								let new_token = entry.index();
								let new_session = Session::new(socket, new_token);

								try!(MqttHandler::register_new_connection(poll, &new_session, new_token));

								entry.insert(new_session).index();
							}
							None => {
								return Err(MqttError::TooManyConnections);
							}
						}

						Ok(())
					}
					Err(e) => {
						println!("{:?}", e);

						match e.kind() {
							ErrorKind::WouldBlock => {
								println!("Socket would block here");
								Ok(())
							},
							_ => Err(MqttError::from(e))
						}
					}
				}
			}
			_ => {
				match self.sessions.get_mut(token) {
					Some(connection) => {
						try!(connection.handle_event(poll, event_type));
					}
					None => println!("Tried to use a token that doesn't exist in the sessions slab: {:?}", token)
				}

				// We use this because we can't call self.sessions.remove inside of the match, and we don't want to use
				// self.sessions[token] because it can cause a panic
				let mut should_remove = false;

				match self.sessions.get(token) {
					Some(connection) => {
						should_remove = connection.is_closed();
					}
					None => println!("Tried to use a token that doesn't exist in the sessions slab: {:?}", token)
				}

				if should_remove {
					println!("Removing {:?} from sessions slab", token);
					self.sessions.remove(token);
				}

				Ok(())
			}
		}
	}

	fn register(&mut self, poll: &mut Poll) -> io::Result<()> {
		poll
		.register(&self.socket, SERVER_TOKEN, Ready::readable(), PollOpt::edge())
		.or_else(|e| {
			println!("Failed to register server {:?}, {:?}", SERVER_TOKEN, e);
			Err(e)
		})
	}

	fn register_new_connection(poll: &mut Poll, new_session: &Session, new_token: Token) -> io::Result<()> {
		let mut interest = Ready::readable();
		interest.insert(Ready::hup());

		// Register the new connection with the event loop
		poll
		.register(&new_session.socket, new_token, interest, PollOpt::edge() | PollOpt::oneshot())
		.or_else(|e| {
			println!("Failed to reregister {:?}, {:?}", new_token, e);
			Err(e)
		})
	}

	pub fn run(&mut self, poll: &mut Poll) -> io::Result<()> {
		let mut events = Events::with_capacity(1024);

		try!(self.register(poll));

		loop {
			try!(poll.poll(&mut events, None)); // None means no timeout

			for event in &events {
				match self.handle_event(poll, event) {
					Ok(_) => (),
					Err(MqttError::Io(e)) => println!("Encountered IO error: {:?}", e),
					Err(MqttError::TooManyConnections) => println!("Too many connections for the server to handle!")
				}
			}

			println!("Tick!");
		}
	}
}
