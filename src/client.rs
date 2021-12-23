use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;
use log::{debug, info};
use crate::GlobalError;
use crate::GlobalError::InvalidInstruction;
use crate::Instruction;

pub struct GuacamoleClient {
    _host: String,
    _port: u16,
    _timeout: Duration,
    stream: TcpStream,
    id: Option<String>,
    buffer: BufReader<TcpStream>,
    connected: bool,
}

#[derive(Clone)]
struct Buffer {
    remaining: usize,
    buf: Vec<u8>,
}


impl GuacamoleClient {
    pub fn new(host: String, port: u16, timeout: Duration) -> Result<Self, GlobalError> {
        env_logger::init();
        let socket = SocketAddr::new(host.parse().unwrap(), port);
        match TcpStream::connect_timeout(&socket, timeout) {
            Ok(stream) => {
                let reader = BufReader::new(stream.try_clone().unwrap());
                info!("Connected to the Guacamole server!");
                Ok(Self {
                    _host: host,
                    _port: port,
                    _timeout: timeout,
                    stream,
                    connected: false,
                    id: None,
                    buffer: reader,
                })
            }
            Err(err) => {
                Err(GlobalError::SocketError(format!("Couldn't connect to Guacamole server...{}", err)))
            }
        }
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn close(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
        self.connected = false;
    }

    pub fn handshake(&mut self, protocol: String, width: String, height: String, dpi: String,
                     audio: Option<Vec<String>>, video: Option<Vec<String>>, image: Option<Vec<String>>) {
        debug!("Send `select` instruction.");
        self.send_instruction(Instruction::new("select".to_string(), vec![protocol]));
        let instruction = self.read_instruction();

        let mut connection_args: Vec<String> = vec![];

        debug!("Send `size` instruction ({},{},{})",width,height,dpi);
        self.send_instruction(Instruction::new("size".to_string(), vec![width, height, dpi]));

        debug!("Send `audio` instruction ({:?})",audio);
        self.send_instruction(Instruction::new("audio".to_string(), audio.unwrap_or_default()));

        debug!("Send `video` instruction ({:?})",video);
        self.send_instruction(Instruction::new("video".to_string(), video.unwrap_or_default()));

        debug!("Send `image` instruction ({:?})",image);
        self.send_instruction(Instruction::new("image".to_string(), image.unwrap_or_default()));

        debug!("Send `timezone` instruction Asia/Shanghai");
        self.send_instruction(Instruction::new("timezone".to_string(), vec!["Asia/Shanghai".to_string()]));

        debug!("Send `connect` instruction ({:?})",connection_args);
        // self.send_instruction(Instruction::new("connect".to_string(), connection_args));

        let instruction = self.read_instruction().unwrap();
        debug!("Send `connect` instruction ({:?})",instruction.encode());

        if instruction.opcode != "ready" {
            debug!("Expected `ready` instruction, received: %s instead");
        }
        if !instruction.args.is_empty() {
            self.id = Option::from(instruction.args[0].to_string());
            self.connected = true;
            debug!("Established connection with client id: {:?}'",self.id());
        } else {
            debug!("No connection with client id");
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
            Err(_) => { Err(GlobalError::InvalidInstruction("".to_string())) }
        };
    }

    pub fn receive(&mut self) -> Vec<u8> {
        let mut received: Vec<u8> = vec![];
        let mut reader = &mut self.buffer;
        reader.read_until(b';', &mut received);
        received
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::client::{GuacamoleClient};
    use crate::instruction::Instruction;

    #[test]
    fn client_conn() {
        let mut client = GuacamoleClient::new("192.168.200.50".to_string(), 4822, Duration::new(1, 1)).unwrap();
        client.send_instruction(Instruction::new("select".to_string(), vec!["rdp".to_string()]));
        let instruction = client.read_instruction().unwrap();
        println!("{}", instruction);
        client.send_instruction(Instruction::new("size".to_string(), vec!["1024".to_string(), "768".to_string(), "96".to_string()]));
    }

    #[test]
    fn handshake() {
        let mut client = GuacamoleClient::new("192.168.200.50".to_string(), 4822, Duration::new(5, 0)).unwrap();
        client.handshake(
            "rdp".to_string(),
            "1024".to_string(),
            "768".to_string(),
            "96".to_string(),
            Option::from(vec!["".to_string()]),
            None,
            None,
        )
    }
}
