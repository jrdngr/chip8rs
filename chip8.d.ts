/* tslint:disable */
export class Screen {
free(): void;
static  new(): Screen;

 draw_sprite(arg0: number, arg1: number, arg2: number): void;

 clear(): void;

}
export class Cpu {
free(): void;
static  new(): Cpu;

 start(): void;

 last_instruction(): string;

 program_counter(): number;

 stack_pointer(): number;

 delay_timer(): number;

 sound_timer(): number;

 i_register(): number;

 data_registers(): number;

 stack(): number;

 ram(): number;

}
