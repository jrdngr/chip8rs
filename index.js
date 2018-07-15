import { Cpu } from "./chip8";
import { memory } from "./chip8_bg";

const cpu = Cpu.new();
console.log(cpu.program_counter());
console.log(cpu.stack_pointer());
console.log(cpu.delay_timer());
console.log(cpu.sound_timer());
console.log(to_array(cpu.data_registers()));
console.log(to_array(cpu.stack()));


function to_array(ptr) {
    return new Uint8Array(memory.buffer, ptr, 16);
}