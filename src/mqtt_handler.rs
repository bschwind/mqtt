extern crate mio;

use super::session::{Session};

use mio::tcp::*;
use mio::{PollOpt, EventLoop, EventSet, Token};
use mio::util::Slab;

pub const SERVER_TOKEN: Token = mio::Token(0);

pub struct MqttHandler {
	server: TcpListener,
	sessions: Slab<Session>
}

impl MqttHandler {
	pub fn new(server: TcpListener) -> MqttHandler {
		MqttHandler {
			server: server,
			sessions: Slab::new_starting_at(Token(1), 4)
		}
	}
}

impl mio::Handler for MqttHandler {
	type Timeout = ();
	type Message = ();

	fn ready(&mut self, event_loop: &mut EventLoop<MqttHandler>, token: mio::Token, events: mio::EventSet) {

		match token {
			SERVER_TOKEN => {
				assert!(events.is_readable());
				println!("The server is ready to accept a connection!");

				match self.server.accept() {
					Ok(Some((socket, addr))) => {
						println!("Client addr is {}", addr);
						match self.sessions.insert_with(|token| Session::new(socket, token)) {
							Some(client_token) => {
								let new_connection: &Session = &self.sessions[client_token];
								println!("Accepted a socket, token is {:?}", client_token);

								match event_loop.register(&new_connection.socket, client_token, EventSet::readable(), PollOpt::level() | PollOpt::oneshot()) {
									Ok(_) => {}
									Err(e) => { println!("Error registering client TcpListener in event loop - {}", e) }
								}
							}
							None => {
								// Disconnect the client here?
								println!("I can't take any more!");
							}
						}
					}
					Ok(None) => {
						println!("Socket would block here");
					}
					Err(e) => {
						println!("Oh no the socket errored! - {}", e);
						event_loop.shutdown();
					}
				}
			}
			_ => {
				match self.sessions.get_mut(token) {
					Some(connection) => {
						connection.ready(event_loop, events);
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
			}
		}
	}

	fn tick(&mut self, _: &mut EventLoop<MqttHandler>) {
		println!("TICK!");
	}
}

// TODO - Implement tests
#[test]
fn lol() -> () {
	assert_eq!(1 + 1, 2);
}
