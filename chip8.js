/* tslint:disable */
import * as wasm from './chip8_bg';

export class Cpu {

                static __construct(ptr) {
                    return new Cpu(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }

            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_cpu_free(ptr);
            }
        static new() {
    return Cpu.__construct(wasm.cpu_new());
}
start() {
    return wasm.cpu_start(this.ptr);
}
program_counter() {
    return wasm.cpu_program_counter(this.ptr);
}
stack_pointer() {
    return wasm.cpu_stack_pointer(this.ptr);
}
delay_timer() {
    return wasm.cpu_delay_timer(this.ptr);
}
sound_timer() {
    return wasm.cpu_sound_timer(this.ptr);
}
i_register() {
    return wasm.cpu_i_register(this.ptr);
}
data_registers() {
    return wasm.cpu_data_registers(this.ptr);
}
stack() {
    return wasm.cpu_stack(this.ptr);
}
ram() {
    return wasm.cpu_ram(this.ptr);
}
}

export class Screen {

                static __construct(ptr) {
                    return new Screen(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }

            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_screen_free(ptr);
            }
        static new() {
    return Screen.__construct(wasm.screen_new());
}
draw_sprite(arg0, arg1, arg2) {
    return wasm.screen_draw_sprite(this.ptr, arg0, arg1, arg2);
}
clear() {
    return wasm.screen_clear(this.ptr);
}
}

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer)
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

