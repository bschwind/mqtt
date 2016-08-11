#![feature(try_from)]

#[macro_use]
extern crate nom;

extern crate mio;
extern crate bytes;

mod mqtt_handler;
mod parser;
mod session;
mod session_state;
mod protocol;

use mio::tcp::*;
use mio::{PollOpt, EventLoop, EventSet};

use mqtt_handler::MqttHandler;

fn main() {
	let port: u16 = 1883;
	let address = format!("0.0.0.0:{}", port).parse().unwrap();
	let server = TcpListener::bind(&address).unwrap();

	let mut event_loop = EventLoop::new().unwrap();
	event_loop
		.register(&server, mqtt_handler::SERVER_TOKEN, EventSet::readable(), PollOpt::level())
		.unwrap_or_else(|e| println!("Error registering TcpListener in event loop - {}", e));

	println!("Running MQTT server on port {}", port);

	event_loop
		.run(&mut MqttHandler::new(server))
		.unwrap_or_else(|e| println!("Error running the event loop - {}", e));
}
