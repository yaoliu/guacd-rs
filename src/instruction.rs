use std::fmt::format;
use crate::error::Error;

const ELEM_SEP: char = '.';
const INST_TERM: char = ';';
const ARG_SEP: char = ',';

pub struct Instruction {
    pub opcode: String,
    pub args: Vec<String>,
}

impl Instruction {
    pub fn new(opcode: String, args: Vec<String>) -> Self {
        Self {
            opcode,
            args,
        }
    }
    pub fn encode(&self) -> String {
        let instruction = vec![vec![self.opcode.clone()], self.args.clone()].concat();
        let args = instruction
            .iter()
            .map(|x| self.encode_arg(x.to_string()))
            .collect::<Vec<String>>();
        // ARG_SEP = ,
        args.join(ARG_SEP.to_string().as_str())
    }

    pub fn encode_arg(&self, arg: String) -> String {
        vec![arg.len().to_string(), arg].join(ELEM_SEP.to_string().as_str())
    }

    pub fn load(instruction: String) -> Result<Self, Error> {
        if !instruction.ends_with(INST_TERM) {
            return Err(Error::InvalidInstruction("Instruction termination not found.".to_string()));
        }
        match Instruction::decode(instruction) {
            Ok(args) => Ok(Self { opcode: args[0].to_string(), args: vec![args[1..]].concat() }),
            Err(err) => Err(err),
        }
    }


    pub fn decode(instruction: String) -> Result<Vec<String>, Error> {
        let mut args: Vec<String> = vec![];

        if !instruction.ends_with(INST_TERM) {
            return Err(Error::InvalidInstruction("Instruction termination not found.".to_string()));
        }

        let elems: Vec<&str> = instruction.splitn(2, ELEM_SEP).collect();

        let arg_size = elems[0].parse::<i32>().unwrap();

        let arg_str = &elems[1][..arg_size as usize];
        let mut remaining = &elems[1][arg_size as usize..];
        args.push(arg_str.to_string());
        if remaining.starts_with(ARG_SEP) {
            remaining = &remaining[1..];
        } else if remaining.to_string() == INST_TERM.to_string() {
            return Ok(args);
        } else {
            return Err(Error::InvalidInstruction(
                format!("Instruction arg {0} has invalid length.", arg_str)));
        }
        match Instruction::decode(remaining.to_string()) {
            Ok(next_args) => args = [args, next_args].concat(),
            Err(err) => {}
        }
        return Ok(args);
    }
}


impl From<String> for Instruction {
    fn from(s: String) -> Self {
        Instruction::load(s).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::instruction::Instruction;

    #[test]
    fn test_decode() {
        let instruction = String::from("4.size,4.1024;");
        match Instruction::decode(instruction) {
            Ok(result) => { for i in result { println!("result:{}", i) } }
            Err(err) => { println!("1") }
        }
    }

    #[test]
    fn test_encode() {
        let instruction = Instruction::new(String::from("size"), vec![String::from("1024")]);
        let result = instruction.encode();
        print!("{}", result);
    }
}

