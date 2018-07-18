/* tslint:disable */
import * as wasm from './chip8_bg';
import { getRandomSeed } from './js/index';
import { setPixel } from './js/index';
import { clearScreen } from './js/index';

const __wbg_f_log_log_n_target = console.log;

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

export function __wbg_f_log_log_n(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    __wbg_f_log_log_n_target(varg0);
}

let cachegetInt64Memory = null;
function getInt64Memory() {
    if (cachegetInt64Memory === null ||
        cachegetInt64Memory.buffer !== wasm.memory.buffer)
        cachegetInt64Memory = new BigInt64Array(wasm.memory.buffer);
    return cachegetInt64Memory;
}

export function __wbg_f_getRandomSeed_getRandomSeed_n(ret) {
    const val = getRandomSeed();
    getInt64Memory()[ret / 8] = val;
}

export function __wbg_f_setPixel_setPixel_n(arg0, arg1) {
    setPixel(arg0, arg1);
}

export function __wbg_f_clearScreen_clearScreen_n() {
    clearScreen();
}

let cachegetUint64Memory = null;
function getUint64Memory() {
    if (cachegetUint64Memory === null ||
        cachegetUint64Memory.buffer !== wasm.memory.buffer)
        cachegetUint64Memory = new BigUint64Array(wasm.memory.buffer);
    return cachegetUint64Memory;
}

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    return [ptr, arg.length];
}

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
load_from_web(arg0) {
    const [ptr0, len0] = passArray8ToWasm(arg0);
    return wasm.cpu_load_from_web(this.ptr, ptr0, len0);
}
start() {
    return wasm.cpu_start(this.ptr);
}
step() {
    return wasm.cpu_step(this.ptr);
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

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

