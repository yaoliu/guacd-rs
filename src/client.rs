use std::fmt::Error;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::rc::Rc;
use std::time::Duration;
use std::io::{Read, Write};
use log::{debug, info, error};
use crate::GlobalError;
use crate::instruction::Instruction;

#[derive(Clone, Copy)]
pub enum Protocols {
    VNC,
    RDP,
    SSH,
}

impl From<String> for Protocols {
    fn from(s: String) -> Self {
        match s {
            Some("RDP") => Self::RDP,
            Some("VNC") => Self::VNC,
            Some("SSH") => Self::SSH,
            None => ""
        }
    }
}


pub struct GuacamoleClient {
    host: String,
    port: u16,
    timeout: Duration,
    stream: TcpStream,
    id: String,
    buffer: Buffer,
}

#[derive(Clone)]
struct Buffer {
    remaining: usize,
    buf: Vec<u8>,
}


impl GuacamoleClient {
    pub fn new(host: String, port: u16, timeout: Duration) -> Result<Self, GlobalError> {
        let socket = SocketAddr::new(host.parse().unwrap(), port);
        match TcpStream::connect_timeout(&socket, timeout) {
            Ok(stream) => {
                info!("Connected to the Guacamole server!");
                Ok(Self {
                    host,
                    port,
                    timeout,
                    stream,
                    id: "".to_string(),
                    buffer: Buffer { remaining: 1, buf: Vec::new() },
                })
            }
            Err(err) => {
                Err(GlobalError::SocketConnect(format!("Couldn't connect to Guacamole server...{}", err)))
            }
        }
    }
    pub fn close(self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
    pub fn handshake(&mut self, protocol: String, width: String, height: String, dpi: String,
                     audio: Option<Vec<String>>, video: Option<Vec<String>>, image: Option<Vec<String>>) {
        debug!("Send `select` instruction.");

        self.send_instruction(Instruction::new("select".to_string(), vec![protocol]));

        let instruction = self.read_instruction();
        match instruction {
            Ok(i) => {
                if i.opcode != "args" {
                    self.close();
                    // return;
                }
            }
            Err(err) => {
                self.close();
                // return;
            }
        }
        debug!("Send `size` instruction ({},{},{})",width,height,dpi);
        self.send_instruction(Instruction::new("size".to_string(), vec![width, height, dpi]));

        debug!("Send `audio` instruction ({:?})",audio);
        self.send_instruction(Instruction::new("audio".to_string(), audio.unwrap()));

        debug!("Send `video` instruction ({:?})",video);
        self.send_instruction(Instruction::new("video".to_string(), video.unwrap()));

        debug!("Send `image` instruction ({:?})",video);
        self.send_instruction(Instruction::new("image".to_string(), image.unwrap()));

        let connection_args = instruction.unwrap().args;

        debug!("Send `connect` instruction ({:?})",connection_args);
        self.send_instruction(Instruction::new("connect".to_string(), connection_args));

        let instruction = self.read_instruction().unwrap();
        debug!("Send `connect` instruction ({:?})",instruction.encode());

        if instruction.opcode != "ready" {
            debug!("Expected `ready` instruction, received: %s instead");
        }
        if instruction.args {
            self.id = instruction.args[0].to_string();
            debug!("Established connection with client id: {}'",self.id)
        }
        debug!("Handshake completed.");
    }

    pub fn send_instruction(&mut self, ins: Instruction) {
        debug!("Sending instruction:{}",ins);
        self.send(ins.encode());
    }

    pub fn send(&mut self, data: String) {
        self.stream.write_all(data.as_bytes());
        self.stream.flush();
    }

    pub fn read_instruction(&mut self) -> Result<Instruction, GlobalError> {
        return match String::from_utf8(self.receive()) {
            Ok(result) => Instruction::load(result),
            Err(e) => { Err(GlobalError::InvalidInstruction("".to_string())) }
        };
    }

    pub fn receive(&mut self) -> Vec<u8> {
        let mut received: Vec<u8> = vec![];
        // Array with a fixed size
        let mut rx_bytes = [0u8; 5];
        loop {
            // Read from the current data in the TcpStream
            let bytes_read = self.stream.read(&mut rx_bytes).unwrap();

            // However many bytes we read, extend the `received` string bytes
            received.extend_from_slice(&rx_bytes[..bytes_read]);

            // If we didn't fill the array
            // stop reading because there's no more data (we hope!)
            if bytes_read < 5 {
                break;
            }
        }
        received
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::client::{GuacamoleClient, Protocols};
    use crate::instruction::Instruction;

    #[test]
    fn client_conn() {
        let mut client = GuacamoleClient::new("192.168.200.50".to_string(), 4822, Duration::new(1, 1)).unwrap();
        client.send_instruction(Instruction::new("select".to_string(), vec!["rdp".to_string()]));
        let instruction = client.read_instruction().unwrap();
        println!("{}", instruction);
        //client.send_instruction(Instruction::new("size".to_string(), vec!["1024".to_string(), "768".to_string(), "96".to_string()]));
    }

    #[test]
    fn enum_protocol() {
        let p = "RDP".parse::<Protocols>();
        println!("{:?}", p);
    }
}
