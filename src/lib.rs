use std::net::UdpSocket;
use std::net::ToSocketAddrs;
use rustc_serialize::json;
extern crate rustc_serialize;


pub fn send<A: ToSocketAddrs>(msg: &Message, socket: &UdpSocket, to: A) {
    let msg_str = json::encode(msg).unwrap();
    socket.send_to(&msg_str.into_bytes(), to);
}

pub fn read_message(socket: &UdpSocket) -> Message {
    let mut buf = [0; 1000];
    let (read, from) = socket.recv_from(&mut buf).unwrap();
    let x = &buf[..read];
    let y = std::str::from_utf8(x).unwrap();
    println!("{}", y);
    let msg: Result<Message, _> = json::decode(y);
    return msg.unwrap()
}

pub struct Message {
    pub _type: String,
    pub connection_id: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>
}

impl ::rustc_serialize::Decodable for Message {
    fn decode<__D: ::rustc_serialize::Decoder>(__arg_0: &mut __D)
     -> ::std::result::Result<Message, __D::Error> {
        __arg_0.read_struct("Message", 4usize, |_d| -> _ {
                            ::std::result::Result::Ok(Message{_type:
                                                                  match _d.read_struct_field("type",
                                                                                             0usize,
                                                                                             ::rustc_serialize::Decodable::decode)
                                                                      {
                                                                      ::std::result::Result::Ok(__try_var)
                                                                      =>
                                                                      __try_var,
                                                                      ::std::result::Result::Err(__try_var)
                                                                      =>
                                                                      return ::std::result::Result::Err(__try_var),
                                                                  },
                                                              connection_id:
                                                                  match _d.read_struct_field("connection-id",
                                                                                             1usize,
                                                                                             ::rustc_serialize::Decodable::decode)
                                                                      {
                                                                      ::std::result::Result::Ok(__try_var)
                                                                      =>
                                                                      __try_var,
                                                                      ::std::result::Result::Err(__try_var)
                                                                      =>
                                                                      return ::std::result::Result::Err(__try_var),
                                                                  },
                                                              address:
                                                                  match _d.read_struct_field("address",
                                                                                             2usize,
                                                                                             ::rustc_serialize::Decodable::decode)
                                                                      {
                                                                      ::std::result::Result::Ok(__try_var)
                                                                      =>
                                                                      __try_var,
                                                                      ::std::result::Result::Err(__try_var)
                                                                      =>
                                                                      return ::std::result::Result::Err(__try_var),
                                                                  },
                                                              port:
                                                                  match _d.read_struct_field("port",
                                                                                             3usize,
                                                                                             ::rustc_serialize::Decodable::decode)
                                                                      {
                                                                      ::std::result::Result::Ok(__try_var)
                                                                      =>
                                                                      __try_var,
                                                                      ::std::result::Result::Err(__try_var)
                                                                      =>
                                                                      return ::std::result::Result::Err(__try_var),
                                                                  },}) })
    }
}

impl ::rustc_serialize::Encodable for Message {
    fn encode<__S: ::rustc_serialize::Encoder>(&self, __arg_0: &mut __S)
     -> ::std::result::Result<(), __S::Error> {
        match *self {
            Message {
            _type: ref __self_0_0,
            connection_id: ref __self_0_1,
            address: ref __self_0_2,
            port: ref __self_0_3 } =>
            __arg_0.emit_struct("Message", 4usize, |_e| -> _ {
                                match _e.emit_struct_field("type", 0usize,
                                                           |_e| -> _ {
                                                           (*__self_0_0).encode(_e)
                                                       }) {
                                    ::std::result::Result::Ok(__try_var) =>
                                    __try_var,
                                    ::std::result::Result::Err(__try_var) =>
                                    return ::std::result::Result::Err(__try_var),
                                };
                                match _e.emit_struct_field("connection_id",
                                                           1usize, |_e| -> _ {
                                                           (*__self_0_1).encode(_e)
                                                       }) {
                                    ::std::result::Result::Ok(__try_var) =>
                                    __try_var,
                                    ::std::result::Result::Err(__try_var) =>
                                    return ::std::result::Result::Err(__try_var),
                                };
                                match _e.emit_struct_field("address", 2usize,
                                                           |_e| -> _ {
                                                           (*__self_0_2).encode(_e)
                                                       }) {
                                    ::std::result::Result::Ok(__try_var) =>
                                    __try_var,
                                    ::std::result::Result::Err(__try_var) =>
                                    return ::std::result::Result::Err(__try_var),
                                };
                                return _e.emit_struct_field("port", 3usize,
                                                            |_e| -> _ {
                                                            (*__self_0_3).encode(_e)
                                                        }); }),
        }
    }
}
