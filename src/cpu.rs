pub mod opcode;
pub mod rng;

use wasm_bindgen::prelude::*;

use self::opcode::{ OpCode };
use self::rng::{ Rng };

use crate::javascript;

const START_ADDRESS: usize = 512;
const INSTRUCTIONS_PER_SECOND: u32 = 500;
const FRAME_DURATION_MS: u32 = 500;  //1000 / INSTRUCTIONS_PER_SECOND;

macro_rules! log {
    ($($t:tt)*) => {
        (javascript::log(&format!($($t)*)))
    };
}

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
    rng: Rng,
    is_paused: bool,
    last_frame: u32,
}

impl Cpu {
    pub fn process_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::ExecuteMachineSubroutine(nnn) => { /* Not implemented */ },
            OpCode::ClearScreen => {   
                javascript::clearScreen();
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
                let vx = self.data_registers[x];
                if vx == nn {
                    self.program_counter += 2;
                }
            },
            OpCode::SkipIfNotEqualValue(x, nn) => {   
                let vx = self.data_registers[x];
                if vx != nn {
                    self.program_counter += 2;
                }
            },
            OpCode::SkipIfEqualRegister(x, y) => {   
                let vx = self.data_registers[x];
                let vy = self.data_registers[y];
                if vx == vy {
                    self.program_counter += 2;
                }
            },
            OpCode::StoreValue(x, nn) => { 
                self.data_registers[x] = nn; 
            },
            OpCode::AddValue(x, nn) => { 
                let vx = self.data_registers[x];
                let (sum, _overflowed) = vx.overflowing_add(nn);
                self.data_registers[x] = sum;
            },
            OpCode::StoreRegister(x, y) => { 
                self.data_registers[x] = self.data_registers[y]; 
            },
            OpCode::Or(x, y) => {  
                let vx = self.data_registers[x];
                let vy = self.data_registers[y];
                self.data_registers[x] = vx | vy;
            },
            OpCode::And(x, y) => {
                let vx = self.data_registers[x];
                let vy = self.data_registers[y];
                self.data_registers[x] = vx & vy;
            },
            OpCode::Xor(x, y) => {
                let vx = self.data_registers[x];
                let vy = self.data_registers[y];
                self.data_registers[x] = vx ^ vy;
            },
            OpCode::AddRegister(x, y) => { 
                let vx = self.data_registers[x];
                let vy = self.data_registers[y];
                let (sum, overflowed) = vx.overflowing_add(vy);
                self.data_registers[x] = sum;
                self.data_registers[15] = if overflowed { 1 } else { 0 };
            },
            OpCode::SubtractRegister(x, y) => { 
                let vx = self.data_registers[x];
                let vy = self.data_registers[y];
                let (sum, overflowed) = vx.overflowing_sub(vy);
                self.data_registers[x] = sum;
                self.data_registers[15] = if overflowed { 0 } else { 1 };
            },
            OpCode::ShiftRight(x, y) => {
                let vy = self.data_registers[y];
                self.data_registers[15] = vy & 1;
                self.data_registers[x] = vy >> 1;
            },
            OpCode::SubtractRegisterReverse(x, y) => {
                let vx = self.data_registers[x];
                let vy = self.data_registers[y];
                let (sum, overflowed) = vy.overflowing_sub(vx);
                self.data_registers[x] = sum;
                self.data_registers[15] = if overflowed { 0 } else { 1 };
            },
            OpCode::ShiftLeft(x, y) => {
                let vy = self.data_registers[y];
                self.data_registers[15] = (vy >> 7) & 1;
                self.data_registers[x] = vy << 1;
            },
            OpCode::SkipIfNotEqualRegister(x, y) => {   
                let vx = self.data_registers[x];
                let vy = self.data_registers[y];
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
                let random = self.rng.random_u8() & nn;
                self.data_registers[x] = random;
            },
            OpCode::DrawSprite(x, y, n) => {   
                let vx = self.data_registers[x];
                let vy = self.data_registers[y];
                for y_offset in 0..n {
                    let byte = self.ram[self.i_register + y_offset as usize];
                    let bits = Cpu::get_bits(byte);
                    for x_offset in 0..bits.len() as u8 {
                        if bits[x_offset as usize] {
                            javascript::setPixel(vx + x_offset, vy + y_offset);
                        }
                    }
                }
            },
            OpCode::SkipIfKeyPressed(x) => { 
                let key = self.data_registers[x]; 
                if javascript::isKeyDown(key as i32) {
                    self.program_counter += 2;
                }
            },
            OpCode::SkipIfKeyNotPressed(x) => { 
                let key = self.data_registers[x];  
                if !javascript::isKeyDown(key as i32) {
                    self.program_counter += 2;
                }
            },
            OpCode::StoreDelayTimer(x) => { 
                self.data_registers[x] = self.delay_timer; 
            },
            OpCode::WaitAndStoreKey(x) => { 
                let key = javascript::getAnyKey();
                if key > 0 {
                    self.data_registers[x] = key as u8;
                } else {
                    self.program_counter -= 2;
                }
            },
            OpCode::SetDelayTimer(x) => { 
                self.delay_timer = self.data_registers[x]; 
            },
            OpCode::SetSoundTimer(x) => { 
                self.sound_timer = self.data_registers[x]; 
            },
            OpCode::AddToRegisterI(x) => { 
                self.i_register += self.data_registers[x] as usize; 
            },
            OpCode::SetIToHexSprite(x) => { 
                let value = self.data_registers[x];
                self.i_register = 5 * value as usize;
            },
            OpCode::StoreDecimal(x) => {   
                let vx = self.data_registers[x];
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
                   self.data_registers[register] = self.ram[self.i_register];
                   self.i_register += 1;
               } 
            },
        }
    }

    pub fn load(&mut self, rom: &[u8]) {
        for (offset, byte) in rom.iter().enumerate() {
            self.ram[START_ADDRESS + offset] = *byte;
        }
        self.reset_program_counter();
        self.is_paused = false;
    }

    fn next_op(&mut self) {
        let next_op = self.get_current_opcode();
        self.process_opcode(next_op);
        self.program_counter += 2;
        if self.delay_timer > 0 { self.delay_timer -= 1 };
        if self.sound_timer > 0 { self.sound_timer -= 1 };
    }
    
    fn get_current_opcode(&self) -> OpCode {
        let first_byte = (self.ram[self.program_counter] as u16) << 8;
        let second_byte = self.ram[self.program_counter + 1] as u16;
        OpCode::from(first_byte | second_byte)
    }

    fn reset_program_counter(&mut self) {
        self.program_counter = START_ADDRESS;
    }

    fn get_bits(byte: u8) -> [bool; 8] {
        [
            (byte & 0b1000_0000) >> 7 == 1,
            (byte & 0b0100_0000) >> 6 == 1,
            (byte & 0b0010_0000) >> 5 == 1,
            (byte & 0b0001_0000) >> 4 == 1,
            (byte & 0b0000_1000) >> 3 == 1,
            (byte & 0b0000_0100) >> 2 == 1,
            (byte & 0b0000_0010) >> 1 == 1,
             byte & 0b0000_0001 == 1,
        ]
    }
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Self {
        let mut ram = [0; 4096];
        for (index, byte) in HEX_SPRITES.iter().enumerate() {
            ram[index] = *byte;
        }

        Cpu {
            program_counter: START_ADDRESS,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            i_register: 0,
            data_registers: [0; 16],
            stack: [0; 16],
            rng: Rng::new(javascript::getRandomSeed()),
            is_paused: true,
            ram,
            last_frame: javascript::now(),
        }
    }

    pub fn load_from_web(&mut self, rom: &[u8]) {
        self.load(&rom);
    }

    pub fn start(&mut self) {
        let max = self.ram.len() - 1;
        while self.program_counter < max {
            let now = javascript::now();
            if (now - self.last_frame > FRAME_DURATION_MS) {
                self.next_op();
                self.last_frame = now; 
            }
        }
    }

    pub fn step(&mut self) {
        let next_op = self.get_current_opcode();
        log!("{:?}", next_op);
        self.next_op();
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


const HEX_SPRITES: [u8; 5 * 16] = [
        0b1111_0000,
        0b1001_0000,
        0b1001_0000,
        0b1001_0000,
        0b1111_0000,

        0b0010_0000,
        0b0110_0000,
        0b0010_0000,
        0b0010_0000,
        0b0111_0000,

        0b1111_0000,
        0b0001_0000,
        0b1111_0000,
        0b1000_0000,
        0b1111_0000,

        0b1111_0000,
        0b0001_0000,
        0b1111_0000,
        0b0001_0000,
        0b1111_0000,

        0b1001_0000,
        0b1001_0000,
        0b1111_0000,
        0b0001_0000,
        0b0001_0000,

        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
        0b0001_0000,
        0b1111_0000,

        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
        0b1001_0000,
        0b1111_0000,

        0b1111_0000,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b0100_0000,

        0b1111_0000,
        0b1001_0000,
        0b1111_0000,
        0b1001_0000,
        0b1111_0000,

        0b1111_0000,
        0b1001_0000,
        0b1111_0000,
        0b0001_0000,
        0b1111_0000,

        0b1111_0000,
        0b1001_0000,
        0b1111_0000,
        0b1001_0000,
        0b1001_0000,

        0b1110_0000,
        0b1001_0000,
        0b1110_0000,
        0b1001_0000,
        0b1110_0000,

        0b1111_0000,
        0b1000_0000,
        0b1000_0000,
        0b1000_0000,
        0b1111_0000,

        0b1110_0000,
        0b1001_0000,
        0b1001_0000,
        0b1001_0000,
        0b1110_0000,

        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
        0b1000_0000,
        0b1111_0000,

        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
        0b1000_0000,
        0b1000_0000,
];