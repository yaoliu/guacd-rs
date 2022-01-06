mod instruction;
mod error;
mod client;

pub use error::GlobalError;
pub use instruction::Instruction;
pub use client::GuacamoleClient;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
