use std::{io::Error, ops::Add};

use bitcoin::blockdata::opcodes::Opcode;

use crate::{decode_varint, encode_varint};

#[derive(Debug, Clone)]
pub struct Script {
    pub commands: Vec<Cmd>
}

#[derive(Debug, Clone)]
pub enum Cmd {
    OpCode(Vec<u8>),
    Data(Vec<u8>),
    OtherCodes(u8)
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

    pub fn parse(&self, ser: &[u8]) -> Result<Self, Error> {
        let (length, varint_size) = decode_varint(ser, 0);
        let mut commands = Vec::new();
        let mut index: usize = varint_size;

        while index < length as usize {
            // let current = &ser[index];
            
            let current_byte = ser[index];
            index += 1;
            let _ = match current_byte {
                1..=75 => {
                    let n = current_byte as usize;
                    // commands.extend_from_slice(&ser[index..index+n]);
                    commands.push(Cmd::Data((ser[index..index+n]).to_vec()));
                    index += n;
                },
                76 => {
                    // let (data_length, _) = decode_varint(ser, index);
                    let data_length = ser[index] as usize;
                    index += 1;
                    // commands.extend_from_slice(&ser[index..index + data_length as usize]);
                    commands.push(Cmd::OpCode((ser[index..index+data_length]).to_vec()));
                    index += data_length as usize;
                },
                77 => {
                    let data_length = u16::from_le_bytes(ser[index..index+2].try_into().unwrap());
                    index += 2;
                    commands.push(Cmd::OpCode(ser[index..index+data_length as usize].to_vec()));
                },
                op_code => {
                    // let op_code = current_byte;
                    commands.push(Cmd::OtherCodes(op_code));
                }
            };

        }
        if index != length as usize {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Parsing script failed"));
        }

        Ok(Script { commands })
    }

    fn raw_serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        for command in &self.commands {
            if let Cmd::OtherCodes(op_code) = command {
                let op_code_bytes = op_code.to_le_bytes();
                result.extend_from_slice(&op_code_bytes);
            } else if let Cmd::OpCode(op_code) = command {
                let data_length = op_code.len();
                
                if data_length > 75 && data_length < 0x100 {
                    result.push(76_u8);
                    result.extend_from_slice(&(data_length as u8).to_le_bytes());
                } else if data_length >= 0x100 && data_length <= 520 {
                    result.push(77);
                    result.extend_from_slice(&(data_length as u16).to_le_bytes());
                }
            } else if let Cmd::Data(op_code) = command {
                let data_length = op_code.len();
                result.extend_from_slice(&(data_length as u8).to_le_bytes());
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

    pub fn evaluate(&self, z: &[u8]) {
        let mut commands = self.commands.clone();
        let stack: Vec<Vec<u8>> = Vec::new();
        let alt_stack: Vec<Vec<u8>> = Vec::new();

        while commands.len() > 0 {
            let current_command = commands.pop().unwrap();
            
            if let Cmd::OtherCodes(op_code) = current_command {
                let operation = Opcode::from(op_code);

                

                if (99..=100).contains(&op_code) {
                    let res = operation;
                }
            }

            
        }
    }
}