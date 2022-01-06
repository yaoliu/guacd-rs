/*// use std::borrow::BorrowMut;
// use std::cell::{Ref, RefCell, RefMut};
// use std::io::{BufRead, BufReader, BufWriter, Read, Write};
// use std::net::{Shutdown, SocketAddr, TcpStream};
// use std::time::Duration;
// use log::{debug, info};
// use std::str::FromStr;
// use crate::GlobalError;
// use crate::GlobalError::InvalidInstruction;
// use crate::Instruction;
//
//
// pub enum Protocol {
//     RDP,
//     SSH,
//     VNC,
// }
//
// impl From<String> for Protocol {
//     fn from(_: String) -> Self {
//         todo!()
//     }
// }
//
// impl FromStr for Protocol {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "RDP" => Ok(Protocol::RDP),
//             "SSH" => Ok(Protocol::SSH),
//             "VNC" => Ok(Protocol::VNC),
//             _ => Err(()),
//         }
//     }
// }
//
//
// pub struct GuacamoleClient {
//     _host: String,
//     _port: u16,
//     _timeout: Duration,
//     stream: Option<TcpStream>,
//     id: Option<String>,
//     connected: bool,
// }
//
// impl GuacamoleClient {
//     pub fn new(host: String, port: u16, timeout: Duration) -> Result<Self, GlobalError> {
//         let socket = SocketAddr::new(host.parse().unwrap(), port);
//         match TcpStream::connect_timeout(&socket, timeout) {
//             Ok(stream) => {
//                 info!("Connected to the Guacamole server!");
//                 Ok(Self {
//                     _host: host,
//                     _port: port,
//                     _timeout: timeout,
//                     stream: Some(stream),
//                     connected: false,
//                     id: None,
//                 })
//             }
//             Err(err) => {
//                 Err(GlobalError::SocketError(format!("Couldn't connect to Guacamole server...{}", err)))
//             }
//         }
//     }
//
//     pub fn id(&self) -> Option<String> {
//         self.id.clone()
//     }
//
//     pub fn close(&mut self) {
//         self.stream.as_ref().unwrap().shutdown(Shutdown::Both).unwrap();
//         self.connected = false;
//     }
//
//     pub fn handshake(&mut self, protocol: String, width: String, height: String, dpi: String,
//                      audio: Option<Vec<String>>, video: Option<Vec<String>>, image: Option<Vec<String>>) {
//         debug!("Send `select` instruction.");
//         self.send_instruction(Instruction::new("select".to_string(), vec![protocol]));
//         let instruction = self.read_instruction();
//
//         let mut connection_args: Vec<String> = vec![];
//
//         debug!("Send `size` instruction ({},{},{})",width,height,dpi);
//         self.send_instruction(Instruction::new("size".to_string(), vec![width, height, dpi]));
//
//         debug!("Send `audio` instruction ({:?})",audio);
//         self.send_instruction(Instruction::new("audio".to_string(), audio.unwrap_or_default()));
//
//         debug!("Send `video` instruction ({:?})",video);
//         self.send_instruction(Instruction::new("video".to_string(), video.unwrap_or_default()));
//
//         debug!("Send `image` instruction ({:?})",image);
//         self.send_instruction(Instruction::new("image".to_string(), image.unwrap_or_default()));
//
//         debug!("Send `timezone` instruction Asia/Shanghai");
//         self.send_instruction(Instruction::new("timezone".to_string(), vec!["Asia/Shanghai".to_string()]));
//
//         debug!("Send `connect` instruction ({:?})",connection_args);
//         self.send_instruction(Instruction::new("connect".to_string(), connection_args));
//
//         let instruction = self.read_instruction().unwrap();
//         debug!("Send `connect` instruction ({:?})",instruction.encode());
//
//         if instruction.opcode != "ready" {
//             debug!("Expected `ready` instruction, received: %s instead");
//         }
//         if !instruction.args.is_empty() {
//             self.id = Option::from(instruction.args[0].to_string());
//             self.connected = true;
            debug!("Established connection with client id: {:?}'",self.id());
//         } else {else
//             debug!("No connection with client id");
//         }
//         debug!("Handshake completed.");
//     }
//
//     pub fn send_instruction(&mut self, instruction: Instruction) {
//         debug!("Sending instruction:{}",instruction);
//         self.send(instruction.encode());
//     }
//
//     pub fn send(&mut self, data: String) {
//         self.stream.as_ref().unwrap().write_all(data.as_bytes());
//         self.stream.as_ref().unwrap().flush();
//     }
//
//     pub fn read_instruction(&mut self) -> Result<Instruction, GlobalError> {
//         match String::from(self.receive()).parse::<Instruction>() {
//             Ok(i) => Ok(i),
//             Err(e) => Err(GlobalError::ReceiveError("staring instruction failed".to_string()))
//         }
//     }
//
//     pub fn receive(&mut self) -> Vec<u8> {
//         let mut received: Vec<u8> = vec![];
//         let mut reader = BufReader::new(&mut self.stream.unwrap());
//         reader.read_until(b';', &mut received);
//         received
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use std::time::Duration;
//     use crate::client::{GuacamoleClient};
//     use crate::instruction::Instruction;
//
//     #[test]
//     fn client_conn() {
//         let mut client = GuacamoleClient::new("192.168.200.50".to_string(), 4822, Duration::new(1, 1)).unwrap();
//         client.send_instruction(Instruction::new("select".to_string(), vec!["rdp".to_string()]));
//         let instruction = client.read_instruction().unwrap();
//         println!("{}", instruction);
//         client.send_instruction(Instruction::new("size".to_string(), vec!["1024".to_string(), "768".to_string(), "96".to_string()]));
//     }
//
//     #[test]
//     fn handshake() {
//         let mut client = GuacamoleClient::new("192.168.200.50".to_string(), 4822, Duration::new(5, 0)).unwrap();
//         client.handshake(
//             "rdp".to_string(),
//             "1024".to_string(),
//             "768".to_string(),
//             "96".to_string(),
//             Option::from(vec!["".to_string()]),
//             None,
//             None,
//         )
//     }
// }
*/