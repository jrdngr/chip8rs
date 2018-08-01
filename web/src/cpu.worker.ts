import { Cpu } from "../chip8";

// Worker.ts
const ctx: Worker = self as any;

// Respond to message from parent thread
ctx.addEventListener("message", (event) => ctx.postMessage({ foo: "foo" }));

//const cpu = Cpu.new();