#![feature(try_from)]

#[macro_use]
extern crate nom;

extern crate mio;
extern crate bytes;
extern crate slab;

mod mqtt_handler;
mod parser;
mod session;
mod session_state;
mod protocol;

use mio::tcp::*;
use mio::{Poll};

use mqtt_handler::MqttHandler;

fn main() {
	let port: u16 = 1883;
	let address = format!("0.0.0.0:{}", port).parse().unwrap();
	let socket = TcpListener::bind(&address).unwrap();

	let mut poll = Poll::new().expect("Failed to create Poll");
	let mut server = MqttHandler::new(socket);

	println!("Running MQTT server on port {}", port);
	server.run(&mut poll).expect("Failed to run the server");
}
