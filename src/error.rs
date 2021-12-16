use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid Guacamole Instruction!")]
    InvalidInstruction,
    #[error("Guacamole Protocol Error. {0}")]
    GuacamoleError(String),
}