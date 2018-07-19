/* tslint:disable */
export class Keyboard {
free(): void;
static  new(): Keyboard;

 set_key_down(arg0: number): void;

 set_key_up(arg0: number): void;

 get_key_down(arg0: number): boolean;

 any_keys_down(): boolean;

 last_key_down(): number;

 get_state(): number;

}
export class Cpu {
free(): void;
static  new(): Cpu;

 load_from_web(arg0: Uint8Array): void;

 start(): void;

 step(): void;

 program_counter(): number;

 stack_pointer(): number;

 delay_timer(): number;

 sound_timer(): number;

 i_register(): number;

 data_registers(): number;

 stack(): number;

 ram(): number;

 get_keyboard_state(): number;

 set_key_down(arg0: number): void;

 set_key_up(arg0: number): void;

}
