pub struct Cpu {
    program_counter: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    i_register: u16,
    data_registers: [u8; 16],
    stack: [u16; 16],
    ram: [u8; 4096],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            program_counter: 0,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            i_register: 0,
            data_registers: [0; 16],
            stack: [0; 16],
            ram: [0; 4096],
        }
    }
}