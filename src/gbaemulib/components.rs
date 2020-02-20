use crate::gbaemulib::instructions::{Instruction, TargetRegister};
use crate::gbaemulib::registers::Registers;

const VIDEO_RAM_SIZE: usize = 0x9FFF - 0x8000;
const NUMBER_OF_PIXELS: usize = 23040;

//todo: find out values for constants
struct GPU {
    tile_set: [u8; 360],
    video_ram: [u8; VIDEO_RAM_SIZE],
    canvas_buffer: [u8; NUMBER_OF_PIXELS],
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
    gpu: GPU,
}

struct CPU {
    registers: Registers,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    TargetRegister::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => { /*TODO: support more targets*/ }
                }
            }
            _ => { /*TODO: support more instructions*/ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        unimplemented!();
    }
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        if address >= VIDEO_RAM_BEGIN ||
            address < VIDEO_RAM_END {
            self.gpu.read_byte(address)
        } else {
            self.memory[address as usize]git
        }
    }
}