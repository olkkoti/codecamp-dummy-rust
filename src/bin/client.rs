use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::str;
use rustc_serialize::json;
use std::collections::HashMap;
use codecamp_dummy_rust::Message;
use codecamp_dummy_rust::send;
use codecamp_dummy_rust::read_message;
extern crate rustc_serialize;
extern crate codecamp_dummy_rust;

fn main() {

    let address = "127.0.0.1";
    let port = 12346;
    let server_address = ("127.0.0.1", 12345);

    let socket = UdpSocket::bind((address, port)).unwrap();

    let connect = Message{
        _type: "connect".to_string(),
        connection_id: None,
        address: Some(address.to_string()),
        port: Some(port)};

    send(&connect, &socket, server_address);

    let connect_ok = read_message(&socket);
    let connection_id = connect_ok.connection_id;

    loop {
        let ping = read_message(&socket);

        let pong = Message{
            _type: "pong".to_string(),
            connection_id: connection_id.clone(),
            address: None,
            port: None
        };
        send(&pong, &socket, server_address);
    }
}
