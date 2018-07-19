/* tslint:disable */
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

 keyboard(): JavaScriptKeyboard;

}
export class JavaScriptKeyboard {
free(): void;
static  new(): JavaScriptKeyboard;

}
