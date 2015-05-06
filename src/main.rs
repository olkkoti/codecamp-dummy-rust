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

struct Connection {
    connection_id: String,
    address: String,
    port: u16
}

fn main() {

    let mut connections: Arc<Mutex<HashMap<String, Connection>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut connections2 = connections.clone();
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:12345").unwrap());
    let socket2 = socket.clone();

    thread::spawn(move || {
        loop {
            thread::sleep_ms(10000);
            let data = connections2.lock().unwrap();
            for (connection_id, connection) in data.iter() {
                println!("{}", connection_id);

                let to = (connection.address.as_ref(), connection.port);
                let ping = Message{_type: "ping".to_string(), connection_id: None, address: None, port: None};
                send(&ping, &socket2, to);
            }
        }
    });

    let mut buf = [0; 1000];
    loop {
        let (read, from) = socket.recv_from(&mut buf).unwrap();
        let x = &buf[..read];
        let y = std::str::from_utf8(x).unwrap();
        println!("{}", y);
        let msg: Result<Message, _> = json::decode(y);
        match msg {
            Ok(msg) => {
                let _type = msg._type.as_ref();
                match _type {
                    "connect" => handleConnect(&msg, &socket, &connections),
                    _ => println!("{}", msg._type)
                }
            },
            Err(e) => println!("{}", e)
        }
    }
}

fn handleConnect(msg: &Message, socket: &UdpSocket, connections: &Mutex<HashMap<String, Connection>>) {
    let connection_id = "1".to_string();
    let connect_ok = Message {
        _type: "connect-ok".to_string(),
        connection_id: Some(connection_id.clone()),
        address: Some("127.0.0.1".to_string()),
        port: Some(12345)};
    let x = msg.address.clone().unwrap();
    let to_addr = x.as_ref();
    let to = (to_addr, msg.port.unwrap());
    send(&connect_ok, socket, to);

    let mut data = connections.lock().unwrap();
    data.insert(connection_id.clone(), Connection {
        connection_id: connection_id.clone(),
        address: x.clone(),
        port: msg.port.unwrap()
        });
}
