extern crate websocket;
use std::str::from_utf8;
use std::thread;
use websocket::message::Type;
use websocket::{Server, Message, Sender, Receiver};

fn main() {
    let server = Server::bind("0.0.0.0:1234").unwrap();

    for connection in server {
        thread::spawn(move || {
            let mut client =
                connection.unwrap()
                .read_request().unwrap()
                .accept()
                .send().unwrap();

            let ip = client.get_mut_sender()
                .get_mut()
                .peer_addr()
                .unwrap();

            println!("Connection from {}", ip);

            let (mut sender, mut receiver) = client.split();

            for message in receiver.incoming_messages() {
                let message: Message = message.unwrap();

                match message.opcode {
                    Type::Close => {
                        let message = Message::close();
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    },
                    Type::Ping => {
                        let message = Message::pong(message.payload);
                        sender.send_message(&message).unwrap();
                        println!("Client {} pinged", ip);
                    },
                    Type::Text => {
                        let content = from_utf8(&*message.payload).unwrap();
                        println!("Client {} sent a text message: {}", ip, content);
                        if content == r#"{"type":"init"}"# {
                            let cafe_corner = format!(r#"{{"buildingId":0, "name":"Cafe Corner", "lights":[{{"lightId":0, "isOn":false}}, {{"lightId":1, "isOn":false}}]}}"#);
                            let green_grocer = format!(r#"{{"buildingId":1, "name":"Green Grocer", "lights":[{{"lightId":0, "isOn":false}}, {{"lightId":1, "isOn":false}}]}}"#);
                            let response = format!(r#"{{"type":"initialize", "buildings":[{}, {}]}}"#, cafe_corner, green_grocer);
                            println!("Sending response: {:?}", response);
                            let message = Message::text(response);
                            sender.send_message(&message).unwrap();
                        }
                    },
                    _ => {
                        sender.send_message(&message).unwrap();
                        println!("Client {} sent a message: {:?}", ip, message);
                    }
                }
            }
        });
    }
}
