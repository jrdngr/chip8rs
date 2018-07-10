use super::opcode::{ OpCode };

pub struct Cpu {
    program_counter: usize,
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    i_register: usize,
    data_registers: [u8; 16],
    stack: [usize; 16],
    ram: [u8; 4096],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            program_counter: 200,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            i_register: 0,
            data_registers: [0; 16],
            stack: [0; 16],
            ram: [0; 4096],
        }
    }

    pub fn process_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::ExecuteMachineSubroutine(nnn) => { /* Not implemented */ },
            OpCode::ClearScreen => {   
                
            },
            OpCode::ReturnFromSubroutine => {   
                self.program_counter = self.stack[self.stack_pointer];
                self.stack_pointer -= 1;
            },
            OpCode::JumpTo(nnn) => {   
                self.program_counter = nnn;
            },
            OpCode::ExecuteSubroutine(nnn) => {   
                self.stack[self.stack_pointer] = self.program_counter;
                self.program_counter = nnn;
            },
            OpCode::SkipIfEqualValue(x, nn) => {   
                let vx = self.data_registers[x as usize];
                if vx == nn {
                    self.program_counter += 1;
                }
            },
            OpCode::SkipIfNotEqualValue(x, nn) => {   
                let vx = self.data_registers[x as usize];
                if vx != nn {
                    self.program_counter += 1;
                }
            },
            OpCode::SkipIfEqualRegister(x, y) => {   
                let vx = self.data_registers[x as usize];
                let vy = self.data_registers[y as usize];
                if vx == vy {
                    self.program_counter += 1;
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
                self.program_counter = nnn + offset;
            },
            OpCode::SetToRandom(x, nn) => {   

            },
            OpCode::DrawSprite(x, y, n) => {   

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
            OpCode::SetIToSprite(x) => { 

            },
            OpCode::StoreDecimal(x) => {   

            },
            OpCode::StoreRegisters(x) => {   

            },
            OpCode::FillRegisters(x) => {   

            },
        }
    }
}