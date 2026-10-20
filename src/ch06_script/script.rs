use std::{collections::VecDeque, io::Error, ops::Add};

use crate::{
    encode_varint,
    opcodes::{Element, opcode_functions},
};

#[derive(Debug, Clone)]
pub struct Script {
    pub commands: Vec<Cmd>,
}

#[derive(Debug, Clone)]
pub enum Cmd {
    OpCode(u8),
    Data(Vec<u8>),
    // OtherCodes(u8),
}

impl Add for Script {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut commands = self.commands;
        commands.extend_from_slice(&rhs.commands);

        Script { commands }
    }
}

impl Script {
    pub fn new(commands: Vec<Cmd>) -> Self {
        Self { commands }
    }

    pub fn parse(ser: &[u8]) -> Result<Self, Error> {
        let invalid = || {
            Error::new(
                std::io::ErrorKind::InvalidData,
                "Truncated or invalid script",
            )
        };
        let prefix = *ser.first().ok_or_else(invalid)?;
        let (length, prefix_size) = match prefix {
            0xfd => (
                u16::from_le_bytes(ser.get(1..3).ok_or_else(invalid)?.try_into().unwrap()) as u64,
                3usize,
            ),
            0xfe => (
                u32::from_le_bytes(ser.get(1..5).ok_or_else(invalid)?.try_into().unwrap()) as u64,
                5usize,
            ),
            0xff => (
                u64::from_le_bytes(ser.get(1..9).ok_or_else(invalid)?.try_into().unwrap()),
                9usize,
            ),
            n => (n as u64, 1usize),
        };
        let length = usize::try_from(length).map_err(|_| invalid())?;
        let end = prefix_size.checked_add(length).ok_or_else(invalid)?;
        let body = ser.get(prefix_size..end).ok_or_else(invalid)?;
        let mut index = 0usize;
        let mut commands = Vec::new();

        while index < body.len() {
            let opcode = body[index];
            index += 1;
            let data_length = match opcode {
                1..=75 => opcode as usize,
                76 => {
                    let length = *body.get(index).ok_or_else(invalid)? as usize;
                    index += 1;
                    length
                }
                77 => {
                    let bytes = body.get(index..index + 2).ok_or_else(invalid)?;
                    index += 2;
                    u16::from_le_bytes(bytes.try_into().unwrap()) as usize
                }
                _ => {
                    commands.push(Cmd::OpCode(opcode));
                    continue;
                }
            };
            let data_end = index.checked_add(data_length).ok_or_else(invalid)?;
            let data = body.get(index..data_end).ok_or_else(invalid)?;
            commands.push(Cmd::Data(data.to_vec()));
            index = data_end;
        }

        Ok(Script { commands })
    }

    fn raw_serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        for command in &self.commands {
            match command {
                Cmd::OpCode(op) => {
                    result.push(*op);
                }
                Cmd::Data(data) => {
                    let len = data.len();

                    if len <= 75 {
                        result.push(len as u8);
                    } else if len < 0x100 {
                        result.push(76); // OP_PUSHDATA1
                        result.push(len as u8);
                    } else if len <= 520 {
                        result.push(77);
                        result.extend_from_slice(&(len as u16).to_le_bytes());
                    } else {
                        panic!("pushdata too long");
                    }

                    result.extend_from_slice(data);
                }
            }
        }

        result
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut ser = Vec::new();
        let result = self.raw_serialize();
        let total = result.len() as u64;

        ser.extend_from_slice(&encode_varint(total));
        ser.extend_from_slice(&result);

        ser
    }

    pub fn evaluate(&self, z: &[u8]) -> Result<bool, Error> {
        let mut commands: VecDeque<Cmd> = VecDeque::from(self.commands.clone());
        let mut stack: Vec<Element> = Vec::new();
        let mut altstack: Vec<Element> = Vec::new();

        let opcodes = opcode_functions();

        while let Some(cmd) = commands.pop_front() {
            match cmd {
                Cmd::Data(bytes) => {
                    stack.push(Element(bytes));
                }

                Cmd::OpCode(opcode) => {
                    let op_fn = match opcodes.get(&opcode) {
                        Some(f) => *f,
                        None => {
                            return Ok(false);
                        }
                    };

                    let ok = op_fn(&mut stack, &mut altstack, &mut commands, z);

                    if !ok {
                        return Ok(false);
                    }
                }
            }
        }

        if stack.is_empty() {
            return Ok(false);
        }

        let top = stack.pop().unwrap();

        Ok(top.is_truthy())
    }
}
