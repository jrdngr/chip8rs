/* tslint:disable */
import * as wasm from './chip8_bg';
import { getRandomSeed } from './js/index';
import { setPixel } from './js/index';
import { clearScreen } from './js/index';
import { isKeyDown } from './js/index';
import { getAnyKey } from './js/index';

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    return [ptr, arg.length];
}

const __wbg_now_d79247c5b9feb298_target = Date.now;

export function __wbg_now_d79247c5b9feb298() {
    return __wbg_now_d79247c5b9feb298_target();
}

export function __wbg_getRandomSeed_a48db48623d5ca77() {
    return getRandomSeed();
}

export function __wbg_setPixel_ea3be18ac54580e6(arg0, arg1) {
    setPixel(arg0, arg1);
}

export function __wbg_clearScreen_f6a6492b5970ed72() {
    clearScreen();
}

export function __wbg_isKeyDown_b88d15fe3f21a9a7(arg0) {
    return isKeyDown(arg0) ? 1 : 0;
}

export function __wbg_getAnyKey_15a57617af813800() {
    return getAnyKey();
}
/**
*/
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
    /**
    * @returns {Cpu}
    */
    static new() {
        return Cpu.__construct(wasm.cpu_new());
    }
    /**
    * @param {Uint8Array} arg0
    * @returns {void}
    */
    load_from_web(arg0) {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const [ptr0, len0] = passArray8ToWasm(arg0);
        try {
            return wasm.cpu_load_from_web(this.ptr, ptr0, len0);
            
        } finally {
            wasm.__wbindgen_free(ptr0, len0 * 1);
            
        }
        
    }
    /**
    * @returns {void}
    */
    start() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_start(this.ptr);
    }
    /**
    * @returns {void}
    */
    step() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_step(this.ptr);
    }
    /**
    * @returns {number}
    */
    program_counter() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_program_counter(this.ptr);
    }
    /**
    * @returns {number}
    */
    stack_pointer() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_stack_pointer(this.ptr);
    }
    /**
    * @returns {number}
    */
    delay_timer() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_delay_timer(this.ptr);
    }
    /**
    * @returns {number}
    */
    sound_timer() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_sound_timer(this.ptr);
    }
    /**
    * @returns {number}
    */
    i_register() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_i_register(this.ptr);
    }
    /**
    * @returns {number}
    */
    data_registers() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_data_registers(this.ptr);
    }
    /**
    * @returns {number}
    */
    stack() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_stack(this.ptr);
    }
    /**
    * @returns {number}
    */
    ram() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.cpu_ram(this.ptr);
    }
}

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

