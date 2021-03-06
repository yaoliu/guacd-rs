use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GlobalError {
    #[error("{0}")]
    InvalidInstruction(String),
    #[error("Guacamole Protocol Error. {0}")]
    GuacamoleError(String),
    #[error("{0}")]
    SocketError(String),
    #[error("{0}")]
    ReceiveError(String),
}