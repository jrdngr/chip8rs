use super::opcode::{ OpCode };
use super::graphics::{ Screen };

use std::io;
use std::io::prelude::*;

use wasm_bindgen::prelude::*;

const START_ADDRESS: usize = 512;

#[wasm_bindgen]
pub struct Cpu {
    program_counter: usize,
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    i_register: usize,
    data_registers: [u8; 16],
    stack: [usize; 16],
    ram: [u8; 4096],
    screen: Screen,
}

impl Cpu {
    pub fn load(&mut self, rom: &Vec<u8>) {
        for (offset, byte) in rom.iter().enumerate() {
            self.ram[START_ADDRESS + offset] = *byte;
        }
    }

    fn get_current_opcode(&self) -> OpCode {
        let first_byte = (self.ram[self.program_counter] as u16) << 8;
        let second_byte = self.ram[self.program_counter + 1] as u16;
        OpCode::from(first_byte | second_byte)
    }

    pub fn process_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::ExecuteMachineSubroutine(nnn) => { /* Not implemented */ },
            OpCode::ClearScreen => {   
                self.screen.clear();
            },
            OpCode::ReturnFromSubroutine => {   
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer];
            },
            OpCode::JumpTo(nnn) => {   
                self.program_counter = nnn - 2;
            },
            OpCode::ExecuteSubroutine(nnn) => {   
                self.stack[self.stack_pointer] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = nnn;
            },
            OpCode::SkipIfEqualValue(x, nn) => {   
                let vx = self.data_registers[x as usize];
                if vx == nn {
                    self.program_counter += 2;
                }
            },
            OpCode::SkipIfNotEqualValue(x, nn) => {   
                let vx = self.data_registers[x as usize];
                if vx != nn {
                    self.program_counter += 2;
                }
            },
            OpCode::SkipIfEqualRegister(x, y) => {   
                let vx = self.data_registers[x as usize];
                let vy = self.data_registers[y as usize];
                if vx == vy {
                    self.program_counter += 2;
                }
            },
            OpCode::StoreValue(x, nn) => { 
                self.data_registers[x as usize] = nn; 
            },
            OpCode::AddValue(x, nn) => { 
                self.data_registers[x as usize] += nn; 
            },
            OpCode::StoreRegister(x, y) => { 
                self.data_registers[x as usize] = self.data_registers[y as usize]; 
            },
            OpCode::Or(x, y) => {  
                let vx = self.data_registers[x as usize];
                let vy = self.data_registers[y as usize];
                self.data_registers[x as usize] = vx | vy;
            },
            OpCode::And(x, y) => {
                let vx = self.data_registers[x as usize];
                let vy = self.data_registers[y as usize];
                self.data_registers[x as usize] = vx & vy;
            },
            OpCode::Xor(x, y) => {
                let vx = self.data_registers[x as usize];
                let vy = self.data_registers[y as usize];
                self.data_registers[x as usize] = vx ^ vy;
            },
            OpCode::AddRegister(x, y) => { 
                let vx = self.data_registers[x as usize];
                let vy = self.data_registers[y as usize];
                let (sum, overflowed) = vx.overflowing_add(vy);
                self.data_registers[x as usize] = sum;
                self.data_registers[15] = if overflowed { 1 } else { 0 };
            },
            OpCode::SubtractRegister(x, y) => { 
                let vx = self.data_registers[x as usize];
                let vy = self.data_registers[y as usize];
                let (sum, overflowed) = vx.overflowing_sub(vy);
                self.data_registers[x as usize] = sum;
                self.data_registers[15] = if overflowed { 0 } else { 1 };
            },
            OpCode::ShiftRight(x, y) => {
                let vy = self.data_registers[y as usize];
                self.data_registers[15] = vy & 1;
                self.data_registers[x as usize] = vy >> 1;
            },
            OpCode::SubtractRegisterReverse(x, y) => {
                let vx = self.data_registers[x as usize];
                let vy = self.data_registers[y as usize];
                let (sum, overflowed) = vy.overflowing_sub(vx);
                self.data_registers[x as usize] = sum;
                self.data_registers[15] = if overflowed { 0 } else { 1 };
            },
            OpCode::ShiftLeft(x, y) => {
                let vy = self.data_registers[y as usize];
                self.data_registers[15] = (vy >> 7) & 1;
                self.data_registers[x as usize] = vy << 1;
            },
            OpCode::SkipIfNotEqualRegister(x, y) => {   
                let vx = self.data_registers[x as usize];
                let vy = self.data_registers[y as usize];
                if vx != vy {
                    self.program_counter += 1;
                }
            },
            OpCode::StoreInI(nnn) => { 
                self.i_register = nnn 
            },
            OpCode::JumpWithOffset(nnn) => {   
                let offset = self.data_registers[0] as usize;
                self.program_counter = nnn + offset - 2;
            },
            OpCode::SetToRandom(x, nn) => {   

            },
            OpCode::DrawSprite(x, y, n) => {   
                self.screen.draw_sprite(x, y, n);
            },
            OpCode::SkipIfKeyPressed(x) => {   

            },
            OpCode::SkipIfKeyNotPressed(x) => {   

            },
            OpCode::StoreDelayTimer(x) => { 
                self.data_registers[x as usize] = self.delay_timer; 
            },
            OpCode::WaitAndStoreKey(x) => {   

            },
            OpCode::SetDelayTimer(x) => { 
                self.delay_timer = self.data_registers[x as usize]; 
            },
            OpCode::SetSoundTimer(x) => { 
                self.sound_timer = self.data_registers[x as usize]; 
            },
            OpCode::AddToRegisterI(x) => { 
                self.i_register += self.data_registers[x as usize] as usize; 
            },
            OpCode::SetIToHexSprite(x) => { 

            },
            OpCode::StoreDecimal(x) => {   
                let vx = self.data_registers[x as usize];
                self.ram[self.i_register] = vx / 100;
                self.ram[self.i_register + 1] = (vx / 10) % 10;
                self.ram[self.i_register + 2] = vx % 10;

            },
            OpCode::StoreRegisters(x) => {   
               for address in 0..x {
                   self.ram[self.i_register] = self.data_registers[address as usize];
                   self.i_register += 1;
               } 
            },
            OpCode::FillRegisters(x) => {   
               for register in 0..x {
                   self.data_registers[register as usize] = self.ram[self.i_register];
                   self.i_register += 1;
               } 
            },
        }
    }
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            program_counter: START_ADDRESS,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            i_register: 0,
            data_registers: [0; 16],
            stack: [0; 16],
            ram: [0; 4096],
            screen: Screen::new(),
        }
    }


    pub fn start(&mut self) {
        let max = self.ram.len() - 1;
        while self.program_counter < max {
            let next_op = self.get_current_opcode();
            self.process_opcode(next_op);
            self.program_counter += 2;
        }
    }
    
    pub fn program_counter(&self) -> usize {
        self.program_counter
    }

    pub fn stack_pointer(&self) -> usize {
        self.stack_pointer
    }

    pub fn delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn i_register(&self) -> usize {
        self.i_register as usize
    }

    pub fn data_registers(&self) -> *const u8 {
        self.data_registers.as_ptr()
    }

    pub fn stack(&self) -> *const usize {
        self.stack.as_ptr()
    }

    pub fn ram(&self) -> *const u8 {
        self.ram.as_ptr()
    }
}
