use std::fmt::{Formatter, Display};
use std::fmt;
use std::str::FromStr;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::GlobalError;


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
            .into_par_iter()
            .map(|x| self.encode_arg(x.to_string()))
            .collect::<Vec<String>>();
        let elems = args.join(ARG_SEP.to_string().as_str());
        format!("{}{}", elems, INST_TERM)
    }

    pub fn encode_arg(&self, arg: String) -> String {
        vec![arg.len().to_string(), arg].join(ELEM_SEP.to_string().as_str())
    }

    pub fn load(instruction: String) -> Result<Self, GlobalError> {
        if instruction.is_empty() {
            return Err(GlobalError::InvalidInstruction("String is empty.".to_string()));
        }
        if !instruction.ends_with(INST_TERM) {
            return Err(GlobalError::InvalidInstruction("Instruction termination not found.".to_string()));
        }
        match Instruction::decode(instruction) {
            Ok(args) => Ok(Self { opcode: args[0].to_string(), args: args[1..].to_owned() }),
            Err(err) => Err(err),
        }
    }


    pub fn decode(instruction: String) -> Result<Vec<String>, GlobalError> {
        let mut args: Vec<String> = vec![];

        if !instruction.ends_with(INST_TERM) {
            return Err(GlobalError::InvalidInstruction("Instruction termination not found.".to_string()));
        }

        let elems: Vec<&str> = instruction.splitn(2, ELEM_SEP).collect();

        let mut arg_size;
        match elems[0].parse::<usize>() {
            Ok(s) => arg_size = s,
            Err(_err) => return Err(GlobalError::InvalidInstruction("Invalid arg length. Possibly due to missing element separator".to_string())),
        };
        let arg_str = &elems[1][..arg_size];

        let mut remaining = &elems[1][arg_size..];

        args.push(arg_str.to_string());
        if remaining.starts_with(ARG_SEP) {
            remaining = &remaining[1..];
        } else if remaining.to_string() == INST_TERM.to_string() {
            return Ok(args);
        } else {
            return Err(GlobalError::InvalidInstruction(
                format!("Instruction arg {0} has invalid length.", arg_str)));
        }
        match Instruction::decode(remaining.to_string()) {
            Ok(next_args) => args = [args, next_args].concat(),
            Err(_err) => {}
        }
        return Ok(args);
    }
}


impl From<String> for Instruction {
    fn from(s: String) -> Self {
        Instruction::load(s).unwrap()
    }
}

impl FromStr for Instruction {
    type Err = GlobalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Instruction::load(s.to_string()) {
            Ok(i) => Ok(i),
            Err(e) => Err(e),
        }
    }
}


impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.opcode, self.args)
    }
}


#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    #[test]
    fn test_decode() {
        let instruction = String::from("4.size,4.1024;");
        match Instruction::decode(instruction) {
            Ok(result) => { for i in result { println!("result:{}", i) } }
            Err(_) => {}
        }
    }

    #[test]
    fn test_encode() {
        let instruction = Instruction::new(String::from("size"), vec![String::from("1024")]);
        let result = instruction.encode();
        assert_eq!(result, "4.size,4.1024;")
    }

    #[test]
    fn test_encode_select() {
        let instruction = Instruction::new(String::from("select"), vec![String::from("rdp")]);
        let result = instruction.encode();
        assert_eq!(result, "6.select,3.rdp;")
    }

    #[test]
    fn print_instruction() {
        let instruction = Instruction::new(String::from("size"), vec![String::from("1024")]);
        println!("{}", instruction);
    }
}

